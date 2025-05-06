use actix_web::web::Data;
use alloy::providers::ProviderBuilder;
use anyhow::Result;

use coingecko_supply::application::services::DefaultTokenSupplyService;
use coingecko_supply::application::workers::TokenSupplyWorker;
use coingecko_supply::configuration::Config;
use coingecko_supply::domain::models::TokenSupply;
use coingecko_supply::domain::providers::TokenMetadataProvider;
use coingecko_supply::infrastructure::providers::BlockchainProvider;
use coingecko_supply::infrastructure::repositories::InMemoryTokenSupplyRepository;
use coingecko_supply::infrastructure::telemetry::setup_tracing;
use coingecko_supply::interfaces::http::HttpApplication;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    setup_tracing(config.application_name, std::io::stdout);

    let alloy_provider = ProviderBuilder::new().on_http(config.blockchain.rpc_url.parse()?);
    let blockchain_provider = BlockchainProvider::new(alloy_provider);

    let token = blockchain_provider
        .fetch_token_metadata(config.token)
        .await?;

    let token_supply = TokenSupply::default();
    let repo = InMemoryTokenSupplyRepository::new(token_supply);
    let data = Data::new(repo);

    let service = DefaultTokenSupplyService::new(blockchain_provider, data.clone());
    let worker = TokenSupplyWorker::new(
        service,
        token,
        config.excluded_sources,
        config.server.update_interval,
    );

    worker.start().await;

    let app = HttpApplication::build(config.server, data).await?;

    app.run().await?;

    Ok(())
}

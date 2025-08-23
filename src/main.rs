use std::sync::Arc;

use actix_web::web::Data;
use alloy::providers::ProviderBuilder;
use anyhow::Result;
use tracing::info;

use coingecko_supply::application::port::outbound::TokenMetadataProvider;
use coingecko_supply::application::use_case::TokenSupplyUseCase;
use coingecko_supply::domain::model::TokenSupply;
use coingecko_supply::infrastructure::adapter::inbound::http::HttpApplication;
use coingecko_supply::infrastructure::adapter::outbound::blockchain::BlockchainTokenMetadataProvider;
use coingecko_supply::infrastructure::adapter::outbound::blockchain::BlockchainTokenSupplyProvider;
use coingecko_supply::infrastructure::adapter::outbound::persistence::InMemoryTokenSupplyRepository;
use coingecko_supply::infrastructure::configuration::Config;
use coingecko_supply::infrastructure::telemetry::setup_tracing;
use coingecko_supply::infrastructure::worker::TokenSupplyWorker;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    setup_tracing(config.application_name, std::io::stdout);

    let alloy_provider = ProviderBuilder::new().on_http(config.blockchain.rpc_url.parse()?);
    let alloy_provider = Arc::new(alloy_provider);

    let supply_provider = BlockchainTokenSupplyProvider::new(alloy_provider.clone());
    let metadata_provider = BlockchainTokenMetadataProvider::new(alloy_provider.clone());

    let token = metadata_provider.fetch_token_metadata(config.token).await?;

    let token_supply = TokenSupply::default();
    let repo = InMemoryTokenSupplyRepository::new(token_supply);

    let service = TokenSupplyUseCase::new(supply_provider, repo);
    let service = Arc::new(service);

    let (worker, handle) = TokenSupplyWorker::new(
        service.clone(),
        token,
        config.excluded_sources,
        config.server.update_interval,
    );

    worker.start(&handle).await;

    let app = HttpApplication::build(config.server, Data::from(service.clone())).await?;

    tokio::select! {
        result = app.run() => result?,
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully");
            handle.shutdown();
        }
    }

    Ok(())
}

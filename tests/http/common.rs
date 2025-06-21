use std::sync::{Arc, LazyLock};

use actix_web::web::Data;
use alloy::primitives::address;
use anyhow::Result;

use coingecko_supply::application::port::outbound::TokenSupplyProvider;
use coingecko_supply::application::service::DefaultTokenSupplyService;
use coingecko_supply::domain::model::{Source, Token, TokenSupply};
use coingecko_supply::infrastructure::adapter::inbound::http::HttpApplication;
use coingecko_supply::infrastructure::adapter::outbound::persistance::InMemoryTokenSupplyRepository;
use coingecko_supply::infrastructure::configuration::Config;
use coingecko_supply::infrastructure::telemetry::setup_tracing;
use coingecko_supply::infrastructure::worker::TokenSupplyWorker;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    setup_tracing("test_app", std::io::sink);
});

#[derive(Clone)]
struct MockSupply;

impl TokenSupplyProvider for MockSupply {
    async fn fetch_token_supply(&self, _token: &Token, _sources: &[Source]) -> Result<TokenSupply> {
        Ok(TokenSupply::new("4242.00", "4200.00"))
    }
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let config = {
        let mut c = Config::load().expect("Failed to retrieve config");
        c.server.port = 0;

        c
    };

    let name = "Supply";
    let symbol = "SUPPLY";
    let address = address!("0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118");
    let decimals = 18;

    let token = Token::new(name, symbol, address, decimals);

    let blockchain_provider = MockSupply;
    let token_supply = TokenSupply::new("4242.00", "4200.00");

    let repo = InMemoryTokenSupplyRepository::new(token_supply);

    let service = DefaultTokenSupplyService::new(blockchain_provider, repo);
    let service = Arc::new(service);

    let (worker, handle) = TokenSupplyWorker::new(
        service.clone(),
        token,
        config.excluded_sources,
        config.server.update_interval,
    );

    let app = HttpApplication::build(config.server, Data::from(service.clone()))
        .await
        .expect("Failed to build the application");

    let port = app.port();

    worker.start(&handle).await;
    let _ = tokio::spawn(app.run());

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
        api_client,
    }
}

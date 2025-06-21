use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio::time::interval;
use tracing::{error, info};

use crate::application::port::inbound::TokenSupplyService;
use crate::domain::model::{Source, Token};

pub struct WorkerHandle {
    shutdown_tx: broadcast::Sender<()>,
}
impl WorkerHandle {
    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
        info!("Shutdown signal sent to worker");
    }
}

/// Worker that periodically updates token supply information.
///
/// This worker runs in the background and updates the latest token supply data
/// at regular intervals based on the configured `update_interval`.
pub struct TokenSupplyWorker<S> {
    token_supply_service: Arc<S>,
    token: Token,
    excluded_sources: Vec<Source>,
    update_interval: u64,
}

impl<S> TokenSupplyWorker<S>
where
    S: TokenSupplyService + Send + Sync + 'static,
{
    /// Creates a new [`TokenSupplyWorker`] instance.
    ///
    /// # Arguments
    ///
    /// * `token_supply_service` - The service used to fetch and update token supply data.
    /// * `token` - The token for which supply information is retrieved.
    /// * `excluded_sources` - A list of sources to exclude from the circulating supply.
    /// * `update_interval` - Time in seconds between update operations.
    ///
    /// # Returns
    ///
    /// A new [`TokenSupplyWorker`] instance configured with the provided parameters.
    pub fn new(
        token_supply_service: Arc<S>,
        token: Token,
        excluded_sources: Vec<Source>,
        update_interval: u64,
    ) -> (Self, WorkerHandle) {
        let (shutdown_tx, _) = broadcast::channel(1);

        let worker = Self {
            token_supply_service,
            token,
            excluded_sources,
            update_interval,
        };

        let handle = WorkerHandle { shutdown_tx };

        (worker, handle)
    }

    /// Starts the background worker task.
    ///
    /// This method initiates a background task that will periodically:
    /// 1. Fetch the latest supply information.
    /// 2. Update the stored token supply data.
    ///
    /// The task will continue running indefinitely until the application shuts
    /// down. Any errors during fetch or update operations are logged but won't
    /// stop the worker.
    ///
    /// # Note
    ///
    /// This method consumes the worker instance as it transfers ownership of
    /// the service to the background task.
    pub async fn start(self, handle: &WorkerHandle) {
        let service = self.token_supply_service;
        let token = self.token;
        let sources = self.excluded_sources;
        let freq = self.update_interval;
        let mut shutdown_rx = handle.shutdown_tx.subscribe();

        info!("Starting supply worker on interval: {} seconds", freq);

        let mut update_interval = interval(Duration::from_secs(freq));
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = update_interval.tick() => {
                        info!("Fetching updated token supply data");

                        match service.fetch_token_supply(&token, &sources).await {
                            Ok(supply) => match service.update_token_supply(supply).await {
                                Ok(_) => info!("Successfully updated token supply data"),
                                Err(e) => error!("Failed to update token supply: {}", e),
                            },
                            Err(e) => error!("Failed to fetch circulating supply: {}", e),
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Received shutdown signal, stopping worker");
                        break;
                    }
                }
            }
            info!("Token supply worker stopped");
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use alloy::primitives::address;
    use anyhow::{Result, anyhow};
    use tokio::time::sleep;

    use crate::domain::model::{Source, TokenSupply};

    use super::*;

    // -------------------------------------------------------------------------
    // Mock Implementations for Testing

    // Mock implementation of TokenSupplyService for testing.
    struct MockTokenSupplyService {
        fetch_count: Arc<Mutex<u32>>,
        update_count: Arc<Mutex<u32>>,
        fetch_should_fail: bool,
        update_should_fail: bool,
    }

    impl MockTokenSupplyService {
        fn new(fetch_should_fail: bool, update_should_fail: bool) -> Self {
            Self {
                fetch_count: Arc::new(Mutex::new(0)),
                update_count: Arc::new(Mutex::new(0)),
                fetch_should_fail,
                update_should_fail,
            }
        }
    }

    impl TokenSupplyService for MockTokenSupplyService {
        async fn fetch_token_supply(
            &self,
            _token: &Token,
            _sources: &[Source],
        ) -> Result<TokenSupply> {
            let mut count = self.fetch_count.lock().unwrap();
            *count += 1;

            match self.fetch_should_fail {
                true => Err(anyhow!("Simulated fetch failure")),
                false => Ok(TokenSupply::new("2000", "1000.00")),
            }
        }

        async fn update_token_supply(&self, _supply: TokenSupply) -> Result<()> {
            let mut count = self.update_count.lock().unwrap();
            *count += 1;

            match self.update_should_fail {
                true => Err(anyhow!("Simulated fetch failure")),
                false => Ok(()),
            }
        }

        async fn get_token_supply(&self) -> Result<TokenSupply> {
            Ok(TokenSupply::new("2000", "1000.00"))
        }
    }

    // -------------------------------------------------------------------------
    // Test Helper Functions

    fn create_token() -> Token {
        let name = "Supply";
        let symbol = "SUPPLY";
        let address = address!("0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118");
        let decimals = 18;

        Token::new(name, symbol, address, decimals)
    }

    // -------------------------------------------------------------------------
    // Tests

    #[tokio::test]
    async fn worker_fetches_and_updates_on_interval() {
        // Arrange.
        let service = MockTokenSupplyService::new(false, false);
        let service = Arc::new(service);
        let fetch_count = service.fetch_count.clone();
        let update_count = service.update_count.clone();
        let token = create_token();
        let sources: Vec<Source> = Vec::new();

        // Act.
        let (worker, handle) = TokenSupplyWorker::new(service, token, sources, 1);
        worker.start(&handle).await;

        sleep(Duration::from_secs(3)).await;

        // Assert.
        assert!(
            *fetch_count.lock().unwrap() >= 2,
            "Should have fetched at least twice"
        );

        assert!(
            *update_count.lock().unwrap() >= 2,
            "Should have updated at least twice"
        );
    }

    #[tokio::test]
    async fn worker_continues_after_fetch_error() {
        // Arrange.
        let service = MockTokenSupplyService::new(true, false);
        let service = Arc::new(service);
        let fetch_count = service.fetch_count.clone();
        let token = create_token();
        let sources: Vec<Source> = Vec::new();

        // Act.
        let (worker, handle) = TokenSupplyWorker::new(service, token, sources, 1);
        worker.start(&handle).await;

        sleep(Duration::from_secs(3)).await;

        // Assert.
        assert!(
            *fetch_count.lock().unwrap() >= 2,
            "Should have attempted to fetch at least twice despite errors"
        );
    }

    #[tokio::test]
    async fn worker_continues_after_update_error() {
        // Arrange.
        let service = MockTokenSupplyService::new(false, true);
        let service = Arc::new(service);
        let fetch_count = service.fetch_count.clone();
        let update_count = service.update_count.clone();
        let token = create_token();
        let sources: Vec<Source> = Vec::new();

        // Act.
        let (worker, handle) = TokenSupplyWorker::new(service, token, sources, 1);
        worker.start(&handle).await;

        sleep(Duration::from_secs(3)).await;

        // Assert.
        assert!(
            *fetch_count.lock().unwrap() >= 2,
            "Should have fetched at least twice"
        );

        assert!(
            *update_count.lock().unwrap() >= 2,
            "Should have attempted to update at least twice despite errors"
        );
    }

    #[tokio::test]
    async fn worker_handles_both_fetch_and_update_errors() {
        // Arrange.
        let service = MockTokenSupplyService::new(true, true);
        let service = Arc::new(service);
        let fetch_count = service.fetch_count.clone();
        let update_count = service.update_count.clone();
        let token = create_token();
        let sources: Vec<Source> = Vec::new();

        // Act.
        let (worker, handle) = TokenSupplyWorker::new(service, token, sources, 1);
        worker.start(&handle).await;

        sleep(Duration::from_secs(3)).await;

        // Assert.
        assert!(
            *fetch_count.lock().unwrap() >= 2,
            "Should have attempted to fetch at least twice despite errors"
        );

        // The update count should be 0 because fetch errors prevent updates from happening
        assert_eq!(
            *update_count.lock().unwrap(),
            0,
            "Should not have attempted any updates due to fetch errors"
        );
    }
}

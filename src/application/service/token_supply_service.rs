use anyhow::Result;

use crate::application::port::inbound::TokenSupplyService;
use crate::application::port::outbound::TokenSupplyProvider;
use crate::domain::model::{Source, Token, TokenSupply};
use crate::domain::repository::TokenSupplyRepository;

/// Default implementation of the [`TokenSupplyService`].
///
/// This service coordinates between a token supply provider and a token supply
/// repository to manage token supply information.
pub struct DefaultTokenSupplyService<S, R> {
    provider: S,
    repository: R,
}

impl<S, R> DefaultTokenSupplyService<S, R>
where
    S: TokenSupplyProvider,
    R: TokenSupplyRepository,
{
    /// Creates a new [`DefaultTokenSupplyService`].
    ///
    /// # Arguments
    ///
    /// * `provider` - Component that fetches token supply data.
    /// * `repository` - Storage for token supply information.
    ///
    /// # Returns
    ///
    /// A new instance of [`DefaultTokenSupplyService`].
    pub fn new(provider: S, repository: R) -> Self {
        Self {
            provider,
            repository,
        }
    }
}

impl<S, R> TokenSupplyService for DefaultTokenSupplyService<S, R>
where
    S: TokenSupplyProvider + Send + Sync,
    R: TokenSupplyRepository + Send + Sync,
{
    async fn fetch_token_supply(&self, token: &Token, sources: &[Source]) -> Result<TokenSupply> {
        self.provider.fetch_token_supply(token, sources).await
    }

    async fn update_token_supply(&self, supply: TokenSupply) -> Result<()> {
        self.repository.store(supply).await
    }

    async fn get_token_supply(&self) -> Result<TokenSupply> {
        self.repository.get_current().await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use alloy::primitives::address;
    use anyhow::anyhow;

    use crate::application::port::outbound::TokenSupplyProvider;

    use super::*;

    // -------------------------------------------------------------------------
    // Mock Implementations for Testing

    struct MockProvider {
        should_fail: bool,
        total_supply: String,
        circulating_supply: String,
    }

    impl MockProvider {
        fn new<T: Into<String>>(should_fail: bool, total: T, circulating: T) -> Self {
            Self {
                should_fail,
                total_supply: total.into(),
                circulating_supply: circulating.into(),
            }
        }

        fn token_supply(&self) -> TokenSupply {
            TokenSupply::new(self.total_supply.clone(), self.circulating_supply.clone())
        }
    }

    impl TokenSupplyProvider for MockProvider {
        async fn fetch_token_supply(&self, _t: &Token, _s: &[Source]) -> Result<TokenSupply> {
            match self.should_fail {
                true => Err(anyhow!("Simulated fetch failure")),
                false => Ok(self.token_supply()),
            }
        }
    }

    struct MockRepository {
        stored_supply: Arc<Mutex<Option<TokenSupply>>>,
        should_fail: bool,
    }

    impl MockRepository {
        fn new(should_fail: bool) -> Self {
            Self {
                stored_supply: Arc::new(Mutex::new(None)),
                should_fail,
            }
        }
    }

    impl TokenSupplyRepository for MockRepository {
        async fn store(&self, supply: TokenSupply) -> Result<()> {
            match self.should_fail {
                true => Err(anyhow!("Simulated store failure")),
                false => {
                    let mut stored = self.stored_supply.lock().unwrap();
                    *stored = Some(supply);
                    Ok(())
                }
            }
        }

        async fn get_current(&self) -> Result<TokenSupply> {
            match self.should_fail {
                true => Err(anyhow!("Simulated get failure")),
                false => match &*self.stored_supply.lock().unwrap() {
                    Some(supply) => Ok(supply.clone()),
                    None => Ok(TokenSupply::default()),
                },
            }
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

    fn create_test_service(
        provider_should_fail: bool,
        repo_should_fail: bool,
        total_supply: &str,
        circulating_supply: &str,
    ) -> DefaultTokenSupplyService<MockProvider, MockRepository> {
        let provider = MockProvider::new(provider_should_fail, total_supply, circulating_supply);
        let repo = MockRepository::new(repo_should_fail);
        let service = DefaultTokenSupplyService::new(provider, repo);

        service
    }

    fn assert_supply_values(supply: &TokenSupply, total: &str, circulating: &str) {
        assert_eq!(supply.total_supply, total);
        assert_eq!(supply.circulating_supply, circulating);
    }

    // -------------------------------------------------------------------------
    // Tests

    #[tokio::test]
    async fn test_fetch_token_supply_should_delegate_to_provider() {
        // Arrange.
        let total = "1000.00";
        let circulating = "500.00";
        let service = create_test_service(false, false, total, circulating);
        let token = create_token();
        let sources = Vec::new();

        // Act.
        let result = service.fetch_token_supply(&token, &sources).await;

        // Assert.
        assert!(result.is_ok(), "Expected successful supply fetch");
        let supply = result.unwrap();
        assert_supply_values(&supply, total, circulating);
    }

    #[tokio::test]
    async fn test_fetch_token_supply_should_propagate_provider_errors() {
        // Arrange.
        let service = create_test_service(true, false, "1000.00", "500.00");
        let token = create_token();
        let sources = Vec::new();

        // Act.
        let result = service.fetch_token_supply(&token, &sources).await;

        // Assert.
        assert!(result.is_err(), "Expected error when provider fails");
    }

    #[tokio::test]
    async fn test_update_token_supply_should_store_in_repository() {
        // Arrange.
        let service = create_test_service(false, false, "0.00", "0.00");
        let total = "2000.00";
        let circulating = "1000.00";
        let new_supply = TokenSupply::new(total, circulating);

        // Act.
        let store_result = service.update_token_supply(new_supply).await;

        // Assert.
        assert!(store_result.is_ok(), "Expected successful supply update");

        let get_result = service.get_token_supply().await;
        assert!(get_result.is_ok(), "Expected successful supply retrieval");

        let stored_supply = get_result.unwrap();
        assert_supply_values(&stored_supply, total, circulating);
    }

    #[tokio::test]
    async fn test_update_token_supply_should_propagate_repository_errors() {
        // Arrange.
        let service = create_test_service(false, true, "1000.00", "500.00");
        let new_supply = TokenSupply::new("2000.00", "1000.00");

        // Act
        let result = service.update_token_supply(new_supply).await;

        // Assert
        assert!(result.is_err(), "Expected error when repository fails");
    }

    #[tokio::test]
    async fn test_get_token_supply_should_propagate_repository_errors() {
        // Arrange.
        let service = create_test_service(false, true, "1000.00", "500.00");

        // Act.
        let result = service.get_token_supply().await;

        // Assert.
        assert!(result.is_err(), "Expected error when repository fails");
    }
}

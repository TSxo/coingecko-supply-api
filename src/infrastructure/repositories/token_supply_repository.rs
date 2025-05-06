use anyhow::Result;
use tokio::sync::RwLock;

use crate::domain::models::TokenSupply;
use crate::domain::repositories::TokenSupplyRepository;

/// In-memory repository for token supply data.
///
/// This repository stores token supply information in memory using a `RwLock`
/// for safe concurrent access.
pub struct InMemoryTokenSupplyRepository {
    token_supply: RwLock<TokenSupply>,
}

impl InMemoryTokenSupplyRepository {
    /// Creates a new in-memory token supply repository.
    ///
    /// # Arguments
    ///
    /// * `token_supply` - Initial token supply data.
    ///
    /// # Returns
    ///
    /// * A new InMemoryTokenSupplyRepository instance.
    pub fn new(token_supply: TokenSupply) -> Self {
        let token_supply = RwLock::new(token_supply);
        Self { token_supply }
    }
}

impl TokenSupplyRepository for InMemoryTokenSupplyRepository {
    async fn store(&self, supply: TokenSupply) -> Result<()> {
        let mut current = self.token_supply.write().await;
        *current = supply;
        Ok(())
    }

    async fn get_current(&self) -> Result<TokenSupply> {
        let current = self.token_supply.read().await;
        Ok(current.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_store_updates_token_supply() {
        // Arrange.
        let initial_supply = TokenSupply::new("2000.00", "1000.00");
        let new_supply = TokenSupply::new("3000.00", "2000.00");

        let repo = InMemoryTokenSupplyRepository::new(initial_supply);

        // Act.
        let store_result = repo.store(new_supply.clone()).await;
        let current = repo.get_current().await.unwrap();

        // Assert.
        assert!(store_result.is_ok());
        assert_eq!(current.total_supply, new_supply.total_supply);
        assert_eq!(current.circulating_supply, new_supply.circulating_supply);
    }

    #[tokio::test]
    async fn test_get_current_returns_current_supply() {
        // Arrange.
        let expected = TokenSupply::new("3000.00", "2000.00");
        let repo = InMemoryTokenSupplyRepository::new(expected.clone());

        // Act.
        let result = repo.get_current().await.unwrap();

        // Assert.
        assert_eq!(result.total_supply, expected.total_supply);
        assert_eq!(result.circulating_supply, expected.circulating_supply);
    }
}

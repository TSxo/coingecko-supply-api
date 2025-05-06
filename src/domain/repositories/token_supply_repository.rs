use anyhow::Result;

use crate::domain::models::TokenSupply;

/// Repository trait for managing token supply data.
///
/// This trait defines the core operations needed to persist and retrieve token
/// supply information.
pub trait TokenSupplyRepository {
    /// Stores token supply information in the repository.
    ///
    /// # Arguments
    ///
    /// * `supply` - The token supply data to store.
    ///
    /// # Returns
    ///
    /// A future resolving to a success indicator, or an error if the operation
    /// failed.
    ///
    /// # Errors
    ///
    /// Returns an error if the data could not be stored in the repository.
    fn store(&self, supply: TokenSupply) -> impl Future<Output = Result<()>> + Send;

    /// Retrieves the current token supply information.
    ///
    /// # Returns
    ///
    /// A future resolving to the current [`TokenSupply`], or an error if the
    /// data could not be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the data could not be retrieved from the repository.
    fn get_current(&self) -> impl Future<Output = Result<TokenSupply>> + Send;
}

use anyhow::Result;

use crate::domain::models::{Source, Token, TokenSupply};

/// Defines the core service operations for token supply management.
///
/// This service coordinates between data providers and storage, providing a
/// complete interface for token supply operations in the application.
pub trait TokenSupplyService {
    /// Fetches the token supply information.
    ///
    /// # Arguments
    ///
    /// * `token` - The token for which supply information is retrieved.
    /// * `excluded_sources` - A list of sources to exclude from the circulating supply.
    ///
    /// # Returns
    ///
    /// A future resolving to [`TokenSupply`], or an error if the data could not
    /// be fetched.
    ///
    /// # Errors
    ///
    /// Returns an error if the supply data could not be retrieved from the sources.
    fn fetch_token_supply(
        &self,
        token: &Token,
        excluded_sources: &[Source],
    ) -> impl Future<Output = Result<TokenSupply>> + Send;

    /// Updates the stored token supply information.
    ///
    /// # Arguments
    ///
    /// * `supply` - The new token supply data to store.
    ///
    /// # Returns
    ///
    /// A future resolving to a success indicator, or an error if the update
    /// failed.
    ///
    /// # Errors
    ///
    /// Returns an error if the supply data could not be updated in storage.
    fn update_token_supply(&self, supply: TokenSupply) -> impl Future<Output = Result<()>> + Send;

    /// Retrieves the current token supply data from storage.
    ///
    /// # Returns
    ///
    /// A future resolving to the stored [`TokenSupply`], or an error if the
    /// data could not be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the supply data could not be retrieved from storage.
    fn get_token_supply(&self) -> impl Future<Output = Result<TokenSupply>> + Send;
}

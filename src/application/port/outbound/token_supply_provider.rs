use anyhow::Result;

use crate::domain::model::{Source, Token, TokenSupply};

/// Defines a capability to fetch token supply data from an external source.
///
/// This trait is implemented by components that can retrieve token supply
/// information, such as blockchain clients, APIs, or other data providers.
pub trait TokenSupplyProvider {
    /// Fetches the current token supply information from an external source.
    ///
    /// # Arguments
    ///
    /// * `token` - The token for which supply information is retrieved.
    /// * `excluded_sources` - A list of sources to exclude from the circulating supply.
    ///
    /// # Returns
    ///
    /// A future resolving to a [`TokenSupply`] instance, or an error if the data
    /// could not be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the data could not be retrieved.
    fn fetch_token_supply(
        &self,
        token: &Token,
        excluded_sources: &[Source],
    ) -> impl Future<Output = Result<TokenSupply>> + Send;
}

use alloy::primitives::Address;
use anyhow::Result;

use crate::domain::models::Token;

/// Defines a capability to fetch token metadata from an external source.
///
/// This trait is implemented by components that can retrieve token metadata,
/// such as blockchain clients, APIs, or other data providers.
pub trait TokenMetadataProvider {
    /// Fetches token metadata from an external source.
    ///
    /// # Arguments
    ///
    /// * `token_address` - The address of the token for which metadata is retrieved.
    ///
    /// # Returns
    ///
    /// A future resolving to a [`Token`] instance, or an error if the data
    /// could not be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the data could not be retrieved.
    fn fetch_token_metadata(
        &self,
        token_address: Address,
    ) -> impl Future<Output = Result<Token>> + Send;
}

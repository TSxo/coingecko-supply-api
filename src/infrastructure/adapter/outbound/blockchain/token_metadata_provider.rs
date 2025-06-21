use std::sync::Arc;

use alloy::primitives::Address;
use alloy::providers::Provider;
use anyhow::Result;
use tracing::info;

use crate::application::port::outbound::TokenMetadataProvider;
use crate::domain::model::Token;
use crate::infrastructure::adapter::outbound::blockchain::contracts::IERC20;

/// Provider for retreiving token metadata from the blockchain.
pub struct BlockchainTokenMetadataProvider<P: Provider> {
    provider: Arc<P>,
}

impl<P: Provider> BlockchainTokenMetadataProvider<P> {
    /// Creates a new [`BlockchainTokenMetadataProvider`] instance.
    ///
    /// # Arguments
    ///
    /// * `provider` - The Alloy [`Provider`] to use for connections.
    ///
    /// # Returns
    ///
    /// * A new [`TokenMetadataProvider`] instance.
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }
}

impl<P: Provider> TokenMetadataProvider for BlockchainTokenMetadataProvider<P> {
    async fn fetch_token_metadata(&self, token_address: Address) -> Result<Token> {
        info!("Fetching token metadata for address: {}", token_address);

        let c = IERC20::new(token_address, &self.provider);

        let n = c.name();
        let s = c.symbol();
        let d = c.decimals();

        let (name_result, symbol_result, dec_result) = tokio::join!(n.call(), s.call(), d.call());

        let name = name_result?._0;
        let symbol = symbol_result?._0;
        let decimals = dec_result?._0;

        let token = Token::new(name, symbol, token_address, decimals);

        info!("Token matadata retrieved: {}", token);

        Ok(token)
    }
}

use std::sync::Arc;

use alloy::primitives::utils;
use alloy::providers::Provider;
use anyhow::Result;
use tracing::info;

use crate::application::port::outbound::TokenSupplyProvider;
use crate::domain::model::{Source, Token, TokenSupply};
use crate::infrastructure::adapter::outbound::blockchain::contracts::IERC20;

/// Provider for retreiving token supply data from the blockchain.
pub struct BlockchainTokenSupplyProvider<P: Provider> {
    provider: Arc<P>,
}

impl<P: Provider> BlockchainTokenSupplyProvider<P> {
    /// Creates a new [`BlockchainTokenSupplyProvider`] instance.
    ///
    /// # Arguments
    ///
    /// * `provider` - The Alloy [`Provider`] to use for connections.
    ///
    /// # Returns
    ///
    /// * A new [`BlockchainTokenSupplyProvider`] instance.
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }
}

impl<P: Provider> TokenSupplyProvider for BlockchainTokenSupplyProvider<P> {
    async fn fetch_token_supply(
        &self,
        token: &Token,
        excluded_sources: &[Source],
    ) -> Result<TokenSupply> {
        info!("Fetching token supply for: {}", token);

        let c = IERC20::new(token.address, &self.provider);

        let total_supply = c.totalSupply().call().await?._0;
        let mut circulating_supply = total_supply;

        for i in excluded_sources.iter() {
            info!("Beginning check for {} at {}", i.name, i.address);

            let bal = c.balanceOf(i.address).call().await?.balance;

            info!("{}: {}", i.name, bal);

            circulating_supply -= bal;

            info!("Finished check for {} at {}", i.name, i.address);
        }

        let total_supply = utils::format_units(total_supply, token.decimals)?;
        let circulating_supply = utils::format_units(circulating_supply, token.decimals)?;

        let token_supply = TokenSupply::new(total_supply, circulating_supply);

        info!("Token Supply: {}", token_supply);

        Ok(token_supply)
    }
}

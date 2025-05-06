use alloy::primitives::{Address, utils};
use alloy::providers::Provider;
use anyhow::Result;
use tracing::info;

use crate::domain::models::{Source, Token, TokenSupply};
use crate::domain::providers::{TokenMetadataProvider, TokenSupplyProvider};
use crate::infrastructure::contracts::IERC20;

/// Provider for interacting with the blockchain.
pub struct BlockchainProvider<P: Provider> {
    provider: P,
}

impl<P: Provider> BlockchainProvider<P> {
    /// Creates a new [`BlockchainProvider`] instance.
    ///
    /// # Arguments
    ///
    /// * `provider` - The Alloy [`Provider`] to use for connections.
    ///
    /// # Returns
    ///
    /// * A new [`BlockchainProvider`] instance.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

impl<P: Provider> TokenMetadataProvider for BlockchainProvider<P> {
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

impl<P: Provider> TokenSupplyProvider for BlockchainProvider<P> {
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

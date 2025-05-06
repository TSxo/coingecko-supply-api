use std::fmt::Display;

/// Represents token supply information.
#[derive(Debug, Clone)]
pub struct TokenSupply {
    /// Formatted string representation of the token's total supply.
    pub total_supply: String,

    /// Formatted string representation of the token's circulating supply.
    pub circulating_supply: String,
}

impl TokenSupply {
    /// Creates a new [`TokenSupply`] instance.
    ///
    /// # Arguments
    ///
    /// * `total_supply` - Formatted string representation of the total supply.
    /// * `circulating_supply` - Formatted string representation of the circulating supply.
    ///
    /// # Returns
    ///
    /// * A new [`TokenSupply`] instance.
    pub fn new<T: Into<String>>(total_supply: T, circulating_supply: T) -> Self {
        TokenSupply {
            total_supply: total_supply.into(),
            circulating_supply: circulating_supply.into(),
        }
    }
}

impl Default for TokenSupply {
    /// Creates a new [`TokenSupply`] instance with a default total and
    /// circulating supply of zero.
    ///
    /// # Returns
    ///
    /// * A new [`TokenSupply`] instance.
    fn default() -> Self {
        TokenSupply::new("0.00", "0.00")
    }
}

impl Display for TokenSupply {
    /// Returns a human-readable representation of [`TokenSupply`].
    ///
    /// # Examples
    ///
    /// ```
    /// use coingecko_supply::domain::models::TokenSupply;
    ///
    /// let token_supply = TokenSupply::default();
    /// assert_eq!(format!("{}", token_supply), "Total: 0.00 Circulating: 0.00");
    ///
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total: {} Circulating: {}",
            self.total_supply, self.circulating_supply
        )
    }
}

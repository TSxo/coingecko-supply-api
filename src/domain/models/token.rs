use std::fmt::Display;

use alloy::primitives::Address;

/// Represents an ERC20 token.
#[derive(Debug, Clone)]
pub struct Token {
    /// The name of the token.
    pub name: String,

    /// The symbol of the token.
    pub symbol: String,

    /// The address of the token.
    pub address: Address,

    /// The decimal places of the token.
    pub decimals: u8,
}

impl Token {
    /// Creates a new [`Token`] instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the token.
    /// * `symbol` - The symbol of the token.
    /// * `address` - The address of the token.
    /// * `decimals` - The decimal places of the token.
    ///
    /// # Returns
    ///
    /// * A new [`Token`] instance.
    pub fn new<T: Into<String>>(name: T, symbol: T, address: Address, decimals: u8) -> Self {
        Token {
            name: name.into(),
            symbol: symbol.into(),
            address,
            decimals,
        }
    }
}

impl Display for Token {
    /// Returns a human-readable representation of [`Token`].
    ///
    /// # Examples
    ///
    /// ```
    /// use alloy::primitives::address;
    /// use coingecko_supply::domain::models::Token;
    ///
    /// let name = "Supply";
    /// let symbol = "SUPPLY";
    /// let address = address!("0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118");
    /// let decimals = 18;
    ///
    /// let expected = "Supply (SUPPLY) - 18 decimals: 0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118";
    ///
    /// let token = Token::new(name, symbol, address, decimals);
    /// assert_eq!(format!("{}", token), expected);
    ///
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) - {} decimals: {}",
            self.name, self.symbol, self.decimals, self.address
        )
    }
}

use alloy::primitives::Address;
use serde::Deserialize;

/// Represents a named source with an associated blockchain address.
#[derive(Deserialize, Debug)]
pub struct Source {
    /// The name of the source.
    pub name: String,

    /// The blockchain address associated with the source.
    pub address: Address,
}

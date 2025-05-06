//! Domain Providers
//!
//! This module defines traits for retrieving data from external sources. These
//! traits are implemented by infrastructure components to supply domain data
//! such as token information.
//!
//! Modules:
//! * `token_metadata_provider`: Trait for fetching token metadata.
//! * `token_supply_provider`: Trait for fetching token supply data.

pub mod token_metadata_provider;
pub mod token_supply_provider;

pub use token_metadata_provider::TokenMetadataProvider;
pub use token_supply_provider::TokenSupplyProvider;

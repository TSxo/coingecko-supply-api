//! External Data Providers
//!
//! This module contains concrete implementations of provider traits defined in
//! the [`crate::domain`] layer. These implementations interact with external
//! systems such as blockchain nodes to fulfill the application's data
//! requirements.
//!
//! Providers:
//! * `blockchain`: Implementation for blockchain interaction and data retrieval.

pub mod blockchain;

pub use blockchain::BlockchainProvider;

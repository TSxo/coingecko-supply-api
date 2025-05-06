//! Domain Repository Implementations
//!
//! This module contains concrete implementations of domain repository traits.
//! These implementations fulfill the contracts defined in the [`crate::domain`]
//! layer.
//!
//! Modules:
//! - `token_supply_repository`: In-memory token supply repository.

pub mod token_supply_repository;

pub use token_supply_repository::InMemoryTokenSupplyRepository;

//! Domain Repositories
//!
//! This module defines traits for storing and retrieving domain data from
//! repositories. These traits are implemented by infrastructure components.
//!
//! Modules:
//! * `token_supply_repository`: Trait for token supply data storage and retrieval.

pub mod token_supply_repository;

pub use token_supply_repository::TokenSupplyRepository;

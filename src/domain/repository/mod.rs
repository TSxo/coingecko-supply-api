//! Domain Repositories
//!
//! This module defines traits for storing and retrieving domain data from
//! repositories. These traits are implemented by infrastructure components.

pub mod token_supply_repository;

pub use token_supply_repository::TokenSupplyRepository;

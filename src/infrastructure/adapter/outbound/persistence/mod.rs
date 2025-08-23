//! Persistence Outbound Adapters
//!
//! Persistence adapters implement repository contracts defined in the domain
//! layer. They handle data storage and retrieval while abstracting the specific
//! persistence technology from the domain.

pub mod in_memory_token_supply_repository;

pub use in_memory_token_supply_repository::InMemoryTokenSupplyRepository;

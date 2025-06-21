//! Token Supply Service
//!
//! This library provides functionality for tracking and reporting token supply
//! metrics in a format compliant with CoinGecko's requirements. It allows
//! developers to maintain up-to-date circulating and total supply information
//! for their tokens.
//!
//! This service can be deployed as a standalone API that blockchain projects can
//! integrate with CoinGecko and other token listing services.

pub mod application;
pub mod domain;
pub mod infrastructure;

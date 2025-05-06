//! HTTP Route Handlers
//!
//! This module contains HTTP route handlers for the token supply API.
//!
//! Modules:
//! * `circulating_supply`: Endpoint for retrieving token circulating supply information.
//! * `health`: Health check endpoint for monitoring service status.
//! * `total_supply`: Endpoint for retrieving token total supply information.

pub mod circulating_supply;
pub mod health;
pub mod total_supply;

pub use circulating_supply::circulating_supply;
pub use health::health;
pub use total_supply::total_supply;

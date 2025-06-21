//! Infrastructure Layer
//!
//! The infrastructure layer contains concrete implementations of external
//! concerns such as databases, web servers, external APIs, and configuration.
//! This layer implements the contracts defined by the application and domain layers.

pub mod adapter;
pub mod configuration;
pub mod telemetry;
pub mod worker;

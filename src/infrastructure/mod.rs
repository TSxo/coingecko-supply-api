//! Infrastructure Layer
//!
//! This module provides concrete implementations of traits defined in the
//! [`crate::domain`] layer. It facilitates interaction with external systems.
//!
//! Modules:
//! * `contracts`: Defines smart contract interfaces for interacting with blockchain protocols.
//! * `providers`: Implementations of domain provider interfaces.
//! * `repositories`: Implementations of domain repositories.
//! * `telemetry`: Provides tracing and structured logging.

pub mod contracts;
pub mod providers;
pub mod repositories;
pub mod telemetry;

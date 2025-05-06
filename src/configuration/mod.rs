//! Configuration Layer
//!
//! This module manages the application's configuration settings from various sources.
//!
//! Modules:
//! * `blockchain`: Configuration settings related to blockchain connectivity and interaction.
//! * `environment`: Environment-specific configuration.
//! * `load`: Functions to load and initialize configuration from files and environment.
//! * `server`: HTTP server configuration.

pub mod blockchain;
pub mod environment;
pub mod load;
pub mod server;

pub use load::Config;

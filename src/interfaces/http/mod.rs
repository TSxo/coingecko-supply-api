//! HTTP Interface Layer
//!
//! This module provides HTTP-based interfaces for interacting with the token
//! supply service.
//!
//! The HTTP interface exposes endpoints for retrieving token supply information
//! in a format compliant with CoinGecko's requirements.
//!
//! Modules:
//! * `routes`: API route handlers.
//! * `server`: HTTP server implementation.

pub mod routes;
pub mod server;

pub use server::HttpApplication;

//! HTTP Inbound Adapter
//!
//! HTTP adapter that exposes the application's functionality through a RESTful
//! API. This adapter handles HTTP-specific concerns while delegating business
//! logic to application services.

pub mod dto;
pub mod route;
pub mod server;

pub use server::HttpApplication;

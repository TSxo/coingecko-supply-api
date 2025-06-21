//! HTTP Data Transfer Objects
//!
//! DTOs handle the serialization and deserialization of HTTP request and
//! response payloads. They provide a stable external API contract while
//! allowing internal domain models to evolve independently.
pub mod supply_response;

pub use supply_response::SupplyResponse;

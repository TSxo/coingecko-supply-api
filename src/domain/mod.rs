//! Domain Layer
//!
//! This module contains the core domain components of the application,
//! representing the business concepts and rules independent of external
//! frameworks or technologies.
//!
//! Modules:
//! * `models`: Core domain entities and value objects.
//! * `providers`: Traits for retrieving data from external sources.
//! * `repositories`: Traits for data persistence operations.
//! * `services`: Business logic and domain operations coordination.

pub mod models;
pub mod providers;
pub mod repositories;
pub mod services;

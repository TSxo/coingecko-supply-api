//! Application Layer
//!
//! This module contains application services and workers that coordinate domain
//! operations.
//!
//! Modules:
//! * `services`: Application-specific services that coordinate domain operations and external resources.
//! * `workers`: Background workers that perform periodic tasks and updates.

pub mod services;
pub mod workers;

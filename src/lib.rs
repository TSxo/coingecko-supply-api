//! Token Supply Service
//!
//! This library provides functionality for tracking and reporting token supply
//! metrics in a format compliant with CoinGecko's requirements. It allows
//! developers to maintain up-to-date circulating and total supply information
//! for their tokens.
//!
//! The service is structured following Domain-Driven Design principles:
//!
//! * `application`: Application services and workers that coordinate domain operations.
//! * `configuration`: Configuration management for the service.
//! * `domain`: Core domain models, providers, repositories, and services.
//! * `infrastructure`: External system integrations and technical implementations.
//! * `interfaces`: External interface adapters, including HTTP API endpoints.
//!
//! This service can be deployed as a standalone API that blockchain projects can
//! integrate with CoinGecko and other token listing services.

pub mod application;
pub mod configuration;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

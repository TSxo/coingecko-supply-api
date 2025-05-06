//! Blockchain Contract Interfaces
//!
//! This module provides interface definitions and implementations for interacting
//! with blockchain smart contracts.
//!
//! Contracts:
//! * `erc20`: Interface for the ERC-20 token standard, enabling token operations.

pub mod erc20;

pub use erc20::IERC20;

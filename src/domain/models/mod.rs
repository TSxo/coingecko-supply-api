//! Domain Models
//!
//! This module defines core domain entities and value objects used in token
//! supply calculations, including representations of sources and supply data.
//!
//! Modules:
//! * `source`: Defines named blockchain sources.
//! * `token`: Defines ERC20 token data.
//! * `token_supply`: Defines total and circulating supply data.

pub mod source;
pub mod token;
pub mod token_supply;

pub use source::Source;
pub use token::Token;
pub use token_supply::TokenSupply;

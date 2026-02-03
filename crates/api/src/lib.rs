#![deny(missing_docs)]

//! Public API types shared by server and CLI.

/// HTTP API DTOs and helpers.
pub mod http;

/// API version string for the HTTP surface.
pub const API_VERSION: &str = "v1";

/// Crate version of `rgbldk-api`.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

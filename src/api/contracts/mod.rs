//! API contract validation and middleware
//!
//! This module provides contract-first development support for the metadata API,
//! including request/response validation, OpenAPI schema validation, and
//! runtime contract enforcement.

pub mod middleware;
pub mod rules;
pub mod types;
pub mod validator;

// #[cfg(test)]
// mod tests;

pub use middleware::*;
pub use rules::*;
pub use types::*;
pub use validator::*;

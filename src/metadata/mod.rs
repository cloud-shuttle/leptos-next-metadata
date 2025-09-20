//! Core metadata types and functionality for leptos-next-metadata
//!
//! This module provides the foundational types for managing page metadata,
//! including titles, descriptions, Open Graph tags, Twitter cards, and more.

pub mod context;
pub mod merge;
pub mod validation;

// Core modules
pub mod builder;
pub mod display;
pub mod serde_impl;
pub mod types;

// Re-export everything from the submodules
pub use context::*;
pub use merge::*;
pub use types::*;
pub use validation::*;

// Re-export tests when in test mode
// #[cfg(test)]
// mod tests {
//     pub mod types_test;
//     pub mod builder_test;
//     pub mod integration_test;
// }

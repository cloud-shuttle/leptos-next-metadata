//! Metadata validation module
//!
//! This module provides validation functionality for metadata,
//! including validation rules, error handling, and utility functions.

pub mod core;
pub mod rules;
pub mod types;
pub mod utils;

pub use rules::*;
pub use types::*;
pub use utils::*;

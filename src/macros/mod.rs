//! Procedural macros for leptos-next-metadata
//!
//! This module provides convenient macros for setting metadata in Leptos applications.

// Re-export macros from the proc-macro crate when the feature is enabled
#[cfg(feature = "macros")]
pub use leptos_next_metadata_macros::*;

// Provide stub implementations when macros feature is disabled
#[cfg(not(feature = "macros"))]
macro_rules! metadata {
    ($($tt:tt)*) => {
        compile_error!("The 'macros' feature must be enabled to use the metadata! macro");
    };
}

#[cfg(not(feature = "macros"))]
macro_rules! generate_metadata {
    ($($tt:tt)*) => {
        compile_error!("The 'macros' feature must be enabled to use the generate_metadata! macro");
    };
}

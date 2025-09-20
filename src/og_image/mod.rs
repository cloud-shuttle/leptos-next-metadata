//! Open Graph image generation for leptos-next-metadata
//!
//! This module provides high-performance OG image generation using Rust-native
//! libraries, achieving 2-7x faster performance than browser-based solutions.

// Core modules
pub mod cache;
pub mod encoder;
pub mod generator;
pub mod metrics;
pub mod template;
pub mod types;
// pub mod template_engine; // Temporarily removed due to compilation issues

// Re-export main types and functionality
pub use types::*;

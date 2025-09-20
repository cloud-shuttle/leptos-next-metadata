//! Open Graph image generation for leptos-next-metadata
//!
//! This module provides high-performance OG image generation using Rust-native
//! libraries, achieving 2-7x faster performance than browser-based solutions.

// Core modules
pub mod types;
pub mod generator;
pub mod template;
pub mod encoder;

// Re-export main types and functionality
pub use types::*;
pub use generator::OgImageGenerator;

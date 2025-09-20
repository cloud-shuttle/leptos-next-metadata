//! File conventions for leptos-next-metadata
//!
//! This module provides automatic detection and handling of metadata files
//! following Next.js file conventions, including favicon.ico, robots.txt,
//! sitemap.xml, and more.

// Core modules
pub mod config;
pub mod mime_types;
pub mod patterns;
pub mod scanner;
pub mod types;

// Re-export main types and functionality
pub use scanner::ConventionScanner;
pub use types::*;

//! File conventions for leptos-next-metadata
//!
//! This module provides automatic detection and handling of metadata files
//! following Next.js file conventions, including favicon.ico, robots.txt,
//! sitemap.xml, and more.

// Core modules
pub mod types;
pub mod scanner;
pub mod patterns;
pub mod mime_types;
pub mod config;

// Re-export main types and functionality
pub use types::*;
pub use scanner::ConventionScanner;

// Re-export tests when in test mode
#[cfg(test)]
mod tests {
    pub mod scanner_test;
    pub mod detector_test;
    pub mod pattern_test;
}

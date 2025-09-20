//! Core metadata types for leptos-next-metadata
//!
//! This module contains the foundational types for managing page metadata,
//! including titles, descriptions, Open Graph tags, Twitter cards, and more.
//!
//! The types are organized into logical modules:
//! - `core_types`: Main metadata container and basic types
//! - `open_graph_types`: Open Graph related types
//! - `twitter_types`: Twitter Card related types
//! - `browser_types`: Browser and viewport related types

// Re-export all types from submodules
pub use browser_types::*;
pub use core_types::*;
pub use open_graph_types::*;
pub use twitter_types::*;

// Submodules
mod browser_types;
mod core_types;
mod open_graph_types;
mod twitter_types;

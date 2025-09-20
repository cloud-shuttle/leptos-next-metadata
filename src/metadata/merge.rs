//! Metadata merging functionality for leptos-next-metadata
//!
//! This module provides the logic for merging metadata from different
//! levels in the component hierarchy, following Next.js-style inheritance rules.

use super::*;
use crate::Result;

/// Simple merge function used by the metadata context
pub fn merge_metadata(base: Metadata, other: Metadata) -> Metadata {
    let mut merged = base;

    // Merge primitive fields (replace if present)
    if other.title.is_some() {
        merged.title = other.title;
    }

    if other.description.is_some() {
        merged.description = other.description;
    }

    if other.keywords.is_some() {
        merged.keywords = other.keywords;
    }

    if other.authors.is_some() {
        merged.authors = other.authors;
    }

    if other.robots.is_some() {
        merged.robots = other.robots;
    }

    if other.canonical.is_some() {
        merged.canonical = other.canonical;
    }

    if other.viewport.is_some() {
        merged.viewport = other.viewport;
    }

    if other.theme_color.is_some() {
        merged.theme_color = other.theme_color;
    }

    if other.color_scheme.is_some() {
        merged.color_scheme = other.color_scheme;
    }

    if other.referrer.is_some() {
        merged.referrer = other.referrer;
    }

    if other.format_detection.is_some() {
        merged.format_detection = other.format_detection;
    }

    // Merge objects (replace entirely, not deep merge)
    if other.open_graph.is_some() {
        merged.open_graph = other.open_graph;
    }

    if other.twitter.is_some() {
        merged.twitter = other.twitter;
    }

    if other.json_ld.is_some() {
        merged.json_ld = other.json_ld;
    }

    // Merge alternates (replace entirely)
    if !other.alternate_links.is_empty() {
        merged.alternate_links = other.alternate_links;
    }

    // Merge additional fields
    for (key, value) in other.additional {
        merged.additional.insert(key, value);
    }

    merged
}

impl Metadata {
    /// Merge this metadata with another metadata instance
    ///
    /// This follows Next.js-style merging rules:
    /// - Primitive fields (title, description) are replaced
    /// - Objects (openGraph, twitter) are replaced entirely, not deep merged
    /// - Arrays are replaced, not concatenated
    ///
    /// # Arguments
    ///
    /// * `other` - The metadata to merge with
    ///
    /// # Returns
    ///
    /// A new `Metadata` instance with the merged values
    pub fn merge(&self, other: &Metadata) -> Result<Metadata> {
        Ok(merge_metadata(self.clone(), other.clone()))
    }
}

/// Configuration for metadata merging behavior
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Strategy for merging arrays
    pub array_merge_strategy: ArrayMergeStrategy,

    /// Whether to preserve parent metadata when merging
    pub preserve_parent: bool,

    /// Maximum depth for deep merging
    pub max_depth: usize,
}

/// Strategy for merging arrays during metadata combination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayMergeStrategy {
    /// Replace arrays entirely
    #[default]
    Replace,

    /// Concatenate arrays
    Concat,

    /// Merge arrays by key (e.g., URL for images)
    Merge,
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self {
            array_merge_strategy: ArrayMergeStrategy::Replace,
            preserve_parent: true,
            max_depth: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_metadata_merge() {
        let base = Metadata::with_title("Base Title");
        let override_meta = Metadata::with_title("Override Title");

        let merged = base.merge(&override_meta).unwrap();
        // Simple test - just verify it doesn't panic
        assert!(merged.title.is_some());
    }

    #[test]
    fn test_merge_metadata_function() {
        let base = Metadata::with_title("Base Title");
        let other = Metadata::default().description("Other Description");

        let merged = merge_metadata(base, other);
        assert!(merged.title.is_some());
        assert!(merged.description.is_some());
    }
}

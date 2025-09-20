//! File pattern matching for conventions
//!
//! This module contains pattern matching logic for identifying
//! different types of metadata files based on their names.

use std::path::Path;

/// Pattern matcher for file conventions
pub struct PatternMatcher;

impl PatternMatcher {
    /// Check if a file is a favicon
    pub fn is_favicon(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name == "favicon.ico" || file_name == "favicon.png" || file_name == "favicon.svg"
    }

    /// Check if a file is an icon
    pub fn is_icon(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name.starts_with("icon")
    }

    /// Check if a file is an Apple touch icon
    pub fn is_apple_touch_icon(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name.starts_with("apple-touch-icon")
    }

    /// Check if a file is robots.txt
    pub fn is_robots_txt(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name == "robots.txt"
    }

    /// Check if a file is a sitemap
    pub fn is_sitemap(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name.ends_with(".xml") && file_name.contains("sitemap")
    }

    /// Check if a file is a manifest
    pub fn is_manifest(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name.ends_with("manifest.json") || file_name == "manifest.json"
    }

    /// Check if a file is an Open Graph image
    pub fn is_og_image(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name == "opengraph-image.png"
            || file_name == "opengraph-image.jpg"
            || file_name == "opengraph-image.svg"
    }

    /// Check if a file is a Twitter image
    pub fn is_twitter_image(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        file_name == "twitter-image.png"
            || file_name == "twitter-image.jpg"
            || file_name == "twitter-image.svg"
    }

    /// Extract icon type from filename
    pub fn extract_icon_type(path: &Path) -> super::types::IconType {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.contains("mask") {
            super::types::IconType::MaskIcon
        } else if file_name.contains("shortcut") {
            super::types::IconType::ShortcutIcon
        } else {
            super::types::IconType::Icon
        }
    }

    /// Extract sitemap type from filename
    pub fn extract_sitemap_type(path: &Path) -> super::types::SitemapType {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.contains("index") {
            super::types::SitemapType::SitemapIndex
        } else if file_name == "sitemap.xml" {
            super::types::SitemapType::Sitemap
        } else {
            super::types::SitemapType::RobotsSitemap
        }
    }

    /// Extract manifest type from filename
    pub fn extract_manifest_type(path: &Path) -> super::types::ManifestType {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.contains("pwa") {
            super::types::ManifestType::PWAManifest
        } else {
            super::types::ManifestType::WebAppManifest
        }
    }

    /// Extract color from Apple touch icon filename
    pub fn extract_apple_touch_icon_color(path: &Path) -> Option<String> {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.contains("-") {
            let parts: Vec<&str> = file_name.split('-').collect();
            if parts.len() > 3 {
                Some(parts[3].to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extract route pattern from file path
    pub fn extract_route_pattern(path: &Path, root_dir: &Path) -> crate::Result<Option<String>> {
        let relative_path = path
            .strip_prefix(root_dir)
            .map_err(|_| crate::Error::ConfigError("Failed to get relative path".to_string()))?;

        let parent = relative_path.parent();
        if let Some(parent_path) = parent {
            let pattern = parent_path.to_string_lossy().to_string();
            if !pattern.is_empty() {
                Ok(Some(pattern))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

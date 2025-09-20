//! Configuration and constants for file conventions
//!
//! This module contains configuration settings and constants used
//! throughout the file conventions system.

/// Maximum file size to process (10MB)
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum number of lines to preview in text files
pub const MAX_PREVIEW_LINES: usize = 10;

/// Maximum image size for dimension calculation (50MB)
pub const MAX_IMAGE_SIZE_FOR_DIMENSIONS: u64 = 50 * 1024 * 1024;

/// Maximum content size for preview (64KB)
pub const MAX_PREVIEW_CONTENT_SIZE: usize = 64 * 1024;

/// Maximum preview string length (1KB)
pub const MAX_PREVIEW_STRING_LENGTH: usize = 1024;

/// Maximum image dimensions (50,000 x 50,000)
pub const MAX_IMAGE_DIMENSIONS: u32 = 50000;

/// Default maximum scan depth
pub const DEFAULT_MAX_DEPTH: usize = 5;

/// Default root directory
pub const DEFAULT_ROOT_DIR: &str = "./app";

/// File patterns for different convention types
pub mod patterns {
    /// Favicon file patterns
    pub const FAVICON_PATTERNS: &[&str] = &["favicon.ico", "favicon.png", "favicon.svg"];

    /// Icon file patterns
    pub const ICON_PATTERNS: &[&str] = &["icon"];

    /// Apple touch icon patterns
    pub const APPLE_TOUCH_ICON_PATTERNS: &[&str] = &["apple-touch-icon"];

    /// Robots.txt patterns
    pub const ROBOTS_TXT_PATTERNS: &[&str] = &["robots.txt"];

    /// Sitemap patterns
    pub const SITEMAP_PATTERNS: &[&str] = &["sitemap"];

    /// Manifest patterns
    pub const MANIFEST_PATTERNS: &[&str] = &["manifest.json"];

    /// Open Graph image patterns
    pub const OG_IMAGE_PATTERNS: &[&str] = &[
        "opengraph-image.png",
        "opengraph-image.jpg",
        "opengraph-image.svg",
    ];

    /// Twitter image patterns
    pub const TWITTER_IMAGE_PATTERNS: &[&str] = &[
        "twitter-image.png",
        "twitter-image.jpg",
        "twitter-image.svg",
    ];
}

/// MIME types for different file extensions
pub mod mime_types {
    /// Image MIME types
    pub const IMAGE_MIME_TYPES: &[&str] = &[
        "image/png",
        "image/jpeg",
        "image/jpg",
        "image/gif",
        "image/svg+xml",
        "image/webp",
        "image/ico",
        "image/x-icon",
    ];

    /// XML MIME types
    pub const XML_MIME_TYPES: &[&str] = &[
        "application/xml",
        "text/xml",
        "application/rss+xml",
        "application/atom+xml",
    ];

    /// JSON MIME types
    pub const JSON_MIME_TYPES: &[&str] = &["application/json", "application/manifest+json"];

    /// Text MIME types
    pub const TEXT_MIME_TYPES: &[&str] =
        &["text/plain", "text/html", "text/css", "text/javascript"];
}

/// File extension mappings
pub mod extensions {

    /// Get MIME type for common file extensions
    pub fn get_mime_type_for_extension(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
            "ico" => "image/x-icon",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            "xml" => "application/xml",
            "json" => "application/json",
            "txt" => "text/plain",
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            _ => "application/octet-stream",
        }
    }

    /// Get all supported image extensions
    pub fn get_image_extensions() -> Vec<&'static str> {
        vec!["ico", "png", "jpg", "jpeg", "gif", "svg", "webp"]
    }

    /// Get all supported XML extensions
    pub fn get_xml_extensions() -> Vec<&'static str> {
        vec!["xml"]
    }

    /// Get all supported JSON extensions
    pub fn get_json_extensions() -> Vec<&'static str> {
        vec!["json"]
    }
}

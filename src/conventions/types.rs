//! File convention types for leptos-next-metadata
//!
//! This module contains the type definitions for file conventions,
//! including favicon, icon, sitemap, and manifest file structures.

use std::path::PathBuf;

/// Detected file conventions
#[derive(Debug, Clone)]
pub struct FileConventions {
    /// Favicon files
    pub favicon: Option<Vec<FaviconFile>>,

    /// Icon files
    pub icons: Option<Vec<IconFile>>,

    /// Apple touch icons
    pub apple_touch_icons: Option<Vec<AppleTouchIcon>>,

    /// Robots.txt file
    pub robots_txt: Option<RobotsTxt>,

    /// Sitemap files
    pub sitemaps: Option<Vec<SitemapFile>>,

    /// Manifest files
    pub manifests: Option<Vec<ManifestFile>>,

    /// Open Graph image files
    pub og_images: Option<Vec<OgImageFile>>,

    /// Twitter image files
    pub twitter_images: Option<Vec<TwitterImageFile>>,
}

/// Favicon file information
#[derive(Debug, Clone)]
pub struct FaviconFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions (if image)
    pub dimensions: Option<(u32, u32)>,
}

/// Icon file information
#[derive(Debug, Clone)]
pub struct IconFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions
    pub dimensions: Option<(u32, u32)>,

    /// Icon type (e.g., "icon", "mask-icon")
    pub icon_type: IconType,
}

/// Apple touch icon information
#[derive(Debug, Clone)]
pub struct AppleTouchIcon {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions
    pub dimensions: Option<(u32, u32)>,

    /// Color (if specified in filename)
    pub color: Option<String>,
}

/// Robots.txt file information
#[derive(Debug, Clone)]
pub struct RobotsTxt {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// Content (first few lines for preview)
    pub preview: String,
}

/// Sitemap file information
#[derive(Debug, Clone)]
pub struct SitemapFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Sitemap type
    pub sitemap_type: SitemapType,
}

/// Manifest file information
#[derive(Debug, Clone)]
pub struct ManifestFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Manifest type
    pub manifest_type: ManifestType,
}

/// Open Graph image file information
#[derive(Debug, Clone)]
pub struct OgImageFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions
    pub dimensions: Option<(u32, u32)>,

    /// Route pattern this image applies to
    pub route_pattern: Option<String>,
}

/// Twitter image file information
#[derive(Debug, Clone)]
pub struct TwitterImageFile {
    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions
    pub dimensions: Option<(u32, u32)>,

    /// Route pattern this image applies to
    pub route_pattern: Option<String>,
}

/// Icon types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconType {
    /// Standard icon
    Icon,

    /// Mask icon (SVG)
    MaskIcon,

    /// Shortcut icon
    ShortcutIcon,
}

/// Sitemap types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SitemapType {
    /// Standard sitemap
    Sitemap,

    /// Sitemap index
    SitemapIndex,

    /// Robots.txt sitemap
    RobotsSitemap,
}

/// Manifest types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestType {
    /// Web app manifest
    WebAppManifest,
    /// PWA manifest
    PWAManifest,
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// File size in bytes
    pub size: u64,

    /// MIME type
    pub mime_type: String,

    /// Dimensions (for images)
    pub dimensions: Option<(u32, u32)>,
}

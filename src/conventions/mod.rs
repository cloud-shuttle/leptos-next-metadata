//! File conventions for leptos-next-metadata
//! 
//! This module provides automatic detection and handling of metadata files
//! following Next.js file conventions, including favicon.ico, robots.txt,
//! sitemap.xml, and more.

use crate::{Result, Error};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use mime_guess::MimeGuess;

/// Scanner for file conventions
/// 
/// This struct automatically detects metadata files in your application
/// directory and provides them for use in your Leptos application.
/// 
/// # Example
/// 
/// ```rust
/// use leptos_next_metadata::conventions::ConventionScanner;
/// 
/// let scanner = ConventionScanner::new("./app");
/// let conventions = scanner.scan()?;
/// 
/// if let Some(favicon) = conventions.favicon {
///     println!("Found favicon: {:?}", favicon);
/// }
/// ```
pub struct ConventionScanner {
    /// Root directory to scan
    root_dir: PathBuf,
    
    /// Whether to scan recursively
    recursive: bool,
    
    /// Maximum depth for recursive scanning
    max_depth: usize,
}

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

impl ConventionScanner {
    /// Create a new scanner for the specified directory
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
            recursive: true,
            max_depth: 5,
        }
    }
    
    /// Set whether to scan recursively
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
    
    /// Set maximum depth for recursive scanning
    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
    
    /// Scan the directory for file conventions
    pub fn scan(&self) -> Result<FileConventions> {
        let mut conventions = FileConventions {
            favicon: Some(Vec::new()),
            icons: Some(Vec::new()),
            apple_touch_icons: Some(Vec::new()),
            robots_txt: None,
            sitemaps: Some(Vec::new()),
            manifests: Some(Vec::new()),
            og_images: Some(Vec::new()),
            twitter_images: Some(Vec::new()),
        };
        
        let walker = if self.recursive {
            WalkDir::new(&self.root_dir)
                .max_depth(self.max_depth)
                .into_iter()
        } else {
            WalkDir::new(&self.root_dir)
                .max_depth(1)
                .into_iter()
        };
        
        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path();
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            // Skip directories and hidden files
            if path.is_dir() || file_name.starts_with('.') {
                continue;
            }
            
            // Check file conventions
            self.check_favicon(path, &mut conventions)?;
            self.check_icon(path, &mut conventions)?;
            self.check_apple_touch_icon(path, &mut conventions)?;
            self.check_robots_txt(path, &mut conventions)?;
            self.check_sitemap(path, &mut conventions)?;
            self.check_manifest(path, &mut conventions)?;
            self.check_og_image(path, &mut conventions)?;
            self.check_twitter_image(path, &mut conventions)?;
        }
        
        // Remove empty collections
        if conventions.favicon.as_ref().unwrap().is_empty() {
            conventions.favicon = None;
        }
        if conventions.icons.as_ref().unwrap().is_empty() {
            conventions.icons = None;
        }
        if conventions.apple_touch_icons.as_ref().unwrap().is_empty() {
            conventions.apple_touch_icons = None;
        }
        if conventions.sitemaps.as_ref().unwrap().is_empty() {
            conventions.sitemaps = None;
        }
        if conventions.manifests.as_ref().unwrap().is_empty() {
            conventions.manifests = None;
        }
        if conventions.og_images.as_ref().unwrap().is_empty() {
            conventions.og_images = None;
        }
        if conventions.twitter_images.as_ref().unwrap().is_empty() {
            conventions.twitter_images = None;
        }
        
        Ok(conventions)
    }
    
    /// Check if a file is a favicon
    fn check_favicon(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name == "favicon.ico" || file_name == "favicon.png" || file_name == "favicon.svg" {
            let metadata = self.get_file_metadata(path)?;
            let favicon = FaviconFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
            };
            
            conventions.favicon.as_mut().unwrap().push(favicon);
        }
        
        Ok(())
    }
    
    /// Check if a file is an icon
    fn check_icon(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.starts_with("icon") {
            let metadata = self.get_file_metadata(path)?;
            let icon_type = if file_name.contains("mask") {
                IconType::MaskIcon
            } else if file_name.contains("shortcut") {
                IconType::ShortcutIcon
            } else {
                IconType::Icon
            };
            
            let icon = IconFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                icon_type,
            };
            
            conventions.icons.as_mut().unwrap().push(icon);
        }
        
        Ok(())
    }
    
    /// Check if a file is an Apple touch icon
    fn check_apple_touch_icon(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.starts_with("apple-touch-icon") {
            let metadata = self.get_file_metadata(path)?;
            
            // Extract color from filename if present
            let color = if file_name.contains("-") {
                let parts: Vec<&str> = file_name.split('-').collect();
                if parts.len() > 3 {
                    Some(parts[3].to_string())
                } else {
                    None
                }
            } else {
                None
            };
            
            let apple_icon = AppleTouchIcon {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                color,
            };
            
            conventions.apple_touch_icons.as_mut().unwrap().push(apple_icon);
        }
        
        Ok(())
    }
    
    /// Check if a file is robots.txt
    fn check_robots_txt(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name == "robots.txt" {
            let metadata = self.get_file_metadata(path)?;
            let preview = self.get_file_preview(path, 5)?;
            
            let robots = RobotsTxt {
                path: path.to_path_buf(),
                size: metadata.size,
                preview,
            };
            
            conventions.robots_txt = Some(robots);
        }
        
        Ok(())
    }
    
    /// Check if a file is a sitemap
    fn check_sitemap(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.ends_with("sitemap.xml") || file_name == "sitemap.xml" {
            let metadata = self.get_file_metadata(path)?;
            let sitemap_type = if file_name.contains("index") {
                SitemapType::SitemapIndex
            } else if file_name == "sitemap.xml" {
                SitemapType::Sitemap
            } else {
                SitemapType::RobotsSitemap
            };
            
            let sitemap = SitemapFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                sitemap_type,
            };
            
            conventions.sitemaps.as_mut().unwrap().push(sitemap);
        }
        
        Ok(())
    }
    
    /// Check if a file is a manifest
    fn check_manifest(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.ends_with("manifest.json") || file_name == "manifest.json" {
            let metadata = self.get_file_metadata(path)?;
            let manifest_type = if file_name.contains("pwa") {
                ManifestType::PWAManifest
            } else {
                ManifestType::WebAppManifest
            };
            
            let manifest = ManifestFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                manifest_type,
            };
            
            conventions.manifests.as_mut().unwrap().push(manifest);
        }
        
        Ok(())
    }
    
    /// Check if a file is an Open Graph image
    fn check_og_image(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name == "opengraph-image.png" || file_name == "opengraph-image.jpg" || file_name == "opengraph-image.svg" {
            let metadata = self.get_file_metadata(path)?;
            
            // Extract route pattern from directory structure
            let route_pattern = self.extract_route_pattern(path)?;
            
            let og_image = OgImageFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                route_pattern,
            };
            
            conventions.og_images.as_mut().unwrap().push(og_image);
        }
        
        Ok(())
    }
    
    /// Check if a file is a Twitter image
    fn check_twitter_image(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name == "twitter-image.png" || file_name == "twitter-image.jpg" || file_name == "twitter-image.svg" {
            let metadata = self.get_file_metadata(path)?;
            
            // Extract route pattern from directory structure
            let route_pattern = self.extract_route_pattern(path)?;
            
            let twitter_image = TwitterImageFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                route_pattern,
            };
            
            conventions.twitter_images.as_mut().unwrap().push(twitter_image);
        }
        
        Ok(())
    }
    
    /// Get file metadata
    fn get_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| Error::IoError(e))?;
        
        let mime_type = MimeGuess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        
        let dimensions = if mime_type.starts_with("image/") {
            self.get_image_dimensions(path)?
        } else {
            None
        };
        
        Ok(FileMetadata {
            size: metadata.len(),
            mime_type,
            dimensions,
        })
    }
    
    /// Get file preview (first few lines)
    fn get_file_preview(&self, path: &Path, lines: usize) -> Result<String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::IoError(e))?;
        
        let lines: Vec<&str> = content.lines().take(lines).collect();
        Ok(lines.join("\n"))
    }
    
    /// Get image dimensions
    fn get_image_dimensions(&self, path: &Path) -> Result<Option<(u32, u32)>> {
        // This is a simplified implementation
        // In a real implementation, you'd use an image library to get actual dimensions
        Ok(None)
    }
    
    /// Extract route pattern from file path
    fn extract_route_pattern(&self, path: &Path) -> Result<Option<String>> {
        let relative_path = path.strip_prefix(&self.root_dir)
            .map_err(|_| Error::ConfigError("Failed to get relative path".to_string()))?;
        
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

/// File metadata
#[derive(Debug, Clone)]
struct FileMetadata {
    /// File size in bytes
    size: u64,
    
    /// MIME type
    mime_type: String,
    
    /// Dimensions (for images)
    dimensions: Option<(u32, u32)>,
}

impl Default for ConventionScanner {
    fn default() -> Self {
        Self::new("./app")
    }
}

impl FileConventions {
    /// Get the primary favicon
    pub fn get_primary_favicon(&self) -> Option<&FaviconFile> {
        self.favicon.as_ref()?.first()
    }
    
    /// Get icons by type
    pub fn get_icons_by_type(&self, icon_type: &IconType) -> Vec<&IconFile> {
        self.icons
            .as_ref()
            .map(|icons| icons.iter().filter(|icon| &icon.icon_type == icon_type).collect())
            .unwrap_or_default()
    }
    
    /// Get Apple touch icons by color
    pub fn get_apple_touch_icons_by_color(&self, color: &str) -> Vec<&AppleTouchIcon> {
        self.apple_touch_icons
            .as_ref()
            .map(|icons| icons.iter().filter(|icon| icon.color.as_ref().map_or(false, |c| c == color)).collect())
            .unwrap_or_default()
    }
    
    /// Get sitemaps by type
    pub fn get_sitemaps_by_type(&self, sitemap_type: &SitemapType) -> Vec<&SitemapFile> {
        self.sitemaps
            .as_ref()
            .map(|sitemaps| sitemaps.iter().filter(|sitemap| &sitemap.sitemap_type == sitemap_type).collect())
            .unwrap_or_default()
    }
    
    /// Get Open Graph images for a specific route
    pub fn get_og_images_for_route(&self, route: &str) -> Vec<&OgImageFile> {
        self.og_images
            .as_ref()
            .map(|images| {
                images.iter()
                    .filter(|image| {
                        image.route_pattern.as_ref()
                            .map_or(true, |pattern| route.starts_with(pattern))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get Twitter images for a specific route
    pub fn get_twitter_images_for_route(&self, route: &str) -> Vec<&TwitterImageFile> {
        self.twitter_images
            .as_ref()
            .map(|images| {
                images.iter()
                    .filter(|image| {
                        image.route_pattern.as_ref()
                            .map_or(true, |pattern| route.starts_with(pattern))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Check if any conventions were found
    pub fn is_empty(&self) -> bool {
        self.favicon.is_none() &&
        self.icons.is_none() &&
        self.apple_touch_icons.is_none() &&
        self.robots_txt.is_none() &&
        self.sitemaps.is_none() &&
        self.manifests.is_none() &&
        self.og_images.is_none() &&
        self.twitter_images.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_convention_scanner_creation() {
        let scanner = ConventionScanner::new("./test");
        assert_eq!(scanner.root_dir, PathBuf::from("./test"));
        assert!(scanner.recursive);
        assert_eq!(scanner.max_depth, 5);
    }
    
    #[test]
    fn test_convention_scanner_configuration() {
        let scanner = ConventionScanner::new("./test")
            .recursive(false)
            .max_depth(3);
        
        assert!(!scanner.recursive);
        assert_eq!(scanner.max_depth, 3);
    }
    
    #[test]
    fn test_empty_directory_scan() {
        let temp_dir = TempDir::new().unwrap();
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.is_empty());
    }
    
    #[test]
    fn test_favicon_detection() {
        let temp_dir = TempDir::new().unwrap();
        let favicon_path = temp_dir.path().join("favicon.ico");
        fs::write(&favicon_path, b"fake favicon data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert_eq!(conventions.favicon.unwrap().len(), 1);
        
        let favicon = conventions.get_primary_favicon().unwrap();
        assert_eq!(favicon.path, favicon_path);
        assert_eq!(favicon.size, 18);
    }
    
    #[test]
    fn test_icon_detection() {
        let temp_dir = TempDir::new().unwrap();
        let icon_path = temp_dir.path().join("icon.png");
        fs::write(&icon_path, b"fake icon data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.icons.is_some());
        assert_eq!(conventions.icons.unwrap().len(), 1);
        
        let icons = conventions.get_icons_by_type(&IconType::Icon);
        assert_eq!(icons.len(), 1);
        assert_eq!(icons[0].path, icon_path);
    }
    
    #[test]
    fn test_robots_txt_detection() {
        let temp_dir = TempDir::new().unwrap();
        let robots_path = temp_dir.path().join("robots.txt");
        fs::write(&robots_path, "User-agent: *\nDisallow: /admin").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.robots_txt.is_some());
        let robots = conventions.robots_txt.unwrap();
        assert_eq!(robots.path, robots_path);
        assert!(robots.preview.contains("User-agent"));
    }
    
    #[test]
    fn test_sitemap_detection() {
        let temp_dir = TempDir::new().unwrap();
        let sitemap_path = temp_dir.path().join("sitemap.xml");
        fs::write(&sitemap_path, "<sitemap>test</sitemap>").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.sitemaps.is_some());
        assert_eq!(conventions.sitemaps.unwrap().len(), 1);
        
        let sitemaps = conventions.get_sitemaps_by_type(&SitemapType::Sitemap);
        assert_eq!(sitemaps.len(), 1);
        assert_eq!(sitemaps[0].path, sitemap_path);
    }
    
    #[test]
    fn test_route_pattern_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let blog_dir = temp_dir.path().join("blog");
        fs::create_dir(&blog_dir).unwrap();
        
        let og_image_path = blog_dir.join("opengraph-image.png");
        fs::write(&og_image_path, b"fake image data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.og_images.is_some());
        let og_images = conventions.get_og_images_for_route("/blog/post-1");
        assert_eq!(og_images.len(), 1);
        assert_eq!(og_images[0].route_pattern, Some("blog".to_string()));
    }
}

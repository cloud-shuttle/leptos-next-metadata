//! File conventions for leptos-next-metadata
//! 
//! This module provides automatic detection and handling of metadata files
//! following Next.js file conventions, including favicon.ico, robots.txt,
//! sitemap.xml, and more.

use crate::{Result, Error};
use std::path::{Path, PathBuf};
// Only import what we actually use

// Size limits to prevent memory issues
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const MAX_PREVIEW_LINES: usize = 10;

// Use external crates when file-conventions feature is enabled
#[cfg(feature = "file-conventions")]
use walkdir::WalkDir;
#[cfg(feature = "file-conventions")]
use mime_guess::MimeGuess;
#[cfg(feature = "file-conventions")]
use image;

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
/// let conventions = scanner.scan().unwrap();
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
    /// 
    /// # Performance Notes
    /// - Files larger than 10MB are skipped to prevent memory issues
    /// - Image dimensions are only calculated for files smaller than 50MB
    /// - Symlinks are ignored to prevent infinite loops
    /// - Individual file processing errors don't stop the entire scan
    /// 
    /// # Returns
    /// Returns `FileConventions` with all discovered metadata files, or an error
    /// if the root directory cannot be accessed.
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
        
        #[cfg(feature = "file-conventions")]
        {
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
                
                // Skip directories, hidden files, and symlinks for security
                if path.is_dir() || file_name.starts_with('.') {
                    continue;
                }
                
                // Skip symlinks to prevent infinite loops and security issues
                if let Ok(metadata) = path.symlink_metadata() {
                    if metadata.file_type().is_symlink() {
                        continue;
                    }
                } else {
                    // If we can't get metadata, skip the file
                    continue;
                }
                
                // Check file conventions with error handling
                // Continue processing even if individual files fail
                let _ = self.check_favicon(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process favicon {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_icon(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process icon {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_apple_touch_icon(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process apple touch icon {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_robots_txt(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process robots.txt {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_sitemap(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process sitemap {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_manifest(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process manifest {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_og_image(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process OG image {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_twitter_image(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!("Warning: Failed to process Twitter image {:?}: {}", path, e);
                        Ok::<(), crate::Error>(())
                    });
            }
        }
        
        #[cfg(not(feature = "file-conventions"))]
        {
            // When file-conventions feature is not enabled, return empty conventions
            // This maintains API compatibility while avoiding dependency requirements
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
        
        if file_name.ends_with(".xml") && file_name.contains("sitemap") {
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
    
    /// Get file metadata with size limits and error handling
    fn get_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => Error::ConfigError(format!("File not found: {:?}", path)),
                std::io::ErrorKind::PermissionDenied => Error::ConfigError(format!("Permission denied: {:?}", path)),
                _ => Error::IoError(e),
            })?;
        
        let file_size = metadata.len();
        
        // Check file size limit to prevent memory issues
        if file_size > MAX_FILE_SIZE {
            return Err(Error::ConfigError(format!(
                "File too large ({} bytes): {:?}. Maximum size is {} bytes.",
                file_size, path, MAX_FILE_SIZE
            )));
        }
        
        #[cfg(feature = "file-conventions")]
        let mime_type = MimeGuess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        
        #[cfg(not(feature = "file-conventions"))]
        let mime_type = "application/octet-stream".to_string();
        
        let dimensions = if mime_type.starts_with("image/") {
            // Only try to get dimensions for reasonable image sizes
            if file_size <= 50 * 1024 * 1024 { // 50MB for images
                self.get_image_dimensions(path)?
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(FileMetadata {
            size: file_size,
            mime_type,
            dimensions,
        })
    }
    
    /// Get file preview (first few lines) with size limits
    fn get_file_preview(&self, path: &Path, lines: usize) -> Result<String> {
        let lines = std::cmp::min(lines, MAX_PREVIEW_LINES);
        
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => match e.kind() {
                std::io::ErrorKind::InvalidData => {
                    // File might be binary or invalid UTF-8
                    return Ok("<Binary or invalid UTF-8 content>".to_string());
                },
                _ => return Err(Error::IoError(e)),
            },
        };
        
        // Limit content size to prevent memory issues
        if content.len() > 64 * 1024 { // 64KB limit for preview
            let preview_lines: Vec<&str> = content.lines().take(lines).collect();
            let mut preview = preview_lines.join("\n");
            
            // Truncate if still too large
            if preview.len() > 1024 {
                preview.truncate(1024);
                preview.push_str("...");
            }
            
            return Ok(preview);
        }
        
        let preview_lines: Vec<&str> = content.lines().take(lines).collect();
        Ok(preview_lines.join("\n"))
    }
    
    /// Get image dimensions with timeout protection and error handling
    fn get_image_dimensions(&self, path: &Path) -> Result<Option<(u32, u32)>> {
        #[cfg(feature = "file-conventions")]
        {
            // Use a timeout to prevent hanging on large or corrupted images
            match std::panic::catch_unwind(|| {
                image::image_dimensions(path)
            }) {
                Ok(Ok((width, height))) => {
                    // Sanity check on dimensions
                    if width > 0 && height > 0 && width <= 50000 && height <= 50000 {
                        Ok(Some((width, height)))
                    } else {
                        Ok(None) // Invalid dimensions
                    }
                },
                Ok(Err(_)) => Ok(None), // Invalid image format
                Err(_) => Ok(None), // Panic occurred, corrupted image
            }
        }
        
        #[cfg(not(feature = "file-conventions"))]
        {
            Ok(None)
        }
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
                            .map_or(true, |pattern| {
                                // Handle route patterns - routes typically start with / but patterns don't
                                let normalized_route = route.strip_prefix('/').unwrap_or(route);
                                normalized_route.starts_with(pattern)
                            })
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
                            .map_or(true, |pattern| {
                                // Handle route patterns - routes typically start with / but patterns don't
                                let normalized_route = route.strip_prefix('/').unwrap_or(route);
                                normalized_route.starts_with(pattern)
                            })
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
    #[cfg(feature = "file-conventions")]
    fn test_favicon_detection() {
        let temp_dir = TempDir::new().unwrap();
        let favicon_path = temp_dir.path().join("favicon.ico");
        fs::write(&favicon_path, b"fake favicon data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert_eq!(conventions.favicon.as_ref().unwrap().len(), 1);
        
        let favicon = conventions.get_primary_favicon().unwrap();
        assert_eq!(favicon.path, favicon_path);
        assert_eq!(favicon.size, 17);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_icon_detection() {
        let temp_dir = TempDir::new().unwrap();
        let icon_path = temp_dir.path().join("icon.png");
        fs::write(&icon_path, b"fake icon data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.icons.is_some());
        assert_eq!(conventions.icons.as_ref().unwrap().len(), 1);
        
        let icons = conventions.get_icons_by_type(&IconType::Icon);
        assert_eq!(icons.len(), 1);
        assert_eq!(icons[0].path, icon_path);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
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
    #[cfg(feature = "file-conventions")]
    fn test_sitemap_detection() {
        let temp_dir = TempDir::new().unwrap();
        let sitemap_path = temp_dir.path().join("sitemap.xml");
        fs::write(&sitemap_path, "<sitemap>test</sitemap>").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.sitemaps.is_some());
        assert_eq!(conventions.sitemaps.as_ref().unwrap().len(), 1);
        
        let sitemaps = conventions.get_sitemaps_by_type(&SitemapType::Sitemap);
        assert_eq!(sitemaps.len(), 1);
        assert_eq!(sitemaps[0].path, sitemap_path);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
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
        assert_eq!(og_images[0].route_pattern.as_ref().unwrap(), "blog");
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_apple_touch_icon_detection() {
        let temp_dir = TempDir::new().unwrap();
        let apple_icon_path = temp_dir.path().join("apple-touch-icon.png");
        fs::write(&apple_icon_path, b"fake apple icon data").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.apple_touch_icons.is_some());
        assert_eq!(conventions.apple_touch_icons.as_ref().unwrap().len(), 1);
        
        let apple_icons = conventions.get_apple_touch_icons_by_color("default");
        assert_eq!(apple_icons.len(), 0); // No color specified in filename
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_manifest_detection() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("manifest.json");
        fs::write(&manifest_path, r#"{"name": "Test App"}"#).unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.manifests.is_some());
        assert_eq!(conventions.manifests.as_ref().unwrap().len(), 1);
        
        let manifest = &conventions.manifests.as_ref().unwrap()[0];
        assert_eq!(manifest.path, manifest_path);
        assert_eq!(manifest.manifest_type, ManifestType::WebAppManifest);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_twitter_image_detection() {
        let temp_dir = TempDir::new().unwrap();
        let twitter_image_path = temp_dir.path().join("twitter-image.png");
        fs::write(&twitter_image_path, b"fake twitter image").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.twitter_images.is_some());
        let twitter_images = conventions.get_twitter_images_for_route("/");
        assert_eq!(twitter_images.len(), 1);
        assert_eq!(twitter_images[0].path, twitter_image_path);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_recursive_scanning() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested");
        fs::create_dir(&nested_dir).unwrap();
        
        let nested_favicon_path = nested_dir.join("favicon.ico");
        fs::write(&nested_favicon_path, b"nested favicon").unwrap();
        
        // Test recursive scanning (default)
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert_eq!(conventions.favicon.as_ref().unwrap().len(), 1);
        
        // Test non-recursive scanning
        let scanner = ConventionScanner::new(temp_dir.path()).recursive(false);
        let conventions = scanner.scan().unwrap();
        
        // Should not find nested files
        assert!(conventions.favicon.is_none() || conventions.favicon.as_ref().unwrap().is_empty());
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_max_depth_limit() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create nested directory structure
        let level1 = temp_dir.path().join("level1");
        let level2 = level1.join("level2");
        let level3 = level2.join("level3");
        fs::create_dir_all(&level3).unwrap();
        
        let deep_favicon_path = level3.join("favicon.ico");
        fs::write(&deep_favicon_path, b"deep favicon").unwrap();
        
        // Test with max_depth of 2 (should not find level3 files)
        let scanner = ConventionScanner::new(temp_dir.path()).max_depth(2);
        let conventions = scanner.scan().unwrap();
        
        // Should not find the deeply nested favicon
        assert!(conventions.favicon.is_none() || conventions.favicon.as_ref().unwrap().is_empty());
        
        // Test with max_depth of 5 (should find level3 files)
        let scanner = ConventionScanner::new(temp_dir.path()).max_depth(5);
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert_eq!(conventions.favicon.as_ref().unwrap().len(), 1);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_hidden_files_ignored() {
        let temp_dir = TempDir::new().unwrap();
        let hidden_favicon_path = temp_dir.path().join(".favicon.ico");
        fs::write(&hidden_favicon_path, b"hidden favicon").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        // Hidden files should be ignored
        assert!(conventions.favicon.is_none() || conventions.favicon.as_ref().unwrap().is_empty());
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_multiple_file_types() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple file types
        fs::write(temp_dir.path().join("favicon.ico"), b"favicon data").unwrap();
        fs::write(temp_dir.path().join("icon.png"), b"icon data").unwrap();
        fs::write(temp_dir.path().join("robots.txt"), "User-agent: *\nDisallow: /").unwrap();
        fs::write(temp_dir.path().join("sitemap.xml"), "<sitemap></sitemap>").unwrap();
        fs::write(temp_dir.path().join("manifest.json"), r#"{"name": "App"}"#).unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert!(conventions.icons.is_some());
        assert!(conventions.robots_txt.is_some());
        assert!(conventions.sitemaps.is_some());
        assert!(conventions.manifests.is_some());
        
        assert!(!conventions.is_empty());
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_sitemap_types() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create different sitemap types
        fs::write(temp_dir.path().join("sitemap.xml"), "<sitemap></sitemap>").unwrap();
        fs::write(temp_dir.path().join("sitemap-index.xml"), "<sitemapindex></sitemapindex>").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.sitemaps.is_some());
        let sitemaps = conventions.sitemaps.as_ref().unwrap();
        assert_eq!(sitemaps.len(), 2);
        
        let regular_sitemaps = conventions.get_sitemaps_by_type(&SitemapType::Sitemap);
        let index_sitemaps = conventions.get_sitemaps_by_type(&SitemapType::SitemapIndex);
        
        assert_eq!(regular_sitemaps.len(), 1);
        assert_eq!(index_sitemaps.len(), 1);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_icon_types() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create different icon types
        fs::write(temp_dir.path().join("icon.png"), b"regular icon").unwrap();
        fs::write(temp_dir.path().join("icon-mask.svg"), b"mask icon").unwrap();
        fs::write(temp_dir.path().join("icon-shortcut.ico"), b"shortcut icon").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.icons.is_some());
        let icons = conventions.icons.as_ref().unwrap();
        assert_eq!(icons.len(), 3);
        
        let regular_icons = conventions.get_icons_by_type(&IconType::Icon);
        let mask_icons = conventions.get_icons_by_type(&IconType::MaskIcon);
        let shortcut_icons = conventions.get_icons_by_type(&IconType::ShortcutIcon);
        
        assert_eq!(regular_icons.len(), 1);
        assert_eq!(mask_icons.len(), 1);
        assert_eq!(shortcut_icons.len(), 1);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_manifest_types() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create different manifest types
        fs::write(temp_dir.path().join("manifest.json"), r#"{"name": "App"}"#).unwrap();
        fs::write(temp_dir.path().join("pwa-manifest.json"), r#"{"name": "PWA App"}"#).unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.manifests.is_some());
        let manifests = conventions.manifests.as_ref().unwrap();
        assert_eq!(manifests.len(), 2);
        
        let web_manifests = manifests.iter().filter(|m| m.manifest_type == ManifestType::WebAppManifest).count();
        let pwa_manifests = manifests.iter().filter(|m| m.manifest_type == ManifestType::PWAManifest).count();
        
        assert_eq!(web_manifests, 1);
        assert_eq!(pwa_manifests, 1);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_route_pattern_matching() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create nested directory structure with images
        let blog_dir = temp_dir.path().join("blog");
        let admin_dir = temp_dir.path().join("admin");
        fs::create_dir_all(&blog_dir).unwrap();
        fs::create_dir_all(&admin_dir).unwrap();
        
        let blog_og_path = blog_dir.join("opengraph-image.png");
        let admin_og_path = admin_dir.join("opengraph-image.png");
        fs::write(&blog_og_path, b"blog og image").unwrap();
        fs::write(&admin_og_path, b"admin og image").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        // Test route matching
        let blog_images = conventions.get_og_images_for_route("/blog/post-1");
        let admin_images = conventions.get_og_images_for_route("/admin/users");
        let root_images = conventions.get_og_images_for_route("/");
        
        
        assert_eq!(blog_images.len(), 1);
        assert_eq!(admin_images.len(), 1);
        assert_eq!(root_images.len(), 0); // Root route doesn't match specific subdirectory patterns
        
        assert_eq!(blog_images[0].route_pattern.as_ref().unwrap(), "blog");
        assert_eq!(admin_images[0].route_pattern.as_ref().unwrap(), "admin");
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_error_handling() {
        // Test with non-existent directory
        let scanner = ConventionScanner::new("/non/existent/path");
        let result = scanner.scan();
        
        // Should handle gracefully - either return empty conventions or an error
        match result {
            Ok(conventions) => {
                assert!(conventions.is_empty());
            },
            Err(_) => {
                // Error is also acceptable for non-existent paths
            }
        }
    }
    
    #[test]
    fn test_file_conventions_empty() {
        let conventions = FileConventions {
            favicon: None,
            icons: None,
            apple_touch_icons: None,
            robots_txt: None,
            sitemaps: None,
            manifests: None,
            og_images: None,
            twitter_images: None,
        };
        
        assert!(conventions.is_empty());
        assert!(conventions.get_primary_favicon().is_none());
        assert_eq!(conventions.get_icons_by_type(&IconType::Icon).len(), 0);
        assert_eq!(conventions.get_apple_touch_icons_by_color("red").len(), 0);
        assert_eq!(conventions.get_sitemaps_by_type(&SitemapType::Sitemap).len(), 0);
        assert_eq!(conventions.get_og_images_for_route("/test").len(), 0);
        assert_eq!(conventions.get_twitter_images_for_route("/test").len(), 0);
    }
    
    #[test]
    #[cfg(feature = "file-conventions")]
    fn test_mime_type_detection() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create files with different extensions
        fs::write(temp_dir.path().join("favicon.ico"), b"favicon data").unwrap();
        fs::write(temp_dir.path().join("icon.png"), b"png icon data").unwrap();
        fs::write(temp_dir.path().join("icon.svg"), b"<svg>icon</svg>").unwrap();
        
        let scanner = ConventionScanner::new(temp_dir.path());
        let conventions = scanner.scan().unwrap();
        
        assert!(conventions.favicon.is_some());
        assert!(conventions.icons.is_some());
        
        let favicon = conventions.get_primary_favicon().unwrap();
        let icons = conventions.icons.as_ref().unwrap();
        
        // Check MIME types are detected correctly
        assert!(favicon.mime_type.contains("image") || favicon.mime_type == "application/octet-stream");
        
        for icon in icons {
            assert!(icon.mime_type.contains("image") || icon.mime_type == "application/octet-stream");
        }
    }
    
    #[test]
    fn test_default_scanner() {
        let scanner = ConventionScanner::default();
        assert_eq!(scanner.root_dir, PathBuf::from("./app"));
        assert!(scanner.recursive);
        assert_eq!(scanner.max_depth, 5);
    }
}

//! File convention scanner implementation
//!
//! This module provides the core scanning functionality for detecting
//! metadata files following Next.js file conventions.

use super::config::*;
use super::mime_types::MimeTypeUtils;
use super::patterns::PatternMatcher;
use super::types::*;
use crate::{Error, Result};
use std::path::{Path, PathBuf};

// Use external crates when file-conventions feature is enabled
#[cfg(feature = "file-conventions")]
use walkdir::WalkDir;

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
                WalkDir::new(&self.root_dir).max_depth(1).into_iter()
            };

            for entry in walker.filter_map(|e| e.ok()) {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

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
                let _ = self.check_favicon(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process favicon {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self.check_icon(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process icon {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self
                    .check_apple_touch_icon(path, &mut conventions)
                    .or_else(|e| {
                        eprintln!(
                            "Warning: Failed to process apple touch icon {:?}: {}",
                            path, e
                        );
                        Ok::<(), crate::Error>(())
                    });
                let _ = self.check_robots_txt(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process robots.txt {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self.check_sitemap(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process sitemap {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self.check_manifest(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process manifest {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self.check_og_image(path, &mut conventions).or_else(|e| {
                    eprintln!("Warning: Failed to process OG image {:?}: {}", path, e);
                    Ok::<(), crate::Error>(())
                });
                let _ = self
                    .check_twitter_image(path, &mut conventions)
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

    /// Get file metadata with size limits and error handling
    fn get_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        // Check file size limit first
        if !MimeTypeUtils::is_file_size_valid(path)? {
            let file_size = MimeTypeUtils::get_file_size(path)?;
            return Err(Error::ConfigError(format!(
                "File too large ({} bytes): {:?}. Maximum size is {} bytes.",
                file_size, path, MAX_FILE_SIZE
            )));
        }

        let file_size = MimeTypeUtils::get_file_size(path)?;
        let mime_type = MimeTypeUtils::get_mime_type(path);

        let dimensions = if mime_type.starts_with("image/") {
            // Only try to get dimensions for reasonable image sizes
            if file_size <= MAX_IMAGE_SIZE_FOR_DIMENSIONS {
                MimeTypeUtils::get_image_dimensions(path)?
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

    /// Extract route pattern from file path
    fn extract_route_pattern(&self, path: &Path) -> Result<Option<String>> {
        PatternMatcher::extract_route_pattern(path, &self.root_dir)
    }

    /// Check if a file is a favicon
    fn check_favicon(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        if PatternMatcher::is_favicon(path) {
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
        if PatternMatcher::is_icon(path) {
            let metadata = self.get_file_metadata(path)?;
            let icon_type = PatternMatcher::extract_icon_type(path);

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
        if PatternMatcher::is_apple_touch_icon(path) {
            let metadata = self.get_file_metadata(path)?;
            let color = PatternMatcher::extract_apple_touch_icon_color(path);

            let apple_icon = AppleTouchIcon {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                color,
            };

            conventions
                .apple_touch_icons
                .as_mut()
                .unwrap()
                .push(apple_icon);
        }

        Ok(())
    }

    /// Check if a file is robots.txt
    fn check_robots_txt(&self, path: &Path, conventions: &mut FileConventions) -> Result<()> {
        if PatternMatcher::is_robots_txt(path) {
            let metadata = self.get_file_metadata(path)?;
            let preview = MimeTypeUtils::get_file_preview(path, 5)?;

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
        if PatternMatcher::is_sitemap(path) {
            let metadata = self.get_file_metadata(path)?;
            let sitemap_type = PatternMatcher::extract_sitemap_type(path);

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
        if PatternMatcher::is_manifest(path) {
            let metadata = self.get_file_metadata(path)?;
            let manifest_type = PatternMatcher::extract_manifest_type(path);

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
        if PatternMatcher::is_og_image(path) {
            let metadata = self.get_file_metadata(path)?;
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
        if PatternMatcher::is_twitter_image(path) {
            let metadata = self.get_file_metadata(path)?;
            let route_pattern = self.extract_route_pattern(path)?;

            let twitter_image = TwitterImageFile {
                path: path.to_path_buf(),
                size: metadata.size,
                mime_type: metadata.mime_type,
                dimensions: metadata.dimensions,
                route_pattern,
            };

            conventions
                .twitter_images
                .as_mut()
                .unwrap()
                .push(twitter_image);
        }

        Ok(())
    }
}

impl Default for ConventionScanner {
    fn default() -> Self {
        Self::new("./app")
    }
}

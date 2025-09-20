//! MIME type detection and image dimension utilities
//!
//! This module provides utilities for detecting MIME types and
//! extracting image dimensions from files.

use crate::Result;
use std::path::Path;

// Size limits to prevent memory issues
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const MAX_PREVIEW_LINES: usize = 10;

/// MIME type and file metadata utilities
pub struct MimeTypeUtils;

impl MimeTypeUtils {
    /// Get MIME type for a file path
    pub fn get_mime_type(path: &Path) -> String {
        #[cfg(feature = "file-conventions")]
        {
            mime_guess::MimeGuess::from_path(path)
                .first_or_octet_stream()
                .to_string()
        }

        #[cfg(not(feature = "file-conventions"))]
        {
            "application/octet-stream".to_string()
        }
    }

    /// Get image dimensions for a file
    pub fn get_image_dimensions(path: &Path) -> Result<Option<(u32, u32)>> {
        #[cfg(feature = "file-conventions")]
        {
            // Use a timeout to prevent hanging on large or corrupted images
            match std::panic::catch_unwind(|| image::image_dimensions(path)) {
                Ok(Ok((width, height))) => {
                    // Sanity check on dimensions
                    if width > 0 && height > 0 && width <= 50000 && height <= 50000 {
                        Ok(Some((width, height)))
                    } else {
                        Ok(None) // Invalid dimensions
                    }
                }
                Ok(Err(_)) => Ok(None), // Invalid image format
                Err(_) => Ok(None),     // Panic occurred, corrupted image
            }
        }

        #[cfg(not(feature = "file-conventions"))]
        {
            Ok(None)
        }
    }

    /// Get file preview (first few lines) with size limits
    pub fn get_file_preview(path: &Path, lines: usize) -> Result<String> {
        let lines = std::cmp::min(lines, MAX_PREVIEW_LINES);

        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => match e.kind() {
                std::io::ErrorKind::InvalidData => {
                    // File might be binary or invalid UTF-8
                    return Ok("<Binary or invalid UTF-8 content>".to_string());
                }
                _ => return Err(crate::Error::IoError(e)),
            },
        };

        // Limit content size to prevent memory issues
        if content.len() > 64 * 1024 {
            // 64KB limit for preview
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

    /// Check if file size is within limits
    pub fn is_file_size_valid(path: &Path) -> Result<bool> {
        let metadata = std::fs::metadata(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                crate::Error::ConfigError(format!("File not found: {:?}", path))
            }
            std::io::ErrorKind::PermissionDenied => {
                crate::Error::ConfigError(format!("Permission denied: {:?}", path))
            }
            _ => crate::Error::IoError(e),
        })?;

        let file_size = metadata.len();
        Ok(file_size <= MAX_FILE_SIZE)
    }

    /// Get file size
    pub fn get_file_size(path: &Path) -> Result<u64> {
        let metadata = std::fs::metadata(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                crate::Error::ConfigError(format!("File not found: {:?}", path))
            }
            std::io::ErrorKind::PermissionDenied => {
                crate::Error::ConfigError(format!("Permission denied: {:?}", path))
            }
            _ => crate::Error::IoError(e),
        })?;

        Ok(metadata.len())
    }
}

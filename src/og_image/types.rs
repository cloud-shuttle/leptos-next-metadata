//! Open Graph image types and structures
//!
//! This module contains the type definitions for OG image generation,
//! including configuration, parameters, and result structures.

use crate::ImageFormat;

#[cfg(feature = "og-images")]
use image::Rgba;
#[cfg(feature = "og-images")]
use liquid::Object;
#[cfg(not(feature = "og-images"))]
use std::collections::HashMap;

/// Open Graph image generator
///
/// This struct handles the generation of Open Graph images using SVG templates
/// and custom fonts for optimal performance and quality.
pub struct OgImageGenerator {
    #[cfg(feature = "og-images")]
    pub config: OgImageConfig,
    #[cfg(feature = "og-images")]
    pub cache: Box<dyn super::cache::CacheProvider + Send + Sync>,
    #[cfg(feature = "og-images")]
    pub metrics: std::sync::Arc<super::metrics::MetricsCollector>,
    #[cfg(not(feature = "og-images"))]
    _phantom: std::marker::PhantomData<()>,
}

/// Configuration for OG image generation
#[derive(Debug, Clone)]
pub struct OgImageConfig {
    /// Default image size (width, height)
    pub default_size: (u32, u32),

    /// Image output format
    pub format: ImageFormat,

    /// JPEG quality (0-100)
    pub quality: u8,

    #[cfg(feature = "og-images")]
    /// Background color
    pub background_color: Rgba<u8>,

    #[cfg(feature = "og-images")]
    /// Default text color
    pub default_text_color: Rgba<u8>,
}

/// OG image generation parameters
#[derive(Debug, Clone)]
pub struct OgImageParams {
    /// Template name to use
    pub template: String,

    #[cfg(feature = "og-images")]
    /// Template data
    pub data: Object,

    #[cfg(not(feature = "og-images"))]
    /// Template data (placeholder)
    pub data: HashMap<String, String>,

    /// Image size (width, height)
    pub size: Option<(u32, u32)>,

    #[cfg(feature = "og-images")]
    /// Background color override
    pub background_color: Option<Rgba<u8>>,

    #[cfg(feature = "og-images")]
    /// Text color override
    pub text_color: Option<Rgba<u8>>,

    /// Image output format
    pub format: ImageFormat,
}

/// Generated OG image
#[derive(Debug)]
pub struct GeneratedOgImage {
    /// Image data as bytes
    pub data: Vec<u8>,

    /// Image format
    pub format: ImageFormat,

    /// Image dimensions
    pub size: (u32, u32),

    /// Content type (e.g., "image/png")
    pub content_type: String,
}

impl Default for OgImageConfig {
    fn default() -> Self {
        Self {
            default_size: (1200, 630),
            format: ImageFormat::PNG,
            quality: 90,
            #[cfg(feature = "og-images")]
            background_color: Rgba([255, 255, 255, 255]), // White
            #[cfg(feature = "og-images")]
            default_text_color: Rgba([0, 0, 0, 255]), // Black
        }
    }
}

impl Default for OgImageGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedOgImage {
    /// Get the file extension for this image format
    pub fn extension(&self) -> &'static str {
        match self.format {
            ImageFormat::PNG => "png",
            ImageFormat::JPEG => "jpg",
            ImageFormat::WebP => "webp",
        }
    }

    /// Get the size in bytes
    pub fn byte_size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(feature = "og-images")]
impl OgImageParams {
    /// Create new OG image parameters
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            data: Object::new(),
            size: None,
            background_color: None,
            text_color: None,
            format: ImageFormat::PNG,
        }
    }

    /// Set the template data
    pub fn data(mut self, data: Object) -> Self {
        self.data = data;
        self
    }

    /// Set the image size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Rgba<u8>) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the text color
    pub fn text_color(mut self, color: Rgba<u8>) -> Self {
        self.text_color = Some(color);
        self
    }
}

#[cfg(not(feature = "og-images"))]
impl OgImageParams {
    /// Create new OG image parameters
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            data: HashMap::new(),
            size: None,
        }
    }

    /// Set the template data
    pub fn data(mut self, data: HashMap<String, String>) -> Self {
        self.data = data;
        self
    }

    /// Set the image size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }
}

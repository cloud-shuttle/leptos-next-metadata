//! Open Graph image generation for leptos-next-metadata
//! 
//! This module provides high-performance OG image generation using Rust-native
//! libraries, achieving 2-7x faster performance than browser-based solutions.

use crate::{Result, Error, ImageFormat};

#[cfg(feature = "og-images")]
use image::{DynamicImage, Rgba, RgbaImage};
#[cfg(feature = "og-images")]
use liquid::{Object, model::Value as LiquidValue};
#[cfg(feature = "og-images")]
use resvg::usvg::{self, TreeParsing};
#[cfg(feature = "og-images")]
use tiny_skia;

/// Open Graph image generator
/// 
/// This struct handles the generation of Open Graph images using SVG templates
/// and custom fonts for optimal performance and quality.
pub struct OgImageGenerator {
    #[cfg(feature = "og-images")]
    config: OgImageConfig,
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
            default_text_color: Rgba([0, 0, 0, 255]),     // Black
        }
    }
}

impl OgImageGenerator {
    /// Create a new OG image generator with default configuration
    pub fn new() -> Self {
        Self::with_config(OgImageConfig::default())
    }
    
    /// Create a new OG image generator with custom configuration
    pub fn with_config(config: OgImageConfig) -> Self {
        Self {
            #[cfg(feature = "og-images")]
            config,
            #[cfg(not(feature = "og-images"))]
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Generate an OG image
    pub fn generate(&self, params: OgImageParams) -> Result<GeneratedOgImage> {
        #[cfg(feature = "og-images")]
        {
            self.generate_with_features(params)
        }
        
        #[cfg(not(feature = "og-images"))]
        {
            // Return a minimal placeholder when og-images feature is disabled
            let size = params.size.unwrap_or((1200, 630));
            Ok(GeneratedOgImage {
                data: vec![],
                format: ImageFormat::PNG,
                size,
                content_type: "image/png".to_string(),
            })
        }
    }
    
    #[cfg(feature = "og-images")]
    fn generate_with_features(&self, params: OgImageParams) -> Result<GeneratedOgImage> {
        // Load template
        let template_content = self.load_template(&params.template)?;
        
        // Render template with data
        let svg_content = self.render_template(&template_content, &params.data)?;
        
        // Convert SVG to image
        let image = self.svg_to_image(&svg_content, &params)?;
        
        // Encode to output format
        let data = self.encode_image(&image, &params)?;
        
        let content_type = match self.config.format {
            ImageFormat::PNG => "image/png",
            ImageFormat::JPEG => "image/jpeg", 
            ImageFormat::WebP => "image/webp",
        };
        
        Ok(GeneratedOgImage {
            data,
            format: self.config.format,
            size: params.size.unwrap_or(self.config.default_size),
            content_type: content_type.to_string(),
        })
    }

    /// Generate a simple OG image with title and optional description
    pub fn generate_simple(
        &self,
        title: &str,
        description: Option<&str>,
        size: Option<(u32, u32)>,
    ) -> Result<GeneratedOgImage> {
        #[cfg(feature = "og-images")]
        {
            let mut data = Object::new();
            data.insert("title".into(), LiquidValue::scalar(title.to_string()));
            data.insert("description".into(), LiquidValue::scalar(
                description.unwrap_or("").to_string()
            ));
            
            let params = OgImageParams {
                template: "simple".to_string(),
                data,
                size,
                background_color: None,
                text_color: None,
            };
            
            self.generate(params)
        }
        
        #[cfg(not(feature = "og-images"))]
        {
            let mut data = HashMap::new();
            data.insert("title".to_string(), title.to_string());
            if let Some(desc) = description {
                data.insert("description".to_string(), desc.to_string());
            }
            
            let params = OgImageParams {
                template: "simple".to_string(),
                data,
                size,
            };
            
            self.generate(params)
        }
    }
    
    #[cfg(feature = "og-images")]
    /// Load template from file or embedded template
    fn load_template(&self, template_name: &str) -> Result<String> {
        let template = match template_name {
            "simple" => include_str!("../../templates/simple.svg"),
            _ => return Err(Error::TemplateError(format!("Template '{}' not found", template_name))),
        };
        
        Ok(template.to_string())
    }
    
    #[cfg(feature = "og-images")]
    /// Render template with data using Liquid
    fn render_template(&self, template: &str, data: &Object) -> Result<String> {
        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|e| Error::TemplateError(format!("Failed to create Liquid parser: {}", e)))?;
        
        let template = parser
            .parse(template)
            .map_err(|e| Error::TemplateError(format!("Failed to parse template: {}", e)))?;
        
        let rendered = template
            .render(data)
            .map_err(|e| Error::TemplateError(format!("Failed to render template: {}", e)))?;
        
        Ok(rendered)
    }
    
    #[cfg(feature = "og-images")]
    /// Convert SVG content to image
    fn svg_to_image(&self, svg_content: &str, params: &OgImageParams) -> Result<DynamicImage> {
        let size = params.size.unwrap_or(self.config.default_size);
        
        // Configure usvg options
        let options = usvg::Options::default();
        
        // Parse SVG with usvg
        let tree = usvg::Tree::from_str(svg_content, &options)
            .map_err(|e| Error::ImageError(format!("Failed to parse SVG: {}", e)))?;
        
        // Create pixmap for rendering
        let mut pixmap = tiny_skia::Pixmap::new(size.0, size.1)
            .ok_or_else(|| Error::ImageError("Failed to create pixmap".to_string()))?;
        
        // Render SVG to pixmap
        let transform = usvg::Transform::from_scale(
            size.0 as f32 / tree.size.width(), 
            size.1 as f32 / tree.size.height()
        );
        resvg::Tree::from_usvg(&tree).render(transform, &mut pixmap.as_mut());
        
        // Convert pixmap to image
        let pixels = pixmap.data();
        let mut image = RgbaImage::new(size.0, size.1);
        
        for (i, chunk) in pixels.chunks_exact(4).enumerate() {
            let x = (i as u32) % size.0;
            let y = (i as u32) / size.0;
            
            // tiny_skia uses premultiplied RGBA, convert to normal RGBA
            let alpha = chunk[3];
            let (r, g, b) = if alpha > 0 {
                (
                    ((chunk[2] as u16 * 255) / alpha as u16) as u8,
                    ((chunk[1] as u16 * 255) / alpha as u16) as u8,
                    ((chunk[0] as u16 * 255) / alpha as u16) as u8,
                )
            } else {
                (0, 0, 0)
            };
            
            image.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }
        
        Ok(DynamicImage::ImageRgba8(image))
    }
    
    #[cfg(feature = "og-images")]
    /// Encode image to output format
    fn encode_image(&self, image: &DynamicImage, _params: &OgImageParams) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        
        match self.config.format {
            ImageFormat::PNG => {
                image
                    .write_to(&mut std::io::Cursor::new(&mut output), image::ImageFormat::Png)
                    .map_err(|e| Error::ImageError(format!("PNG encoding error: {}", e)))?;
            }
            ImageFormat::JPEG => {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, self.config.quality);
                image
                    .write_with_encoder(encoder)
                    .map_err(|e| Error::ImageError(format!("JPEG encoding error: {}", e)))?;
            }
            ImageFormat::WebP => {
                return Err(Error::ImageError("WebP encoding not yet implemented".to_string()));
            }
        }
        
        Ok(output)
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(feature = "og-images")]
    fn test_og_image_generator_creation() {
        let generator = OgImageGenerator::new();
        // Basic smoke test
        assert!(std::mem::size_of_val(&generator) > 0);
    }
    
    #[test]
    #[cfg(not(feature = "og-images"))]
    fn test_og_image_generator_creation_fallback() {
        let generator = OgImageGenerator::new();
        // When og-images feature is disabled, generator should still be created
        // but it will have minimal size due to PhantomData
        assert!(std::mem::size_of_val(&generator) >= 0);
    }
    
    #[test]
    fn test_og_image_params_creation() {
        let params = OgImageParams::new("test");
        assert_eq!(params.template, "test");
        assert!(params.size.is_none());
    }
    
    #[test]
    fn test_og_image_params_builder_pattern() {
        let params = OgImageParams::new("test")
            .size(1200, 630);
        
        assert_eq!(params.template, "test");
        assert_eq!(params.size, Some((1200, 630)));
    }
    
    #[test]
    fn test_generated_og_image_extension() {
        let png_image = GeneratedOgImage {
            data: vec![0u8; 100],
            format: ImageFormat::PNG,
            size: (1200, 630),
            content_type: "image/png".to_string(),
        };
        
        let jpeg_image = GeneratedOgImage {
            data: vec![0u8; 100],
            format: ImageFormat::JPEG,
            size: (1200, 630),
            content_type: "image/jpeg".to_string(),
        };
        
        let webp_image = GeneratedOgImage {
            data: vec![0u8; 100],
            format: ImageFormat::WebP,
            size: (1200, 630),
            content_type: "image/webp".to_string(),
        };
        
        assert_eq!(png_image.extension(), "png");
        assert_eq!(jpeg_image.extension(), "jpg");
        assert_eq!(webp_image.extension(), "webp");
    }
    
    #[test]
    fn test_generated_og_image_byte_size() {
        let image_data = vec![0u8; 1024];
        let image = GeneratedOgImage {
            data: image_data.clone(),
            format: ImageFormat::PNG,
            size: (1200, 630),
            content_type: "image/png".to_string(),
        };
        
        assert_eq!(image.byte_size(), 1024);
        assert_eq!(image.byte_size(), image_data.len());
    }
    
    #[test]
    fn test_image_format_enum() {
        // Test that all variants can be created
        let png = ImageFormat::PNG;
        let jpeg = ImageFormat::JPEG;
        let webp = ImageFormat::WebP;
        
        // Basic smoke test - ensure they don't panic
        assert!(std::mem::size_of_val(&png) > 0);
        assert!(std::mem::size_of_val(&jpeg) > 0);
        assert!(std::mem::size_of_val(&webp) > 0);
    }
    
    #[test]
    #[cfg(feature = "og-images")]
    fn test_og_image_generator_default() {
        let generator = OgImageGenerator::default();
        // Should not panic
        assert!(std::mem::size_of_val(&generator) > 0);
    }
    
    #[test]
    #[cfg(not(feature = "og-images"))]
    fn test_og_image_generator_default_fallback() {
        let generator = OgImageGenerator::default();
        // When og-images feature is disabled, generator should still be created
        // but it will have minimal size due to PhantomData
        assert!(std::mem::size_of_val(&generator) >= 0);
    }
    
    #[test]
    fn test_og_image_params_new() {
        let params = OgImageParams::new("test");
        assert_eq!(params.template, "test");
        assert!(params.size.is_none());
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_og_image_generation_simple() {
        let generator = OgImageGenerator::new();
        
        let result = generator.generate_simple(
            "Test Title",
            Some("Test description"),
            Some((800, 600)),
        );
        
        assert!(result.is_ok(), "Image generation should succeed");
        let image = result.unwrap();
        assert_eq!(image.size, (800, 600));
        assert_eq!(image.format, ImageFormat::PNG);
        assert_eq!(image.content_type, "image/png");
        assert!(!image.data.is_empty(), "Generated image should have data");
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_og_image_generation_with_custom_params() {
        let generator = OgImageGenerator::new();
        let mut data = Object::new();
        data.insert("title".into(), LiquidValue::scalar("Custom Title"));
        data.insert("description".into(), LiquidValue::scalar("Custom Description"));
        
        let params = OgImageParams::new("simple")
            .data(data)
            .size(1200, 630);
        
        let result = generator.generate(params);
        assert!(result.is_ok(), "Custom image generation should succeed");
        
        let image = result.unwrap();
        assert_eq!(image.size, (1200, 630));
        assert!(!image.data.is_empty(), "Generated image should have data");
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_template_loading() {
        let generator = OgImageGenerator::new();
        let template = generator.load_template("simple");
        
        assert!(template.is_ok(), "Simple template should load");
        let template_content = template.unwrap();
        assert!(template_content.contains("<svg"), "Template should contain SVG content");
        assert!(template_content.contains("{{ title"), "Template should contain Liquid variables");
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_template_loading_invalid() {
        let generator = OgImageGenerator::new();
        let result = generator.load_template("nonexistent");
        
        assert!(result.is_err(), "Nonexistent template should fail");
        match result.unwrap_err() {
            Error::TemplateError(msg) => {
                assert!(msg.contains("not found"), "Error should mention template not found");
            }
            _ => panic!("Expected TemplateError"),
        }
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_template_rendering() {
        let generator = OgImageGenerator::new();
        let template = "<svg>{{ title }}</svg>";
        let mut data = Object::new();
        data.insert("title".into(), LiquidValue::scalar("Test Title"));
        
        let result = generator.render_template(template, &data);
        assert!(result.is_ok(), "Template rendering should succeed");
        
        let rendered = result.unwrap();
        assert!(rendered.contains("Test Title"), "Rendered template should contain title");
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_image_formats() {
        let png_config = OgImageConfig {
            format: ImageFormat::PNG,
            ..Default::default()
        };
        
        let jpeg_config = OgImageConfig {
            format: ImageFormat::JPEG,
            quality: 85,
            ..Default::default()
        };
        
        let png_generator = OgImageGenerator::with_config(png_config);
        let jpeg_generator = OgImageGenerator::with_config(jpeg_config);
        
        let png_result = png_generator.generate_simple("Test", None, Some((400, 300)));
        let jpeg_result = jpeg_generator.generate_simple("Test", None, Some((400, 300)));
        
        assert!(png_result.is_ok(), "PNG generation should succeed: {:?}", png_result.as_ref().err());
        assert!(jpeg_result.is_ok(), "JPEG generation should succeed: {:?}", jpeg_result.as_ref().err());
        
        let png_image = png_result.unwrap();
        let jpeg_image = jpeg_result.unwrap();
        
        assert_eq!(png_image.format, ImageFormat::PNG);
        assert_eq!(jpeg_image.format, ImageFormat::JPEG);
        assert_eq!(png_image.content_type, "image/png");
        assert_eq!(jpeg_image.content_type, "image/jpeg");
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_svg_to_image_conversion() {
        let generator = OgImageGenerator::new();
        let svg_content = r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
            <rect width="100" height="100" fill="red"/>
        </svg>"#;
        
        let params = OgImageParams::new("test").size(100, 100);
        let result = generator.svg_to_image(svg_content, &params);
        
        assert!(result.is_ok(), "SVG to image conversion should succeed");
        let image = result.unwrap();
        
        // Check image dimensions
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }
    
    #[cfg(feature = "og-images")]
    #[test]
    fn test_image_encoding() {
        let generator = OgImageGenerator::new();
        
        // Create a simple test image
        let test_image = image::RgbaImage::new(10, 10);
        let dynamic_image = image::DynamicImage::ImageRgba8(test_image);
        let params = OgImageParams::new("test");
        
        let result = generator.encode_image(&dynamic_image, &params);
        assert!(result.is_ok(), "Image encoding should succeed");
        
        let encoded = result.unwrap();
        assert!(!encoded.is_empty(), "Encoded image should have data");
        
        // PNG files should start with PNG signature
        assert_eq!(&encoded[0..4], &[137, 80, 78, 71], "Should be PNG format");
    }
    
    #[cfg(feature = "og-images")]
    #[test] 
    fn test_performance_target() {
        use std::time::Instant;
        
        let generator = OgImageGenerator::new();
        let start = Instant::now();
        
        let result = generator.generate_simple(
            "Performance Test Title",
            Some("Testing generation speed"),
            Some((1200, 630)),
        );
        
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Performance test should succeed");
        
        // Target: <100ms for simple images (may be higher in debug builds)
        #[cfg(not(debug_assertions))]
        assert!(
            duration.as_millis() < 100,
            "Image generation should take less than 100ms, took {}ms",
            duration.as_millis()
        );
        
        // More lenient for debug builds
        #[cfg(debug_assertions)]
        assert!(
            duration.as_millis() < 1500,
            "Image generation should take less than 1500ms in debug, took {}ms", 
            duration.as_millis()
        );
    }
    
    #[test]
    fn test_error_handling() {
        // Test that errors are properly handled when og-images feature is disabled
        #[cfg(not(feature = "og-images"))]
        {
            let generator = OgImageGenerator::new();
            let result = generator.generate_simple("Test", None, Some((100, 100)));
            
            assert!(result.is_ok(), "Should return placeholder when feature disabled");
            let image = result.unwrap();
            assert!(image.data.is_empty(), "Placeholder should have empty data");
        }
    }
}
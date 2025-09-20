# OG Image Generator Component Design

## Overview
Design specification for the OpenGraph image generation component after refactoring and hardening.

## Current Problems
- Single 751-line file with mixed concerns
- WebP encoding marked "not yet implemented" causing runtime errors
- Insufficient error handling and recovery
- Limited test coverage (74/88 lines = 84%)
- No golden image testing for visual regression

## Design Goals
- **Reliability**: No runtime panics or unimplemented errors
- **Performance**: Sub-500ms generation for typical images
- **Quality**: Consistent visual output across formats
- **Testability**: Golden file testing for visual validation
- **Extensibility**: Plugin system for custom templates

## Architecture

### 1. Generator Core (`generator.rs`)
**Responsibility**: High-level image generation orchestration
**Line Target**: 280 lines

```rust
//! Core image generation logic

use crate::og_image::{config::*, encoder::*, template::*};
use crate::utils::errors::*;

/// Main image generator with caching support
pub struct OgImageGenerator {
    config: GeneratorConfig,
    template_engine: TemplateEngine,
    encoder: ImageEncoder,
    cache: Option<Box<dyn CacheProvider>>,
}

impl OgImageGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self {
            template_engine: TemplateEngine::new(&config.template_config),
            encoder: ImageEncoder::new(&config.encoder_config),
            cache: config.cache_provider.map(|p| p.into_boxed()),
            config,
        }
    }
    
    /// Generate OG image with full error handling
    pub async fn generate(
        &self, 
        params: &OgImageParams
    ) -> Result<GeneratedImage, OgImageError> {
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Ok(cached) = cache.get(&params.cache_key()).await {
                return Ok(cached);
            }
        }
        
        // Render template to SVG
        let svg_content = self.template_engine
            .render(&params.template, &params.data)
            .map_err(OgImageError::TemplateRender)?;
            
        // Convert SVG to target format
        let image_data = self.encoder
            .encode(&svg_content, &params.format, &params.dimensions)
            .await
            .map_err(OgImageError::ImageEncode)?;
            
        let generated = GeneratedImage {
            data: image_data,
            format: params.format,
            dimensions: params.dimensions,
            metadata: ImageMetadata::new(params),
        };
        
        // Cache result
        if let Some(cache) = &self.cache {
            let _ = cache.set(&params.cache_key(), &generated).await;
        }
        
        Ok(generated)
    }
    
    /// Generate with fallback strategy
    pub async fn generate_with_fallback(
        &self,
        params: &OgImageParams
    ) -> Result<GeneratedImage, OgImageError> {
        // Try primary format first
        match self.generate(params).await {
            Ok(image) => Ok(image),
            Err(e) if params.format == ImageFormat::WebP => {
                // Fallback to PNG for WebP failures
                let fallback_params = OgImageParams {
                    format: ImageFormat::Png,
                    ..params.clone()
                };
                self.generate(&fallback_params).await
            }
            Err(e) => Err(e),
        }
    }
}
```

**Key Features**:
- Async generation with proper error handling
- Built-in caching strategy
- Fallback format support for WebP
- Metrics collection points

### 2. Template Engine (`template.rs`)
**Responsibility**: Template rendering and customization
**Line Target**: 250 lines

```rust
//! Template rendering engine

use liquid::Parser;
use std::collections::HashMap;
use crate::utils::errors::*;

pub struct TemplateEngine {
    parser: Parser,
    template_cache: HashMap<String, liquid::Template>,
    config: TemplateConfig,
}

impl TemplateEngine {
    pub fn new(config: &TemplateConfig) -> Self {
        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .expect("Failed to create liquid parser");
            
        Self {
            parser,
            template_cache: HashMap::new(),
            config: config.clone(),
        }
    }
    
    /// Render template with data context
    pub fn render(
        &mut self,
        template_name: &str,
        data: &TemplateData
    ) -> Result<String, TemplateError> {
        // Load template (with caching)
        let template = self.load_template(template_name)?;
        
        // Prepare template context
        let context = self.create_context(data)?;
        
        // Render template
        template.render(&context)
            .map_err(|e| TemplateError::RenderFailed {
                template: template_name.to_string(),
                source: e,
            })
    }
    
    fn load_template(&mut self, name: &str) -> Result<&liquid::Template, TemplateError> {
        if !self.template_cache.contains_key(name) {
            let template_content = self.read_template_file(name)?;
            let template = self.parser.parse(&template_content)
                .map_err(|e| TemplateError::ParseFailed {
                    template: name.to_string(),
                    source: e,
                })?;
            self.template_cache.insert(name.to_string(), template);
        }
        
        Ok(self.template_cache.get(name).unwrap())
    }
    
    fn create_context(&self, data: &TemplateData) -> Result<liquid::Object, TemplateError> {
        let mut context = liquid::object!({
            "title": data.title.clone().unwrap_or_default(),
            "description": data.description.clone().unwrap_or_default(),
        });
        
        // Add dynamic fields
        for (key, value) in &data.custom_fields {
            context.insert(key.clone().into(), value.clone());
        }
        
        // Add theme and styling
        if let Some(theme) = &data.theme {
            context.insert("theme".into(), theme.to_liquid_value());
        }
        
        Ok(context)
    }
}

#[derive(Debug, Clone)]
pub struct TemplateData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub theme: Option<Theme>,
    pub custom_fields: HashMap<String, liquid::model::Value>,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background_color: String,
    pub text_color: String,
    pub font_family: String,
    pub brand_logo: Option<String>,
}
```

### 3. Image Encoder (`encoder.rs`)
**Responsibility**: Format-specific image encoding
**Line Target**: 200 lines

```rust
//! Image format encoding implementations

use image::{DynamicImage, ImageFormat as ImageLibFormat};
use crate::og_image::config::*;
use crate::utils::errors::*;

pub struct ImageEncoder {
    config: EncoderConfig,
}

impl ImageEncoder {
    pub fn new(config: &EncoderConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
    
    /// Encode SVG to target format
    pub async fn encode(
        &self,
        svg_content: &str,
        format: &ImageFormat,
        dimensions: &ImageDimensions,
    ) -> Result<Vec<u8>, EncodeError> {
        // Convert SVG to raster image
        let raster_image = self.svg_to_raster(svg_content, dimensions).await?;
        
        // Encode to target format
        match format {
            ImageFormat::Png => self.encode_png(&raster_image).await,
            ImageFormat::Jpeg => self.encode_jpeg(&raster_image).await,
            ImageFormat::WebP => self.encode_webp(&raster_image).await,
        }
    }
    
    async fn svg_to_raster(
        &self,
        svg_content: &str,
        dimensions: &ImageDimensions,
    ) -> Result<DynamicImage, EncodeError> {
        use resvg::tiny_skia;
        use resvg::usvg;
        
        // Parse SVG
        let opts = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg_content, &opts)
            .map_err(|e| EncodeError::SvgParse { source: e })?;
            
        // Create pixel buffer
        let mut pixmap = tiny_skia::Pixmap::new(dimensions.width, dimensions.height)
            .ok_or(EncodeError::PixmapCreate)?;
            
        // Render SVG to pixel buffer
        resvg::render(&tree, usvg::FitTo::Size(dimensions.width, dimensions.height), pixmap.as_mut());
        
        // Convert to DynamicImage
        let image_buffer = image::RgbaImage::from_raw(
            dimensions.width,
            dimensions.height,
            pixmap.take(),
        ).ok_or(EncodeError::ImageBufferCreate)?;
        
        Ok(DynamicImage::ImageRgba8(image_buffer))
    }
    
    async fn encode_png(&self, image: &DynamicImage) -> Result<Vec<u8>, EncodeError> {
        let mut buffer = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut buffer), ImageLibFormat::Png)
            .map_err(|e| EncodeError::PngEncode { source: e })?;
        Ok(buffer)
    }
    
    async fn encode_jpeg(&self, image: &DynamicImage) -> Result<Vec<u8>, EncodeError> {
        let mut buffer = Vec::new();
        // Convert to RGB first (JPEG doesn't support alpha)
        let rgb_image = image.to_rgb8();
        let dynamic_rgb = DynamicImage::ImageRgb8(rgb_image);
        
        dynamic_rgb.write_to(&mut std::io::Cursor::new(&mut buffer), ImageLibFormat::Jpeg)
            .map_err(|e| EncodeError::JpegEncode { source: e })?;
        Ok(buffer)
    }
    
    async fn encode_webp(&self, image: &DynamicImage) -> Result<Vec<u8>, EncodeError> {
        #[cfg(feature = "webp-support")]
        {
            // WebP encoding implementation using webp crate
            use webp::Encoder;
            
            let rgba = image.to_rgba8();
            let encoder = Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
            let webp_data = encoder.encode(self.config.webp_quality);
            Ok(webp_data.to_vec())
        }
        
        #[cfg(not(feature = "webp-support"))]
        {
            Err(EncodeError::WebPNotSupported)
        }
    }
}
```

### 4. Configuration (`config.rs`)
**Responsibility**: Configuration types and defaults
**Line Target**: 150 lines

```rust
//! Configuration types for OG image generation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub template_config: TemplateConfig,
    pub encoder_config: EncoderConfig,
    pub cache_provider: Option<CacheConfig>,
    pub performance_limits: PerformanceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub template_dir: String,
    pub default_template: String,
    pub cache_templates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct EncoderConfig {
    pub jpeg_quality: u8,
    pub webp_quality: f32,
    pub png_compression: u8,
    pub max_dimensions: ImageDimensions,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    #[cfg(feature = "webp-support")]
    WebP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceLimits {
    pub max_generation_time_ms: u64,
    pub max_template_size_bytes: usize,
    pub max_cache_entries: usize,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            template_config: TemplateConfig::default(),
            encoder_config: EncoderConfig::default(),
            cache_provider: None,
            performance_limits: PerformanceLimits::default(),
        }
    }
}
```

## Testing Strategy

### 1. Golden File Testing
```rust
// tests/golden_images_test.rs
#[tokio::test]
async fn test_basic_template_golden() {
    let generator = OgImageGenerator::default();
    let params = OgImageParams::new()
        .title("Test Title")
        .description("Test Description")
        .template("basic");
        
    let result = generator.generate(&params).await.unwrap();
    
    // Compare with golden file
    let expected = std::fs::read("tests/golden/basic_template.png").unwrap();
    assert_images_similar(&result.data, &expected, 0.95);
}

fn assert_images_similar(actual: &[u8], expected: &[u8], threshold: f32) {
    // Use image comparison library to verify visual similarity
    let actual_img = image::load_from_memory(actual).unwrap();
    let expected_img = image::load_from_memory(expected).unwrap();
    
    let similarity = calculate_structural_similarity(&actual_img, &expected_img);
    assert!(similarity >= threshold, "Image similarity {} below threshold {}", similarity, threshold);
}
```

### 2. Error Path Testing
```rust
#[tokio::test]
async fn test_webp_fallback_when_unavailable() {
    let generator = OgImageGenerator::default();
    let params = OgImageParams::new()
        .format(ImageFormat::WebP);
        
    #[cfg(not(feature = "webp-support"))]
    {
        let result = generator.generate_with_fallback(&params).await.unwrap();
        assert_eq!(result.format, ImageFormat::Png); // Should fallback to PNG
    }
}
```

### 3. Performance Testing
```rust
#[tokio::test]
async fn test_generation_performance() {
    let generator = OgImageGenerator::default();
    let params = OgImageParams::new().title("Performance Test");
    
    let start = std::time::Instant::now();
    let _result = generator.generate(&params).await.unwrap();
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 500, "Generation took too long: {:?}", duration);
}
```

## Migration Plan

### Phase 1: Refactor Core (3 days)
1. Extract configuration types
2. Split encoder logic into separate module
3. Create clean error types
4. Maintain backward compatibility

### Phase 2: Implement WebP Properly (2 days)
1. Add webp-support feature flag
2. Implement proper WebP encoding
3. Add fallback mechanisms
4. Test error paths thoroughly

### Phase 3: Add Golden Testing (2 days)
1. Create golden file test infrastructure
2. Generate reference images for all templates
3. Add visual regression test suite
4. Set up CI golden file validation

### Phase 4: Performance & Caching (2 days)
1. Add performance monitoring
2. Implement caching layer
3. Add timeout protection
4. Benchmark generation performance

## Success Criteria
- [ ] All files under 300 lines
- [ ] WebP properly implemented with feature flag
- [ ] Golden file testing prevents visual regressions
- [ ] Generation time <500ms for typical images
- [ ] Error paths fully tested and handled
- [ ] Zero panic/unwrap in production code
- [ ] Comprehensive fallback strategies

## Performance Targets
- **Cold generation**: <500ms
- **Cached generation**: <10ms
- **Template parsing**: <50ms
- **Memory usage**: <50MB peak per generation
- **Concurrent generations**: Up to 10 parallel

//! Open Graph image generator implementation
//!
//! This module provides the core functionality for generating OG images
//! using SVG templates and image processing.

use super::cache::{CacheKey, CacheProvider, MemoryCache, NoOpCache};
use super::metrics::{MetricsCollector, PerformanceMonitor, TimingGuard};
use super::types::*;
use crate::{ImageFormat, Result};

#[cfg(feature = "og-images")]
use liquid::{model::Value as LiquidValue, Object};
#[cfg(not(feature = "og-images"))]
use std::collections::HashMap;

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
            #[cfg(feature = "og-images")]
            cache: Box::new(MemoryCache::new(100)),
            #[cfg(feature = "og-images")]
            metrics: std::sync::Arc::new(MetricsCollector::new()),
            #[cfg(not(feature = "og-images"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new OG image generator with custom cache
    pub fn with_cache(cache: Box<dyn CacheProvider + Send + Sync>) -> Self {
        Self {
            #[cfg(feature = "og-images")]
            config: OgImageConfig::default(),
            #[cfg(feature = "og-images")]
            cache,
            #[cfg(feature = "og-images")]
            metrics: std::sync::Arc::new(MetricsCollector::new()),
            #[cfg(not(feature = "og-images"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new OG image generator with no caching
    pub fn without_cache() -> Self {
        Self {
            #[cfg(feature = "og-images")]
            config: OgImageConfig::default(),
            #[cfg(feature = "og-images")]
            cache: Box::new(NoOpCache),
            #[cfg(feature = "og-images")]
            metrics: std::sync::Arc::new(MetricsCollector::new()),
            #[cfg(not(feature = "og-images"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Generate an OG image with caching support
    pub async fn generate(&self, params: OgImageParams) -> Result<GeneratedOgImage> {
        #[cfg(feature = "og-images")]
        {
            self.generate_with_features_cached(params).await
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

    /// Generate an OG image without caching (for backward compatibility)
    pub async fn generate_sync(&self, params: OgImageParams) -> Result<GeneratedOgImage> {
        #[cfg(feature = "og-images")]
        {
            self.generate_with_features(params).await
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
    async fn generate_with_features_cached(
        &self,
        params: OgImageParams,
    ) -> Result<GeneratedOgImage> {
        let _timing_guard = TimingGuard::new(self.metrics.clone());

        // Check cache first
        let cache_key = CacheKey::new(&params);
        if let Some(cached_data) = self.cache.get(&cache_key).await? {
            self.metrics.record_cache_hit();
            let content_type = match self.config.format {
                ImageFormat::PNG => "image/png",
                ImageFormat::JPEG => "image/jpeg",
                ImageFormat::WebP => "image/webp",
            };

            return Ok(GeneratedOgImage {
                data: cached_data,
                format: self.config.format,
                size: params.size.unwrap_or(self.config.default_size),
                content_type: content_type.to_string(),
            });
        }

        self.metrics.record_cache_miss();

        // Generate new image
        let result = self.generate_with_features(params.clone()).await?;

        // Cache the result
        let _ = self.cache.set(&cache_key, &result.data).await;

        Ok(result)
    }

    #[cfg(feature = "og-images")]
    async fn generate_with_features(&self, params: OgImageParams) -> Result<GeneratedOgImage> {
        // Load template
        let template_content = self.load_template(&params.template).await?;

        // Render template with data
        let svg_content = self
            .render_template(&template_content, &params.data)
            .await?;

        // Convert SVG to image
        let image = self.svg_to_image(&svg_content, &params).await?;

        // Encode to output format
        let data = self.encode_image(&image, &params).await?;

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
    pub async fn generate_simple(
        &self,
        title: &str,
        description: Option<&str>,
        size: Option<(u32, u32)>,
    ) -> Result<GeneratedOgImage> {
        #[cfg(feature = "og-images")]
        {
            let mut data = Object::new();
            data.insert("title".into(), LiquidValue::scalar(title.to_string()));
            data.insert(
                "description".into(),
                LiquidValue::scalar(description.unwrap_or("").to_string()),
            );

            let params = OgImageParams {
                template: "simple".to_string(),
                data,
                size,
                background_color: None,
                text_color: None,
                format: self.config.format,
            };

            self.generate(params).await
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
                format: ImageFormat::PNG,
            };

            self.generate(params).await
        }
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> Result<super::cache::CacheStats> {
        #[cfg(feature = "og-images")]
        {
            self.cache.stats().await
        }
        #[cfg(not(feature = "og-images"))]
        {
            Ok(super::cache::CacheStats {
                entries: 0,
                max_entries: 0,
                total_accesses: 0,
                average_age: std::time::Duration::ZERO,
                oldest_entry_age: std::time::Duration::ZERO,
                hit_rate: 0.0,
            })
        }
    }

    /// Clear the cache
    pub async fn clear_cache(&self) -> Result<()> {
        #[cfg(feature = "og-images")]
        {
            self.cache.clear().await
        }
        #[cfg(not(feature = "og-images"))]
        {
            Ok(())
        }
    }

    /// Generate with fallback strategy (WebP -> PNG)
    pub async fn generate_with_fallback(
        &self,
        mut params: OgImageParams,
    ) -> Result<GeneratedOgImage> {
        // Try primary format first
        match self.generate(params.clone()).await {
            Ok(image) => Ok(image),
            Err(_e) if params.format == ImageFormat::WebP => {
                // Fallback to PNG for WebP failures
                #[cfg(feature = "og-images")]
                self.metrics.record_webp_fallback();
                params.format = ImageFormat::PNG;
                self.generate(params).await
            }
            Err(e) => {
                #[cfg(feature = "og-images")]
                self.metrics.record_error();
                Err(e)
            }
        }
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> super::metrics::GenerationMetrics {
        #[cfg(feature = "og-images")]
        {
            self.metrics.get_metrics()
        }
        #[cfg(not(feature = "og-images"))]
        {
            super::metrics::GenerationMetrics::default()
        }
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> super::metrics::PerformanceReport {
        #[cfg(feature = "og-images")]
        {
            let monitor = PerformanceMonitor::with_default_limits();
            monitor.get_performance_report()
        }
        #[cfg(not(feature = "og-images"))]
        {
            super::metrics::PerformanceReport {
                metrics: super::metrics::GenerationMetrics::default(),
                limits: super::metrics::PerformanceLimits::default(),
                health_status: super::metrics::HealthStatus::Healthy,
                recommendations: vec![],
            }
        }
    }

    /// Reset performance metrics
    pub fn reset_metrics(&self) {
        #[cfg(feature = "og-images")]
        {
            self.metrics.reset();
        }
    }
}

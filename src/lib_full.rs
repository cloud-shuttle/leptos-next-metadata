//! # leptos-next-metadata
//! 
//! Next.js-style metadata management for Leptos applications with type-safe APIs,
//! blazing-fast OG image generation, and comprehensive SEO optimization.
//! 
//! ## Quick Example
//! 
//! ```rust
//! use leptos::*;
//! use leptos_next_metadata::prelude::*;
//! 
//! #[component]
//! fn MyPage() -> impl IntoView {
//!     metadata! {
//!         title: "My Page",
//!         description: "Page description",
//!     }
//!     
//!     view! { <h1>"My Page"</h1> }
//! }
//! ```
//! 
//! ## Features
//! 
//! - 🚀 **2-7x faster** OG image generation than browser-based solutions
//! - 🦀 **Type-safe** metadata with compile-time validation
//! - 🎯 **Next.js compatible** API for easy migration
//! - 🖼️ **Dynamic OG images** with SVG templates and custom fonts
//! - 📊 **JSON-LD support** with Schema.org types
//! - 🔍 **SEO validation** with best practices enforcement
//! - ⚡ **SSR/CSR/Islands** - works with all Leptos rendering modes
//! - 📁 **File conventions** - automatic favicon, robots.txt, sitemap detection
//! - 🎨 **Template system** - Liquid templates for OG images
//! - 💾 **Smart caching** - multi-level caching for optimal performance
//! 
//! ## Feature Flags
//! 
//! - `ssr` - Server-side rendering support (default)
//! - `csr` - Client-side rendering support  
//! - `hydrate` - Hydration support
//! - `og-images` - Open Graph image generation (default)
//! - `file-conventions` - File-based metadata conventions (default)
//! - `json-ld` - JSON-LD structured data support
//! - `caching` - Advanced caching with LRU and TTL
//! - `http` - HTTP client for external metadata fetching
//! - `debug` - Debug logging and validation
//! 
//! ## Modules
//! 
//! - [`metadata`] - Core metadata types and traits
//! - [`og_image`] - Open Graph image generation
//! - [`json_ld`] - JSON-LD structured data
//! - [`conventions`] - File convention scanning
//! - [`macros`] - Procedural macros for metadata
//! - [`utils`] - Utility functions and helpers

pub mod metadata;
pub mod og_image;
pub mod json_ld;
pub mod conventions;
pub mod macros;
pub mod utils;

/// Re-exports for common use cases
pub mod prelude {
    pub use crate::metadata::{
        Metadata, Title, Description, Keywords, Authors, Robots,
        OpenGraph, Twitter, TwitterCard, Article, Profile,
        AlternateLink, CanonicalUrl, Viewport, ThemeColor,
        ColorScheme, ReferrerPolicy, FormatDetection,
    };
    
    pub use crate::og_image::{GeneratedOgImage, OgImageGenerator, OgImageParams};
    pub use crate::json_ld::{JsonLd, SchemaOrg};
    pub use crate::conventions::{ConventionScanner, FileConventions};
    
    #[cfg(feature = "macros")]
    pub use crate::macros::{metadata, generate_metadata};
    
    pub use crate::metadata::context::{MetadataContext, MetadataProvider, provide_metadata_context};
}

/// Re-export commonly used types
pub use metadata::*;
pub use og_image::*;
pub use json_ld::*;
pub use conventions::*;

#[cfg(feature = "macros")]
pub use macros::*;

/// Result type for metadata operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for metadata operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Metadata context not provided")]
    ContextNotProvided,
    
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("Image generation error: {0}")]
    ImageError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Configuration for the metadata system
#[derive(Debug, Clone)]
pub struct MetadataConfig {
    /// Cache configuration
    pub cache: CacheConfig,
    
    /// OG image generation configuration
    pub og_image: OgImageConfig,
    
    /// File conventions configuration
    pub conventions: ConventionConfig,
    
    /// Debug configuration
    pub debug: DebugConfig,
    
    /// Resource limits
    pub limits: LimitConfig,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable memory caching
    pub memory_cache_enabled: bool,
    
    /// Memory cache size (number of items)
    pub memory_cache_size: usize,
    
    /// Memory cache TTL in seconds
    pub memory_cache_ttl: u64,
    
    /// Enable disk caching
    pub disk_cache_enabled: bool,
    
    /// Disk cache path
    pub disk_cache_path: String,
    
    /// OG image cache size in MB
    pub og_image_cache_size: usize,
}

/// OG image configuration
#[derive(Debug, Clone)]
pub struct OgImageConfig {
    /// Default image dimensions
    pub default_size: (u32, u32),
    
    /// Font configuration
    pub fonts: Vec<FontConfig>,
    
    /// Template directory
    pub template_dir: String,
    
    /// Output format
    pub format: ImageFormat,
    
    /// Quality (for JPEG)
    pub quality: u8,
}

/// Font configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Font family name
    pub family: String,
    
    /// Font weight
    pub weight: FontWeight,
    
    /// Font data
    pub data: Vec<u8>,
}

/// Font weight enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WebP,
}

/// Convention configuration
#[derive(Debug, Clone)]
pub struct ConventionConfig {
    /// Root directory for file conventions
    pub root_dir: String,
    
    /// Enable automatic detection
    pub auto_detect: bool,
    
    /// Custom convention patterns
    pub patterns: Vec<String>,
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Log metadata resolution
    pub log_metadata_resolution: bool,
    
    /// Log cache hits/misses
    pub log_cache_hits: bool,
    
    /// Log generation time
    pub log_generation_time: bool,
    
    /// Validate output
    pub validate_output: bool,
}

/// Resource limits configuration
#[derive(Debug, Clone)]
pub struct LimitConfig {
    /// Maximum OG image size in bytes
    pub max_og_image_size: usize,
    
    /// Maximum template size in bytes
    pub max_template_size: usize,
    
    /// Maximum cache memory in bytes
    pub max_cache_memory: usize,
    
    /// Maximum generation time in milliseconds
    pub max_generation_time: u64,
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            og_image: OgImageConfig::default(),
            conventions: ConventionConfig::default(),
            debug: DebugConfig::default(),
            limits: LimitConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            memory_cache_enabled: true,
            memory_cache_size: 1000,
            memory_cache_ttl: 3600,
            disk_cache_enabled: false,
            disk_cache_path: "./cache".to_string(),
            og_image_cache_size: 100,
        }
    }
}

impl Default for OgImageConfig {
    fn default() -> Self {
        Self {
            default_size: (1200, 630),
            fonts: Vec::new(),
            template_dir: "./templates".to_string(),
            format: ImageFormat::PNG,
            quality: 90,
        }
    }
}

impl Default for ConventionConfig {
    fn default() -> Self {
        Self {
            root_dir: "./app".to_string(),
            auto_detect: true,
            patterns: Vec::new(),
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            log_metadata_resolution: false,
            log_cache_hits: false,
            log_generation_time: false,
            validate_output: false,
        }
    }
}

impl Default for LimitConfig {
    fn default() -> Self {
        Self {
            max_og_image_size: 10_000_000, // 10MB
            max_template_size: 1_000_000,   // 1MB
            max_cache_memory: 100_000_000,  // 100MB
            max_generation_time: 5000,      // 5s
        }
    }
}

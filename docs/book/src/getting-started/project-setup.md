# Project Setup

This guide covers how to configure leptos-next-metadata for different project structures and deployment scenarios.

## Basic Configuration

### Application Root Setup

Set up the metadata context at your application root:

```rust
use leptos::*;
use leptos_next_metadata::*;

#[component]
fn App() -> impl IntoView {
    // Configure global metadata context
    provide_metadata_context();
    
    // Set default metadata for your entire site
    metadata! {
        title: {
            template: "%s | My Site",
            default: "My Site - Welcome"
        },
        description: "My amazing website built with Leptos",
        openGraph: {
            site_name: "My Site",
            locale: "en_US",
            type: "website",
        },
        twitter: {
            site: "@mysite",
        },
        // Default icons that apply to all pages
        icons: {
            icon: "/favicon.ico",
            apple: "/apple-touch-icon.png",
        }
    }
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/blog/*any" view=BlogLayout />
                <Route path="/products/*any" view=ProductLayout />
            </Routes>
        </Router>
    }
}
```

## Server-Side Configuration

### Axum Integration

For Axum-based Leptos applications:

```rust
use axum::{routing::get, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_next_metadata::server::*;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        // Add metadata middleware for server-side metadata processing
        .layer(MetadataMiddleware::new())
        .leptos_routes(&leptos_options, routes, App)
        // Static file serving for metadata assets
        .route("/favicon.ico", get(serve_favicon))
        .route("/robots.txt", get(serve_robots))
        .route("/sitemap.xml", get(serve_sitemap));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Serve generated favicon
async fn serve_favicon() -> impl axum::response::IntoResponse {
    // leptos-next-metadata can generate this from your icon files
    serve_metadata_file("favicon.ico").await
}
```

### Environment Configuration

Create a configuration file for environment-specific settings:

```rust
// src/config.rs
use leptos_next_metadata::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub site_url: String,
    pub site_name: String,
    pub default_locale: String,
    pub twitter_handle: Option<String>,
    pub og_image_generation: OgImageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgImageConfig {
    pub enabled: bool,
    pub default_template: String,
    pub font_families: Vec<String>,
    pub cache_duration: u64,
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            site_url: "https://localhost:3000".to_string(),
            site_name: "My Leptos App".to_string(),
            default_locale: "en_US".to_string(),
            twitter_handle: None,
            og_image_generation: OgImageConfig {
                enabled: true,
                default_template: "default".to_string(),
                font_families: vec!["Inter".to_string()],
                cache_duration: 3600, // 1 hour
            },
        }
    }
}

// Load configuration from environment or file
pub fn load_config() -> MetadataConfig {
    // Try to load from environment variables first
    if let Ok(site_url) = std::env::var("SITE_URL") {
        MetadataConfig {
            site_url,
            ..Default::default()
        }
    } else {
        // Fallback to default configuration
        MetadataConfig::default()
    }
}
```

Use the configuration in your app:

```rust
#[component]
fn App() -> impl IntoView {
    let config = load_config();
    
    provide_metadata_context_with_config(MetadataContextConfig {
        site_url: config.site_url.clone(),
        og_image_config: config.og_image_generation,
        ..Default::default()
    });
    
    metadata! {
        title: {
            template: format!("%s | {}", config.site_name),
            default: config.site_name.clone(),
        },
        openGraph: {
            site_name: config.site_name,
            locale: config.default_locale,
        },
        twitter: {
            site: config.twitter_handle,
        }
    }
    
    // Rest of your app...
}
```

## File Structure Conventions

### Recommended Project Structure

```
src/
├── app/                          # App-level metadata
│   ├── favicon.ico              # Site favicon
│   ├── icon-192.png             # PWA icon
│   ├── icon-512.png             # PWA icon
│   ├── apple-touch-icon.png     # Apple touch icon
│   ├── manifest.json            # PWA manifest
│   ├── robots.txt               # SEO robots file
│   └── sitemap.xml              # XML sitemap
│
├── components/
│   ├── metadata/                # Reusable metadata components
│   │   ├── blog_metadata.rs     # Blog-specific metadata
│   │   ├── product_metadata.rs  # Product metadata
│   │   └── seo_defaults.rs      # Default SEO settings
│   └── og_images/               # OG image templates
│       ├── blog_template.rs     # Blog post OG images  
│       ├── product_template.rs  # Product OG images
│       └── default_template.rs  # Default OG template
│
└── pages/
    ├── blog/
    │   ├── [slug].rs             # Dynamic blog posts
    │   ├── opengraph-image.rs    # Custom OG images for blog
    │   └── twitter-image.rs      # Custom Twitter images
    └── products/
        ├── [id].rs               # Dynamic product pages
        └── opengraph-image.rs    # Product OG images
```

### File Convention Configuration

Configure which file conventions to scan:

```rust
use leptos_next_metadata::conventions::*;

#[component]
fn App() -> impl IntoView {
    provide_metadata_context_with_conventions(
        ConventionConfig {
            // Scan these directories for metadata files
            scan_paths: vec![
                "src/app".to_string(),
                "src/pages".to_string(),
                "public".to_string(),
            ],
            
            // Enable specific conventions
            enabled_conventions: vec![
                Convention::Favicon,
                Convention::Icons,
                Convention::OpenGraphImage,
                Convention::TwitterImage,
                Convention::Robots,
                Convention::Sitemap,
                Convention::Manifest,
            ],
            
            // Custom convention patterns
            custom_patterns: vec![
                ConventionPattern {
                    pattern: "**/*-og.{png,jpg,svg}".to_string(),
                    handler: Box::new(CustomOgImageHandler),
                }
            ],
        }
    );
}
```

## Development vs Production

### Development Configuration

```rust
#[cfg(debug_assertions)]
fn setup_development() -> MetadataContextConfig {
    MetadataContextConfig {
        // Enable validation and helpful warnings in development
        validation_enabled: true,
        warn_on_missing_meta: true,
        warn_on_long_descriptions: true,
        
        // Hot reload metadata files during development
        hot_reload: true,
        
        // Use local URLs for development
        site_url: "http://localhost:3000".to_string(),
        
        // Disable image generation caching for faster iteration
        og_image_config: OgImageConfig {
            cache_enabled: false,
            ..Default::default()
        },
    }
}
```

### Production Configuration

```rust
#[cfg(not(debug_assertions))]
fn setup_production() -> MetadataContextConfig {
    MetadataContextConfig {
        // Disable validation warnings in production
        validation_enabled: false,
        warn_on_missing_meta: false,
        
        // Use production URLs
        site_url: std::env::var("SITE_URL")
            .unwrap_or_else(|_| "https://mysite.com".to_string()),
        
        // Enable aggressive caching in production
        og_image_config: OgImageConfig {
            cache_enabled: true,
            cache_duration: 24 * 60 * 60, // 24 hours
            ..Default::default()
        },
    }
}
```

## Build Configuration

### Cargo Features for Different Environments

```toml
# Cargo.toml
[features]
default = ["ssr", "og-images", "file-conventions"]

# Development features
dev = ["validation", "hot-reload", "dev-tools"]

# Production features  
prod = ["ssr", "og-images", "file-conventions", "compression"]

# Minimal build (just basic metadata)
minimal = []

# Full build (all features)
full = [
    "ssr", "og-images", "file-conventions", 
    "json-ld", "validation", "templates", "i18n"
]
```

Use different features for different builds:

```bash
# Development build
cargo build --features dev

# Production build
cargo build --release --features prod

# Minimal build for size-constrained environments
cargo build --release --features minimal
```

### Build Scripts

Create a build script to process metadata files at compile time:

```rust
// build.rs
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/app");
    println!("cargo:rerun-if-changed=public");
    
    // Process metadata files during build
    let metadata_files = scan_metadata_files("src/app").unwrap();
    generate_metadata_manifest(metadata_files).unwrap();
    
    // Generate optimized OG image templates
    if cfg!(feature = "og-images") {
        compile_og_templates("src/components/og_images").unwrap();
    }
}

fn scan_metadata_files(path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    // Implementation to scan and collect metadata files
    todo!()
}
```

## Testing Configuration

### Unit Testing Setup

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_next_metadata::testing::*;
    
    #[test]
    fn test_homepage_metadata() {
        let runtime = create_runtime();
        
        run_scope(runtime, |cx| {
            // Setup test metadata context
            provide_test_metadata_context();
            
            let view = view! { <HomePage /> };
            
            // Test that metadata was set correctly
            let metadata = get_current_metadata();
            assert_eq!(metadata.title, Some("Welcome to My Site".to_string()));
            assert!(metadata.description.is_some());
        });
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_og_image_generation() {
    let config = MetadataContextConfig {
        og_image_config: OgImageConfig {
            enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let generator = setup_test_og_generator(config).await;
    
    let params = OgImageParams {
        title: "Test Title".to_string(),
        description: "Test Description".to_string(),
        template: "default".to_string(),
    };
    
    let image_bytes = generator.generate(params).await.unwrap();
    assert!(!image_bytes.is_empty());
    assert_eq!(&image_bytes[0..8], b"\x89PNG\r\n\x1a\n"); // PNG header
}
```

## Deployment Considerations

### CDN Configuration

Configure your CDN to serve metadata assets efficiently:

```rust
// Configure cache headers for metadata assets
fn metadata_cache_headers() -> Vec<(String, String)> {
    vec![
        // Cache icons for 1 year
        ("Cache-Control".to_string(), "public, max-age=31536000".to_string()),
        ("Content-Type".to_string(), "image/x-icon".to_string()),
    ]
}

// Serve favicon with proper headers
async fn serve_favicon() -> impl axum::response::IntoResponse {
    use axum::response::Response;
    use http::header::{CACHE_CONTROL, CONTENT_TYPE};
    
    let favicon_data = include_bytes!("../public/favicon.ico");
    
    Response::builder()
        .header(CONTENT_TYPE, "image/x-icon")
        .header(CACHE_CONTROL, "public, max-age=31536000")
        .body(favicon_data.as_slice().into())
        .unwrap()
}
```

### Performance Optimization

```rust
// Pre-generate OG images at build time for static content
#[cfg(feature = "build-time-og-generation")]
fn prebuild_og_images() {
    let static_pages = vec![
        ("Home", "Welcome to our site", "/"),
        ("About", "Learn about our company", "/about"),
        ("Contact", "Get in touch with us", "/contact"),
    ];
    
    for (title, description, path) in static_pages {
        let params = OgImageParams {
            title: title.to_string(),
            description: description.to_string(),
            path: path.to_string(),
        };
        
        // Generate and save to public directory
        generate_and_save_og_image(params).unwrap();
    }
}
```

---

**Next Steps:**

- [Static Metadata Guide](../guides/static-metadata.md) - Deep dive into static metadata patterns
- [Dynamic Metadata Guide](../guides/dynamic-metadata.md) - Advanced dynamic metadata techniques  
- [Performance Guide](../advanced/performance.md) - Optimization strategies and best practices
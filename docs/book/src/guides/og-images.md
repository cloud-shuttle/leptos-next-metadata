# Open Graph Images

Generate beautiful, performant Open Graph images with leptos-next-metadata's built-in SVG-to-PNG rendering engine.

## Why Built-in OG Image Generation?

Traditional OG image generation solutions use headless browsers (Puppeteer, Playwright) which are:
- **Slow**: 800ms+ generation times
- **Resource Heavy**: High memory usage
- **Complex**: Browser dependencies and setup

leptos-next-metadata uses **resvg + tiny-skia** for:
- **Fast**: ~100ms generation times (7x faster)
- **Lightweight**: No browser dependencies
- **Simple**: Pure Rust implementation

## Quick Start

### Basic OG Image

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn BlogPost() -> impl IntoView {
    metadata! {
        title: "My Blog Post",
        description: "An interesting article about Rust and web development",
        openGraph: {
            images: [{
                // This will auto-generate an OG image
                url: og_image! {
                    title: "My Blog Post",
                    description: "An interesting article about Rust and web development",
                    template: "blog"  // Use built-in blog template
                },
                width: 1200,
                height: 630,
                alt: "Blog post cover image"
            }]
        }
    }
    
    view! {
        <article>
            <h1>"My Blog Post"</h1>
            <p>"Article content..."</p>
        </article>
    }
}
```

### Dynamic OG Images

For dynamic content, combine with data loading:

```rust
#[component]
fn DynamicBlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    generate_metadata! {
        async |params, parent| {
            let post = fetch_blog_post(&params.slug).await?;
            
            Metadata {
                title: Some(Title::Static(post.title.clone())),
                description: Some(post.excerpt.clone()),
                openGraph: Some(OpenGraph {
                    images: vec![OgImage {
                        // Generate OG image with post data
                        url: generate_og_image! {
                            template: "blog",
                            data: {
                                "title": post.title,
                                "author": post.author.name,
                                "date": post.published_at.format("%B %d, %Y"),
                                "category": post.category.name,
                                "reading_time": format!("{} min read", post.reading_time),
                                "image": post.featured_image.unwrap_or_default(),
                            }
                        }.await?,
                        width: Some(1200),
                        height: Some(630),
                        alt: Some(format!("Cover image for: {}", post.title)),
                    }],
                    ..Default::default()
                }),
                ..parent.await
            }
        }
    }
    
    // Component implementation...
}
```

## Built-in Templates

### Default Template

Simple, clean design suitable for most content:

```rust
og_image! {
    template: "default",
    data: {
        "title": "Page Title",
        "description": "Page description",
        "logo": "/logo.png"  // Optional
    }
}
```

### Blog Template

Optimized for blog posts and articles:

```rust
og_image! {
    template: "blog", 
    data: {
        "title": "Article Title",
        "author": "Author Name",
        "date": "March 15, 2024",
        "category": "Technology",
        "reading_time": "5 min read",
        "author_image": "/author-avatar.jpg"  // Optional
    }
}
```

### Product Template

Perfect for e-commerce and product pages:

```rust
og_image! {
    template: "product",
    data: {
        "name": "Product Name",
        "price": "$99.99", 
        "rating": "4.8",
        "reviews": "124 reviews",
        "image": "/product-image.jpg",
        "brand": "Brand Name"
    }
}
```

### Event Template

For events, webinars, and announcements:

```rust
og_image! {
    template: "event",
    data: {
        "title": "Event Name",
        "date": "April 20, 2024",
        "time": "2:00 PM EST", 
        "location": "Virtual Event",
        "speakers": ["John Doe", "Jane Smith"],
        "image": "/event-background.jpg"
    }
}
```

## Custom Templates

### Creating Custom Templates

Define your own SVG templates with Liquid templating:

```rust
// src/og_templates/custom.svg
const CUSTOM_TEMPLATE: &str = r#"
<svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:{{ background_start | default: '#667eea' }}" />
      <stop offset="100%" style="stop-color:{{ background_end | default: '#764ba2' }}" />
    </linearGradient>
  </defs>
  
  <!-- Background -->
  <rect width="1200" height="630" fill="url(#bg)" />
  
  <!-- Title -->
  <text x="80" y="200" 
        font-family="Inter, Arial, sans-serif" 
        font-size="72" 
        font-weight="bold" 
        fill="white"
        text-anchor="start">
    {{ title | truncate: 30 }}
  </text>
  
  <!-- Description -->
  <text x="80" y="280" 
        font-family="Inter, Arial, sans-serif" 
        font-size="32" 
        fill="rgba(255,255,255,0.9)"
        text-anchor="start">
    {{ description | truncate: 60 }}
  </text>
  
  <!-- Author info -->
  {% if author %}
  <text x="80" y="500" 
        font-family="Inter, Arial, sans-serif" 
        font-size="24" 
        fill="rgba(255,255,255,0.8)"
        text-anchor="start">
    By {{ author }}
  </text>
  {% endif %}
  
  <!-- Logo -->
  {% if logo %}
  <image href="{{ logo }}" x="80" y="80" width="120" height="40" />
  {% endif %}
</svg>
"#;

// Register your custom template
register_og_template! {
    name: "custom",
    template: CUSTOM_TEMPLATE,
    fonts: ["Inter"],
}
```

### Using Custom Templates

```rust
og_image! {
    template: "custom",
    data: {
        "title": "Custom Design",
        "description": "Beautiful custom OG image",
        "author": "John Doe", 
        "logo": "/logo.svg",
        "background_start": "#ff6b6b",
        "background_end": "#4ecdc4"
    }
}
```

### Advanced Template Features

#### Conditional Content

```svg
<!-- Only show if author exists -->
{% if author %}
<text x="80" y="500" 
      font-family="Inter, Arial, sans-serif" 
      font-size="24" 
      fill="white">
  By {{ author }}
</text>
{% endif %}

<!-- Show different layouts based on content -->
{% if image %}
  <!-- Layout with image -->
  <image href="{{ image }}" x="600" y="100" width="500" height="300" />
  <text x="80" y="200">{{ title }}</text>
{% else %}
  <!-- Layout without image - center title -->
  <text x="600" y="315" text-anchor="middle">{{ title }}</text>
{% endif %}
```

#### Loops and Arrays

```svg
<!-- Display multiple speakers -->
{% for speaker in speakers limit: 3 %}
<text x="80" y="{{ 400 | plus: forloop.index0 | times: 40 }}"
      font-family="Inter"
      font-size="20"
      fill="white">
  • {{ speaker }}
</text>
{% endfor %}
```

#### Text Formatting

```svg
<!-- Truncate long text -->
<text>{{ title | truncate: 40 }}</text>

<!-- Uppercase text -->
<text>{{ category | upcase }}</text>

<!-- Format dates -->
<text>{{ date | date: "%B %d, %Y" }}</text>

<!-- Format numbers -->
<text>${{ price | round: 2 }}</text>
```

## Font Management

### Using Custom Fonts

```rust
use leptos_next_metadata::og_image::fonts::*;

// Register fonts for OG image generation
register_fonts! {
    "Inter" => include_bytes!("../assets/fonts/Inter-Regular.ttf"),
    "Inter-Bold" => include_bytes!("../assets/fonts/Inter-Bold.ttf"), 
    "Roboto" => include_bytes!("../assets/fonts/Roboto-Regular.ttf"),
}

// Use in templates
og_image! {
    template: "custom",
    fonts: ["Inter", "Inter-Bold"],
    data: {
        "title": "Custom Typography",
        "description": "Beautiful fonts in OG images"
    }
}
```

### Font Loading Strategies

```rust
// Load fonts at build time (recommended)
const FONTS: &[(&str, &[u8])] = &[
    ("Inter", include_bytes!("../fonts/Inter.ttf")),
    ("PlayfairDisplay", include_bytes!("../fonts/PlayfairDisplay.ttf")),
];

// Load fonts at runtime (for dynamic fonts)
async fn load_runtime_fonts() -> FontDatabase {
    let mut db = FontDatabase::new();
    
    // Load from file system
    db.load_font_file("./fonts/CustomFont.ttf")?;
    
    // Load from URL (careful with performance)
    let font_data = reqwest::get("https://fonts.example.com/font.ttf")
        .await?
        .bytes()
        .await?;
    db.load_font_data(font_data);
    
    Ok(db)
}
```

### Fallback Fonts

```svg
<text font-family="Inter, Arial, Helvetica, sans-serif">
  {{ title }}
</text>
```

## Performance Optimization

### Caching Strategy

```rust
use leptos_next_metadata::og_image::cache::*;

// Configure caching
configure_og_cache! {
    // In-memory cache for hot images
    memory: {
        max_size: 100,  // Cache 100 images in memory
        ttl: 3600,      // 1 hour TTL
    },
    
    // Disk cache for persistence
    disk: {
        path: "./cache/og-images",
        max_size: "500MB",
        ttl: 86400,  // 24 hours
    },
    
    // CDN cache headers
    headers: {
        "Cache-Control": "public, max-age=86400",
        "ETag": true,  // Generate ETags for better caching
    }
}
```

### Generation Optimization

```rust
// Batch generate similar images
batch_generate_og_images! {
    template: "blog",
    variations: [
        { "title": "Post 1", "author": "John" },
        { "title": "Post 2", "author": "Jane" },
        { "title": "Post 3", "author": "Bob" },
    ]
}

// Pre-generate static images at build time
#[cfg(feature = "build-time-generation")]
prebuild_og_images! {
    ("home", "default", { "title": "Welcome" }),
    ("about", "default", { "title": "About Us" }),
    ("contact", "default", { "title": "Contact" }),
}
```

### Size Optimization

```rust
// Optimize image output
og_image! {
    template: "default", 
    data: { "title": "Optimized Image" },
    
    // Output options
    format: "png",  // or "jpeg" for smaller sizes
    quality: 85,    // JPEG quality (1-100)
    compression: 9, // PNG compression (0-9)
    
    // Size constraints  
    max_size: "500KB",  // Maximum file size
}
```

## File-Based OG Images

### Convention-Based Generation

Place OG image files in your route directories:

```
src/pages/
├── blog/
│   ├── [slug].rs
│   └── opengraph-image.rs     # Dynamic OG generation
├── products/
│   ├── [id].rs  
│   └── opengraph-image.png    # Static OG image
└── about/
    ├── page.rs
    └── twitter-image.rs       # Twitter-specific image
```

### Dynamic OG Image Files

Create `opengraph-image.rs` files for dynamic generation:

```rust
// src/pages/blog/opengraph-image.rs
use leptos_next_metadata::og_image::*;

#[og_image]
pub async fn generate(params: RouteParams) -> OgImageResponse {
    let post = fetch_blog_post(&params.slug).await?;
    
    OgImageResponse {
        template: "blog".to_string(),
        data: liquid::object!({
            "title" => post.title,
            "author" => post.author.name,
            "date" => post.published_at.format("%B %d, %Y").to_string(),
            "category" => post.category.name,
        }),
        cache_key: format!("blog_{}", params.slug),
        cache_duration: 86400, // 24 hours
    }
}
```

## Testing OG Images

### Visual Testing

```rust
#[tokio::test]
async fn test_og_image_generation() {
    let generator = create_test_og_generator().await;
    
    let params = OgImageParams {
        template: "blog".to_string(),
        data: liquid::object!({
            "title" => "Test Article",
            "author" => "Test Author",
        }),
    };
    
    let image_data = generator.generate(params).await.unwrap();
    
    // Verify PNG header
    assert_eq!(&image_data[0..8], b"\x89PNG\r\n\x1a\n");
    
    // Verify dimensions
    let image = image::load_from_memory(&image_data).unwrap();
    assert_eq!(image.width(), 1200);
    assert_eq!(image.height(), 630);
    
    // Optional: Save for visual inspection
    #[cfg(feature = "visual-testing")]
    {
        std::fs::write("test_output/og_image.png", &image_data).unwrap();
    }
}
```

### Benchmark Testing

```rust
#[tokio::test]
async fn benchmark_og_generation() {
    let generator = create_test_og_generator().await;
    let params = create_test_params();
    
    let start = std::time::Instant::now();
    let _image = generator.generate(params).await.unwrap();
    let duration = start.elapsed();
    
    // Should be under 200ms for complex templates
    assert!(duration.as_millis() < 200);
}
```

## Troubleshooting

### Common Issues

**Font not found:**
```rust
// ❌ Font not registered
font-family="UnknownFont"

// ✅ Use registered fonts with fallbacks
font-family="Inter, Arial, sans-serif"
```

**SVG parsing errors:**
```rust
// ❌ Invalid SVG syntax
<text x="invalid">Title</text>

// ✅ Valid SVG attributes
<text x="80" y="100">Title</text>
```

**Performance issues:**
```rust
// ❌ No caching
og_image! { template: "complex", data: complex_data }

// ✅ Enable caching
og_image! { 
    template: "complex", 
    data: complex_data,
    cache_key: "unique_key",
    cache_duration: 3600 
}
```

### Debug Mode

```rust
#[cfg(debug_assertions)]
configure_og_debug! {
    save_intermediate_svg: true,  // Save SVG files for inspection
    log_generation_time: true,    // Log performance metrics
    validate_templates: true,     // Validate template syntax
}
```

---

**Next Steps:**
- [JSON-LD Guide](json-ld.md) - Add structured data to your pages
- [File Conventions](file-conventions.md) - Automatic metadata detection
- [Advanced Performance](../advanced/performance.md) - Optimization strategies
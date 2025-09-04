# 🚀 Quick Start Guide

> **Navigation**: [📚 Documentation Index](../index.md) | [📋 Design Document](design.md) | [🔧 API Reference](../api/core.md)

Get up and running with `leptos-next-metadata` in under 5 minutes! This guide will walk you through the essential setup and basic usage.

---

## ⚡ **Installation**

### **1. Add to Your Project**

```bash
cargo add leptos-next-metadata
```

### **2. Enable Features (Optional)**

```bash
cargo add leptos-next-metadata --features "og-images,json-ld,file-conventions"
```

**Available Features:**
- `og-images` - Open Graph image generation
- `json-ld` - Structured data support
- `file-conventions` - File-based metadata scanning
- `caching` - Advanced caching strategies

---

## 🎯 **Basic Usage**

### **Simple Page with Metadata**

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn MyPage() -> impl IntoView {
    metadata! {
        title: "My Awesome Page",
        description: "This is a fantastic page with great content",
        keywords: ["rust", "leptos", "metadata", "seo"],
        og_type: "website",
        og_image: "/og-image.jpg",
    }
    
    view! { 
        <div>
            <h1>"Welcome to My Page"</h1>
            <p>"This page has automatic metadata generation!"</p>
        </div>
    }
}
```

### **Dynamic Metadata Generation**

```rust
use leptos_next_metadata_macros::generate_metadata;

#[generate_metadata]
fn get_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Dynamic Page".into())),
        description: Some("Generated description".into()),
        og_type: Some("article".into()),
        ..Default::default()
    }
}

#[component]
fn DynamicPage() -> impl IntoView {
    view! { <div>"Dynamic metadata page"</div> }
}
```

---

## 🔧 **Core Concepts**

### **1. Metadata Structure**

The `Metadata` struct contains all your page's metadata:

```rust
use leptos_next_metadata::metadata::Metadata;

let metadata = Metadata {
    title: Some(Title::Static("Page Title".into())),
    description: Some("Page description".into()),
    keywords: Some(vec!["rust".into(), "leptos".into()]),
    og_type: Some("website".into()),
    og_image: Some("/og-image.jpg".into()),
    twitter_card: Some(TwitterCard::SummaryLargeImage),
    ..Default::default()
};
```

### **2. Title Types**

```rust
use leptos_next_metadata::metadata::Title;

// Static title
let static_title = Title::Static("My Page".into());

// Dynamic title with signal
let dynamic_title = Title::Dynamic(create_signal("Dynamic Title".into()).0);

// Template title
let template_title = Title::Template("{} - Site Name".into());
```

### **3. Open Graph Support**

```rust
use leptos_next_metadata::metadata::OpenGraph;

let og = OpenGraph {
    title: Some("OG Title".into()),
    description: Some("OG Description".into()),
    image: Some("/og-image.jpg".into()),
    url: Some("https://example.com/page".into()),
    ..Default::default()
};
```

---

## 📱 **Open Graph Images**

### **Basic OG Image Generation**

```rust
use leptos_next_metadata::og_image::OgImageGenerator;

let generator = OgImageGenerator::new()?;
let image_bytes = generator.generate_og_image(
    "My Page Title",
    "This is a description for the OG image",
    &OgImageParams::default()
)?;
```

### **Custom OG Image Templates**

```rust
let params = OgImageParams::builder()
    .width(1200)
    .height(630)
    .background_color("#1f2937")
    .text_color("#ffffff")
    .font_size(48)
    .build()?;

let image_bytes = generator.generate_og_image(
    "Custom Template",
    "With custom styling",
    &params
)?;
```

---

## 🏷️ **JSON-LD Structured Data**

### **Article Schema**

```rust
use leptos_next_metadata::json_ld::{Article, SchemaOrg};

let article = Article {
    headline: "Article Title".into(),
    description: "Article description".into(),
    author: Some("Author Name".into()),
    date_published: Some("2025-09-04".into()),
    ..Default::default()
};

let schema = SchemaOrg::Article(article);
let json_ld = schema.to_json_ld()?;
```

### **Organization Schema**

```rust
use leptos_next_metadata::json_ld::{Organization, SchemaOrg};

let org = Organization {
    name: "Company Name".into(),
    url: Some("https://company.com".into()),
    logo: Some("https://company.com/logo.png".into()),
    ..Default::default()
};

let schema = SchemaOrg::Organization(org);
```

---

## 📁 **File Conventions**

### **Automatic Asset Detection**

```rust
use leptos_next_metadata::conventions::ConventionScanner;

let scanner = ConventionScanner::new("./app");
let conventions = scanner.scan()?;

if let Some(favicon) = conventions.favicon {
    println!("Found favicon: {:?}", favicon);
}

if let Some(manifest) = conventions.manifest {
    println!("Found manifest: {:?}", manifest);
}
```

**Supported Conventions:**
- `favicon.ico` - Website favicon
- `apple-touch-icon.png` - Apple device icon
- `manifest.json` - Web app manifest
- `robots.txt` - Search engine directives
- `sitemap.xml` - Site structure

---

## 🚀 **Advanced Features**

### **Metadata Merging**

```rust
use leptos_next_metadata::metadata::merge_metadata;

let base_metadata = Metadata {
    title: Some(Title::Template("{} - Site Name".into())),
    description: Some("Default description".into()),
    ..Default::default()
};

let page_metadata = Metadata {
    title: Some(Title::Static("Page Title".into())),
    og_image: Some("/page-image.jpg".into()),
    ..Default::default()
};

let merged = merge_metadata(&base_metadata, &page_metadata)?;
// Result: "Page Title - Site Name" with page-specific OG image
```

### **Caching Strategies**

```rust
use leptos_next_metadata::utils::cache::MetadataCache;

let cache = MetadataCache::new(1000); // 1000 entries
cache.set("page_key", metadata.clone(), Duration::from_secs(3600));

if let Some(cached) = cache.get("page_key") {
    // Use cached metadata
}
```

---

## 🧪 **Testing Your Setup**

### **1. Verify Installation**

```bash
cargo check
```

### **2. Run Examples**

```bash
cargo run --example basic
cargo run --example dynamic_metadata
```

### **3. Run Tests**

```bash
cargo test
```

### **4. Generate Documentation**

```bash
cargo doc --open
```

---

## 🔍 **Common Patterns**

### **Blog Post Metadata**

```rust
metadata! {
    title: "Blog Post Title",
    description: "Blog post description",
    og_type: "article",
    og_image: "/blog-post-image.jpg",
    twitter_card: "summary_large_image",
    keywords: ["blog", "rust", "leptos"],
    author: "Author Name",
    date_published: "2025-09-04",
}
```

### **Product Page Metadata**

```rust
metadata! {
    title: "Product Name",
    description: "Product description",
    og_type: "product",
    og_image: "/product-image.jpg",
    keywords: ["product", "category", "brand"],
    price: "29.99",
    currency: "USD",
}
```

### **Landing Page Metadata**

```rust
metadata! {
    title: "Welcome to Our Site",
    description: "The best solution for your needs",
    og_type: "website",
    og_image: "/hero-image.jpg",
    keywords: ["solution", "service", "company"],
    canonical_url: "https://example.com",
}
```

---

## 🚨 **Troubleshooting**

### **Common Issues**

**1. Compilation Errors**
```bash
# Ensure you have the right Leptos version
cargo add leptos@0.8
```

**2. Missing Features**
```bash
# Enable required features
cargo add leptos-next-metadata --features "og-images,json-ld"
```

**3. Runtime Errors**
```rust
// Use proper error handling
let metadata = metadata! {
    title: "My Page",
    // ... other fields
}.map_err(|e| eprintln!("Metadata error: {}", e))?;
```

### **Getting Help**

- 📚 [Documentation Index](../index.md)
- 🐛 [GitHub Issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- 💬 [GitHub Discussions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- 📖 [API Reference](../api/core.md)

---

## 🎯 **Next Steps**

Now that you're up and running:

1. **Explore Examples**: Check out the [examples directory](../../examples/) for more use cases
2. **Read the API**: Dive into the [API reference](../api/core.md) for detailed documentation
3. **Learn Advanced Features**: Discover [OG image generation](../api/og-image.md) and [JSON-LD](../api/json-ld.md)
4. **Contribute**: Help improve the project by [contributing](../../CONTRIBUTING.md)

---

## 📊 **Performance Tips**

- **Use Static Metadata**: When possible, use `Title::Static` for better performance
- **Enable Caching**: Use the `caching` feature for production applications
- **Optimize Images**: Keep OG images under 1MB for fast loading
- **Lazy Load**: Generate metadata on-demand for dynamic content

---

**🎉 Congratulations!** You're now ready to build amazing Leptos applications with professional metadata management.

---

*Last Updated: September 4, 2025*  
*Next: [API Reference](../api/core.md) | [Examples](../../examples/)*

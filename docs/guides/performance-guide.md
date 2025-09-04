# ⚡ Performance Guide

> **Navigation**: [📚 Documentation Index](../index.md) | [🚀 Quick Start](getting-started.md) | [📋 Production Roadmap](PRODUCTION_ROADMAP.md)

## 📖 **Overview**

This guide covers performance optimization techniques for `leptos-next-metadata`. Learn how to achieve maximum performance while maintaining code quality and developer experience.

---

## 🎯 **Performance Targets**

### **Current Performance Status**

| Metric | Target | Current | Status |
|--------|--------|---------|---------|
| **Metadata Merge** | <10μs | ✅ | **Met** |
| **OG Image Generation** | <100ms | ✅ | **Met** |
| **JSON-LD Serialization** | <5μs | ✅ | **Met** |
| **Template Rendering** | <50μs | ✅ | **Met** |
| **Memory Usage** | <50MB | 🔄 | **In Progress** |

---

## 🚀 **Core Performance Optimizations**

### **1. Static vs Dynamic Metadata**

**Static Metadata** (Recommended for most cases):
```rust
// ✅ Fast - compiled at build time
metadata! {
    title: "My Page",
    description: "Page description",
    og_type: "website",
}
```

**Dynamic Metadata** (Use only when necessary):
```rust
// ⚠️ Slower - runtime generation
generate_metadata! {
    move || async move {
        let data = fetch_data().await;
        Metadata {
            title: Some(data.title),
            description: Some(data.description),
            ..Default::default()
        }
    }
}
```

**Performance Impact:**
- **Static**: ~1μs (zero-cost abstraction)
- **Dynamic**: ~100μs (runtime overhead)

---

### **2. Title Type Optimization**

**Static Title** (Fastest):
```rust
// ✅ Fastest - direct string
title: "My Page"
```

**Template Title** (Fast):
```rust
// ✅ Fast - simple string formatting
title: "{} - Site Name"
```

**Dynamic Title** (Slower):
```rust
// ⚠️ Slower - signal overhead
title: move || title_signal.get()
```

**Performance Ranking:**
1. **Static**: ~0.1μs
2. **Template**: ~1μs
3. **Dynamic**: ~10μs

---

### **3. Metadata Merging Strategy**

**Efficient Merging:**
```rust
use leptos_next_metadata::metadata::merge_metadata;

// ✅ Efficient - single merge operation
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
```

**Avoid Multiple Merges:**
```rust
// ❌ Inefficient - multiple merge operations
let mut metadata = base_metadata;
metadata = merge_metadata(&metadata, &page_metadata)?;
metadata = merge_metadata(&metadata, &user_metadata)?;
metadata = merge_metadata(&metadata, &seo_metadata)?;
```

---

## 🖼️ **OG Image Performance**

### **1. Image Generation Optimization**

**Batch Generation:**
```rust
use leptos_next_metadata::og_image::{OgImageGenerator, OgImageParams};

let generator = OgImageGenerator::new()?;
let params = OgImageParams::default();

// ✅ Generate multiple images in batch
let images = vec![
    ("Title 1", "Description 1"),
    ("Title 2", "Description 2"),
    ("Title 3", "Description 3"),
];

for (title, description) in images {
    let image_bytes = generator.generate_og_image(title, description, &params)?;
    // Save or cache image
}
```

**Template Reuse:**
```rust
// ✅ Reuse template for similar images
let template = generator.load_template("blog-post.svg")?;

for post in blog_posts {
    let image_bytes = generator.generate_og_image_with_template(
        &template,
        post.title,
        post.excerpt,
        &params
    )?;
}
```

### **2. Caching Strategy**

**Enable Caching Feature:**
```toml
[dependencies]
leptos-next-metadata = { version = "0.1.0-beta.1", features = ["caching"] }
```

**Use Metadata Cache:**
```rust
use leptos_next_metadata::utils::cache::MetadataCache;
use std::time::Duration;

let cache = MetadataCache::new(1000); // 1000 entries

// Cache metadata with TTL
cache.set("page_key", metadata.clone(), Duration::from_secs(3600));

// Retrieve cached metadata
if let Some(cached) = cache.get("page_key") {
    // Use cached metadata
    return cached;
}
```

**OG Image Cache:**
```rust
use leptos_next_metadata::utils::cache::OgImageCache;

let image_cache = OgImageCache::new(500); // 500 image entries

// Cache generated images
image_cache.set("image_key", image_bytes.clone(), Duration::from_secs(7200));

// Retrieve cached images
if let Some(cached) = image_cache.get("image_key") {
    return cached;
}
```

---

## 🏷️ **JSON-LD Performance**

### **1. Schema Optimization**

**Minimal Schema** (Fastest):
```rust
// ✅ Fast - minimal fields
let article = Article {
    headline: post.title,
    description: post.excerpt,
    ..Default::default()
};
```

**Complete Schema** (Slower but more SEO value):
```rust
// ⚠️ Slower - more fields to serialize
let article = Article {
    headline: post.title,
    description: post.excerpt,
    author: Some(post.author.name),
    date_published: Some(post.published_at),
    date_modified: Some(post.updated_at),
    image: Some(post.featured_image),
    word_count: Some(post.word_count),
    article_section: Some(post.category),
    ..Default::default()
};
```

### **2. Serialization Optimization**

**Lazy Serialization:**
```rust
use leptos_next_metadata::json_ld::{SchemaOrg, Article};

// ✅ Only serialize when needed
let schema = SchemaOrg::Article(article);
let json_ld = if should_include_json_ld {
    Some(schema.to_json_ld()?)
} else {
    None
};
```

**Batch Serialization:**
```rust
// ✅ Serialize multiple schemas at once
let schemas = vec![
    SchemaOrg::Article(article),
    SchemaOrg::Organization(org),
    SchemaOrg::WebPage(webpage),
];

let json_ld_schemas: Vec<JsonLd> = schemas
    .into_iter()
    .map(|s| s.to_json_ld())
    .collect::<Result<Vec<_>>>()?;
```

---

## 📁 **File Conventions Performance**

### **1. Scanning Optimization**

**Limit Scan Depth:**
```rust
use leptos_next_metadata::conventions::ConventionScanner;

// ✅ Limit scan depth for performance
let scanner = ConventionScanner::new("./app")
    .with_max_depth(3)  // Only scan 3 levels deep
    .with_ignore_patterns(vec!["node_modules", "target", ".git"]);

let conventions = scanner.scan()?;
```

**Cache Scan Results:**
```rust
use std::sync::OnceLock;

static CONVENTIONS: OnceLock<FileConventions> = OnceLock::new();

fn get_conventions() -> &'static FileConventions {
    CONVENTIONS.get_or_init(|| {
        let scanner = ConventionScanner::new("./app");
        scanner.scan().unwrap_or_default()
    })
}
```

### **2. Asset Detection**

**Efficient Asset Detection:**
```rust
// ✅ Only scan for needed assets
let scanner = ConventionScanner::new("./app")
    .with_asset_types(vec![
        AssetType::Favicon,
        AssetType::Manifest,
        AssetType::RobotsTxt,
    ]);

let conventions = scanner.scan()?;
```

---

## 🗄️ **Caching Best Practices**

### **1. Cache Configuration**

**Optimal Cache Sizes:**
```rust
// Metadata cache - larger for more metadata
let metadata_cache = MetadataCache::new(2000); // 2000 entries

// OG Image cache - smaller for memory efficiency
let image_cache = OgImageCache::new(500); // 500 entries

// File conventions cache - very small, rarely changes
let conventions_cache = Arc::new(Mutex::new(HashMap::new()));
```

**TTL Strategy:**
```rust
use std::time::Duration;

// Short TTL for dynamic content
cache.set("dynamic_key", metadata, Duration::from_secs(300)); // 5 minutes

// Medium TTL for semi-static content
cache.set("semi_static_key", metadata, Duration::from_secs(3600)); // 1 hour

// Long TTL for static content
cache.set("static_key", metadata, Duration::from_secs(86400)); // 24 hours
```

### **2. Cache Key Generation**

**Efficient Key Generation:**
```rust
use leptos_next_metadata::utils::cache::generate_cache_key;

// ✅ Generate cache keys efficiently
let cache_key = generate_cache_key(
    &metadata.title,
    &metadata.description,
    &metadata.og_type
);

cache.set(&cache_key, metadata, Duration::from_secs(3600));
```

---

## 📊 **Performance Monitoring**

### **1. Benchmarking**

**Run Performance Tests:**
```bash
# Run all benchmarks
cargo bench

# Run specific benchmarks
cargo bench metadata_merge
cargo bench og_image_generation
cargo bench json_ld_serialization
```

**Custom Benchmarks:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_metadata_merge(c: &mut Criterion) {
    c.bench_function("metadata_merge", |b| {
        let base = create_test_metadata();
        let override_metadata = create_override_metadata();
        
        b.iter(|| {
            black_box(merge_metadata(&base, &override_metadata).unwrap())
        });
    });
}

criterion_group!(benches, benchmark_metadata_merge);
criterion_main!(benches);
```

### **2. Performance Profiling**

**Memory Profiling:**
```bash
# Install memory profiler
cargo install memory-profiler

# Profile memory usage
memory-profiler --output memory_report.json cargo run
```

**CPU Profiling:**
```bash
# Install flamegraph generator
cargo install flamegraph

# Generate CPU flamegraph
cargo flamegraph
```

---

## 🔧 **Advanced Optimizations**

### **1. Compile-Time Optimizations**

**Feature Flags:**
```toml
[dependencies]
leptos-next-metadata = { version = "0.1.0-beta.1", features = ["ssr", "caching"] }

# Only enable needed features
# - ssr: Server-side rendering support
# - caching: Advanced caching strategies
# - og-images: Open Graph image generation
# - json-ld: Structured data support
# - file-conventions: File-based metadata scanning
```

**Release Builds:**
```bash
# Optimized release build
cargo build --release

# Profile-guided optimization
cargo build --release -C profile-generate
cargo run --release  # Collect profile data
cargo build --release -C profile-use
```

### **2. Runtime Optimizations**

**Lazy Loading:**
```rust
use std::sync::OnceLock;

static OG_GENERATOR: OnceLock<OgImageGenerator> = OnceLock::new();

fn get_og_generator() -> &'static OgImageGenerator {
    OG_GENERATOR.get_or_init(|| {
        OgImageGenerator::new().expect("Failed to create OG generator")
    })
}
```

**Async Optimization:**
```rust
use futures::future::join_all;

// ✅ Process multiple metadata operations concurrently
let metadata_futures = pages
    .into_iter()
    .map(|page| generate_page_metadata(page));

let results = join_all(metadata_futures).await;
```

---

## 🚨 **Performance Anti-Patterns**

### **1. Avoid These Patterns**

**❌ Excessive Dynamic Metadata:**
```rust
// Don't do this - too much runtime overhead
generate_metadata! {
    move || async move {
        let user = get_user().await;
        let preferences = get_preferences().await;
        let analytics = get_analytics().await;
        
        Metadata {
            title: Some(user.preferred_title),
            description: Some(preferences.description),
            keywords: Some(analytics.trending_keywords),
            // ... many more dynamic fields
        }
    }
}
```

**❌ Deep Nesting:**
```rust
// Don't do this - complex merge operations
let metadata = merge_metadata(
    &merge_metadata(
        &merge_metadata(&base, &user),
        &preferences
    ),
    &analytics
)?;
```

**❌ Unbounded Caching:**
```rust
// Don't do this - memory leaks
let cache = MetadataCache::new(usize::MAX); // No limit
```

### **2. Better Alternatives**

**✅ Static with Minimal Dynamic:**
```rust
metadata! {
    title: "My Page",
    description: "Page description",
    // Only make truly dynamic fields dynamic
    og_image: if has_custom_image { custom_image } else { "/default.jpg" },
}
```

**✅ Single Merge Operation:**
```rust
// Do this - single merge with all overrides
let all_overrides = Metadata {
    title: Some(user.preferred_title),
    description: Some(preferences.description),
    keywords: Some(analytics.trending_keywords),
    ..Default::default()
};

let metadata = merge_metadata(&base, &all_overrides)?;
```

**✅ Bounded Caching:**
```rust
// Do this - reasonable cache limits
let cache = MetadataCache::new(1000); // 1000 entries max
```

---

## 📈 **Performance Metrics**

### **1. Key Performance Indicators**

**Response Time:**
- **Metadata Generation**: <1ms
- **OG Image Generation**: <100ms
- **JSON-LD Serialization**: <5ms
- **File Convention Scan**: <10ms

**Throughput:**
- **Metadata Operations**: >1000 ops/sec
- **Image Generation**: >10 images/sec
- **Cache Hit Rate**: >90%

**Resource Usage:**
- **Memory**: <50MB under load
- **CPU**: <10% during normal operation
- **Disk I/O**: <1MB/sec

### **2. Monitoring Tools**

**Built-in Metrics:**
```rust
use leptos_next_metadata::utils::cache::CacheStats;

let stats = cache.stats();
println!("Cache hit rate: {:.2}%", stats.hit_rate * 100.0);
println!("Cache size: {}/{}", stats.current_size, stats.max_size);
println!("Average access time: {:?}", stats.avg_access_time);
```

**External Monitoring:**
- **Prometheus**: Metrics collection
- **Grafana**: Visualization
- **Jaeger**: Distributed tracing
- **pprof**: Go-style profiling

---

## 🔗 **Related Documentation**

- **[Quick Start](getting-started.md)** - Get up and running quickly
- **[Core API](../api/core.md)** - Core metadata types and functions
- **[Macros API](../api/macros.md)** - Procedural macro documentation
- **[Migration Guide](migration-guide.md)** - From Next.js to Leptos

---

## 📞 **Getting Help**

- **GitHub Issues**: [Report performance issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- **GitHub Discussions**: [Ask performance questions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- **Documentation**: [Index](../index.md)

---

## 🎯 **Performance Checklist**

### **Before Optimization**
- [ ] **Profile current performance** - Identify bottlenecks
- [ ] **Set performance targets** - Define success criteria
- [ ] **Establish baseline metrics** - Measure current state
- [ ] **Identify optimization opportunities** - Plan improvements

### **During Optimization**
- [ ] **Implement caching** - Add appropriate cache layers
- [ ] **Optimize metadata generation** - Use static when possible
- [ ] **Improve image generation** - Batch and cache operations
- [ ] **Optimize serialization** - Minimize JSON-LD complexity
- [ ] **Profile improvements** - Measure impact of changes

### **After Optimization**
- [ ] **Validate performance gains** - Confirm improvements
- [ ] **Monitor in production** - Track real-world performance
- [ ] **Document optimizations** - Share learnings with team
- [ ] **Plan next iteration** - Identify future improvements

---

## 🎉 **Success Metrics**

After implementing these optimizations, you should see:

- **✅ Faster metadata generation** - Reduced from 5ms to 1ms
- **✅ Improved OG image performance** - Reduced from 800ms to 100ms
- **✅ Better cache efficiency** - Hit rates above 90%
- **✅ Reduced memory usage** - Under 50MB under load
- **✅ Higher throughput** - Over 1000 ops/sec

---

*Last Updated: September 4, 2025*  
*Next: [Troubleshooting Guide](troubleshooting.md) | [Production Roadmap](PRODUCTION_ROADMAP.md)*

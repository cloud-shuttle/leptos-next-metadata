# Implementation Plan: leptos-next-metadata

> **Navigation**: [📚 Documentation Index](index.md) | [📋 Design Document](design.md) | [🤖 Claude Context](../claude.md)

## Table of Contents

- [Project Setup and Initial Structure](#project-setup-and-initial-structure)
- [Core Module Implementation](#core-module-implementation)
- [OG Image Generation Implementation](#og-image-generation-implementation)
- [File Convention Scanner](#file-convention-scanner)
- [JSON-LD Implementation](#json-ld-implementation)
- [SSR/CSR Context Management](#ssrcsr-context-management)
- [Testing Strategy](#testing-strategy)
- [Documentation and Examples](#documentation-and-examples)
- [Benchmarks and Performance Testing](#benchmarks-and-performance-testing)
- [Release Preparation](#release-preparation)
- [Timeline Summary](#timeline-summary)
- [Success Metrics](#success-metrics)
- [Post-Launch Roadmap](#post-launch-roadmap)

## Project Setup and Initial Structure

### Step 1: Initialize the Workspace (Day 1)

```bash
cargo new leptos-next-metadata --lib
cd leptos-next-metadata
```

**Cargo.toml setup:**
```toml
[package]
name = "leptos-next-metadata"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0"
description = "Next.js-style metadata management for Leptos"
repository = "https://github.com/yourusername/leptos-next-metadata"
keywords = ["leptos", "metadata", "seo", "og-image", "json-ld"]
categories = ["web-programming", "wasm"]

[dependencies]
leptos = { version = "0.8.8", features = ["nightly"] }
leptos_meta = { version = "0.8.8", features = ["nightly"] }
leptos_router = { version = "0.8.8", features = ["nightly"] }
leptos_axum = { version = "0.8.8", optional = true }

# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"], optional = true }
async-trait = "0.1"
thiserror = "2.0"

# Image generation
resvg = { version = "0.44", optional = true }
tiny-skia = { version = "0.11", optional = true }
usvg = { version = "0.44", optional = true }
fontdb = { version = "0.22", optional = true }
liquid = { version = "0.26", optional = true }

# JSON-LD
json-ld = "0.17"
iref = "3.2"
static-iref = "3.0"

# Caching & utilities
lru = "0.12"
dashmap = "6.1"
once_cell = "1.20"
regex = "1.11"
glob = "0.3"
walkdir = "2"
notify = { version = "7.0", optional = true }

# Macros
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
proc-macro2 = "1.0"

[dev-dependencies]
criterion = "0.5"
insta = "1.41"
pretty_assertions = "1.4"
tempfile = "3.14"

[features]
default = ["ssr", "og-images", "file-conventions"]
ssr = ["dep:leptos_axum", "dep:tokio", "leptos/ssr"]
hydrate = ["leptos/hydrate"]
csr = ["leptos/csr"]
og-images = ["dep:resvg", "dep:tiny-skia", "dep:usvg", "dep:fontdb", "dep:liquid"]
file-conventions = ["dep:notify"]
nightly = ["leptos/nightly"]

[[bench]]
name = "metadata_resolution"
harness = false

[[bench]]
name = "og_image_generation"
harness = false

[workspace]
members = ["macros"]
```

### Step 2: Create Macro Crate (Day 1)

```bash
cargo new macros --lib
```

**macros/Cargo.toml:**
```toml
[package]
name = "leptos-next-metadata-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full", "extra-traits"] }
```

## Core Module Implementation

### Step 3: Metadata Types and Traits (Days 2-3)

**src/metadata/types.rs:**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<Title>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub authors: Vec<Author>,
    pub creator: Option<String>,
    pub publisher: Option<String>,
    pub robots: Option<Robots>,
    pub viewport: Option<Viewport>,
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<Twitter>,
    pub icons: Vec<Icon>,
    pub manifest: Option<String>,
    pub alternate: HashMap<String, AlternateLink>,
    pub other: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Title {
    Static(String),
    Template { template: String, default: String },
    Absolute(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub site_name: Option<String>,
    pub images: Vec<OgImage>,
    pub locale: Option<String>,
    pub og_type: Option<String>,
    pub videos: Vec<OgVideo>,
    pub audio: Vec<OgAudio>,
}

// Implement merge strategies
pub trait MetadataMerge {
    fn merge(self, parent: Self) -> Self;
}

impl MetadataMerge for Metadata {
    fn merge(mut self, parent: Self) -> Self {
        // Implement shallow merge logic matching Next.js
        self.title = self.title.or(parent.title);
        self.description = self.description.or(parent.description);
        
        if self.keywords.is_empty() {
            self.keywords = parent.keywords;
        }
        
        if let Some(parent_og) = parent.open_graph {
            self.open_graph = Some(match self.open_graph {
                Some(og) => og.merge(parent_og),
                None => parent_og,
            });
        }
        
        // Continue for other fields...
        self
    }
}
```

### Step 4: Static Metadata Macro (Days 4-5)

**macros/src/metadata.rs:**
```rust
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

pub fn metadata_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MetadataInput);
    
    let title = input.title.map(|t| quote! {
        Some(Title::Static(#t.to_string()))
    }).unwrap_or(quote! { None });
    
    let description = input.description.map(|d| quote! {
        Some(#d.to_string())
    }).unwrap_or(quote! { None });
    
    // Process other fields...
    
    quote! {
        {
            use leptos_next_metadata::metadata::*;
            use leptos::*;
            
            create_effect(move |_| {
                let meta = Metadata {
                    title: #title,
                    description: #description,
                    // ... other fields
                    ..Default::default()
                };
                
                set_metadata(meta);
            });
        }
    }
}
```

### Step 5: Dynamic Metadata Generation (Days 6-7)

**src/metadata/dynamic.rs:**
```rust
use leptos::*;
use std::future::Future;
use std::pin::Pin;

pub trait GenerateMetadata {
    type Params;
    type Output: Future<Output = Metadata>;
    
    fn generate(
        params: Self::Params,
        parent: ResolvingMetadata,
    ) -> Self::Output;
}

#[derive(Clone)]
pub struct ResolvingMetadata(Resource<(), Metadata>);

impl ResolvingMetadata {
    pub async fn resolve(&self) -> Metadata {
        self.0.get().unwrap_or_default()
    }
}

pub fn use_metadata<F, Fut>(f: F) -> Metadata
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = Metadata> + 'static,
{
    let meta_resource = create_resource(
        || (),
        move |_| f(),
    );
    
    create_effect(move |_| {
        if let Some(metadata) = meta_resource.get() {
            set_page_metadata(metadata);
        }
    });
    
    meta_resource.get().unwrap_or_default()
}
```

## OG Image Generation Implementation

### Step 6: Image Generation Core (Days 8-10)

**src/og_image/generator.rs:**
```rust
use resvg::usvg::{self, TreeParsing, TreeTextToPath};
use tiny_skia::{Pixmap, Transform};
use std::sync::Arc;

pub struct OgImageGenerator {
    fonts_db: Arc<fontdb::Database>,
    cache: Arc<DashMap<String, Vec<u8>>>,
}

impl OgImageGenerator {
    pub fn new() -> Self {
        let mut fonts_db = fontdb::Database::new();
        fonts_db.load_system_fonts();
        
        Self {
            fonts_db: Arc::new(fonts_db),
            cache: Arc::new(DashMap::new()),
        }
    }
    
    pub async fn generate(&self, params: OgImageParams) -> Result<Vec<u8>, OgImageError> {
        // Check cache
        if let Some(cached) = self.cache.get(&params.cache_key()) {
            return Ok(cached.clone());
        }
        
        // Generate SVG from template
        let svg_data = self.render_template(&params)?;
        
        // Parse SVG
        let tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())?;
        
        // Convert text to paths
        let tree = tree.convert_text(&self.fonts_db);
        
        // Render to pixmap
        let pixmap_size = tree.size.to_int_size();
        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .ok_or(OgImageError::InvalidSize)?;
        
        resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
        
        // Encode as PNG
        let png_data = pixmap.encode_png()?;
        
        // Cache result
        self.cache.insert(params.cache_key(), png_data.clone());
        
        Ok(png_data)
    }
    
    fn render_template(&self, params: &OgImageParams) -> Result<Vec<u8>, OgImageError> {
        // Implement template rendering with Liquid
        todo!()
    }
}
```

### Step 7: Template System (Days 11-12)

**src/og_image/templates.rs:**
```rust
use liquid::{ParserBuilder, Template};
use std::collections::HashMap;

pub struct TemplateEngine {
    parser: ParserBuilder,
    templates: HashMap<String, Template>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let parser = ParserBuilder::with_stdlib()
            .filter(ColorFilter)
            .filter(GradientFilter)
            .build()
            .unwrap();
        
        Self {
            parser,
            templates: HashMap::new(),
        }
    }
    
    pub fn register_template(&mut self, name: &str, source: &str) -> Result<(), Error> {
        let template = self.parser.parse(source)?;
        self.templates.insert(name.to_string(), template);
        Ok(())
    }
    
    pub fn render(&self, name: &str, data: liquid::Object) -> Result<String, Error> {
        let template = self.templates.get(name)
            .ok_or(Error::TemplateNotFound)?;
        
        let output = template.render(&data)?;
        Ok(output)
    }
}

// Custom Liquid filters for SVG generation
struct ColorFilter;
impl liquid::Filter for ColorFilter {
    fn evaluate(&self, input: &dyn liquid::ValueView, _: &liquid::Runtime) -> liquid::Result<liquid::Value> {
        // Implement color manipulation
        todo!()
    }
}
```

## File Convention Scanner

### Step 8: Convention Detection (Days 13-14)

**src/conventions/scanner.rs:**
```rust
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub trait FileConvention: Send + Sync {
    fn pattern(&self) -> &str;
    fn matches(&self, path: &Path) -> bool;
    fn process(&self, path: &Path) -> Result<ConventionMetadata, Error>;
}

pub struct FaviconConvention;

impl FileConvention for FaviconConvention {
    fn pattern(&self) -> &str {
        "favicon.ico"
    }
    
    fn matches(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "favicon.ico")
            .unwrap_or(false)
    }
    
    fn process(&self, path: &Path) -> Result<ConventionMetadata, Error> {
        Ok(ConventionMetadata::Favicon {
            path: path.to_path_buf(),
            sizes: "any".to_string(),
        })
    }
}

pub struct ConventionScanner {
    root: PathBuf,
    conventions: Vec<Box<dyn FileConvention>>,
}

impl ConventionScanner {
    pub fn scan(&self) -> Result<Vec<ConventionMetadata>, Error> {
        let mut results = Vec::new();
        
        for entry in WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            for convention in &self.conventions {
                if convention.matches(path) {
                    results.push(convention.process(path)?);
                    break;
                }
            }
        }
        
        Ok(results)
    }
}
```

## JSON-LD Implementation

### Step 9: Schema Types (Days 15-16)

**src/json_ld/schema.rs:**
```rust
use serde::{Deserialize, Serialize};
use json_ld::syntax::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub schema_type: String,
    pub headline: String,
    pub author: Person,
    pub date_published: String,
    pub date_modified: Option<String>,
    pub image: Vec<String>,
    pub publisher: Organization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@type")]
    pub schema_type: String,
    pub name: String,
    pub url: Option<String>,
}

impl Default for Article {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            schema_type: "Article".to_string(),
            headline: String::new(),
            author: Person::default(),
            date_published: String::new(),
            date_modified: None,
            image: Vec::new(),
            publisher: Organization::default(),
        }
    }
}

// Builder pattern implementation
pub struct ArticleBuilder {
    article: Article,
}

impl ArticleBuilder {
    pub fn new() -> Self {
        Self {
            article: Article::default(),
        }
    }
    
    pub fn headline(mut self, headline: impl Into<String>) -> Self {
        self.article.headline = headline.into();
        self
    }
    
    pub fn author(mut self, author: Person) -> Self {
        self.article.author = author;
        self
    }
    
    pub fn build(self) -> Article {
        self.article
    }
}
```

### Step 10: JSON-LD Component (Days 17-18)

**src/json_ld/component.rs:**
```rust
use leptos::*;

#[component]
pub fn JsonLd<T>(
    data: T,
) -> impl IntoView
where
    T: Serialize + Clone + 'static,
{
    let json_string = create_memo(move |_| {
        serde_json::to_string(&data).unwrap_or_default()
    });
    
    view! {
        <script type="application/ld+json">
            {json_string}
        </script>
    }
}

// Macro for easier usage
#[macro_export]
macro_rules! json_ld {
    ($schema_type:ident { $($field:ident: $value:expr),* $(,)? }) => {
        {
            let schema = $crate::json_ld::schema::$schema_type::builder()
                $(.$field($value))*
                .build();
            
            view! {
                <JsonLd data=schema />
            }
        }
    };
}
```

## SSR/CSR Context Management

### Step 11: Context Detection (Days 19-20)

**src/context/mod.rs:**
```rust
use leptos::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderMode {
    Server,
    Client,
    Hydrate,
}

pub struct MetadataContext {
    mode: RenderMode,
    cache: Arc<dyn MetadataCache>,
}

impl MetadataContext {
    pub fn new() -> Self {
        let mode = Self::detect_mode();
        let cache: Arc<dyn MetadataCache> = match mode {
            RenderMode::Server => Arc::new(ServerCache::new()),
            RenderMode::Client => Arc::new(ClientCache::new()),
            RenderMode::Hydrate => Arc::new(HydrateCache::new()),
        };
        
        Self { mode, cache }
    }
    
    fn detect_mode() -> RenderMode {
        #[cfg(feature = "ssr")]
        {
            if is_browser() {
                RenderMode::Hydrate
            } else {
                RenderMode::Server
            }
        }
        
        #[cfg(all(feature = "csr", not(feature = "ssr")))]
        {
            RenderMode::Client
        }
        
        #[cfg(not(any(feature = "ssr", feature = "csr")))]
        {
            compile_error!("Either 'ssr' or 'csr' feature must be enabled");
        }
    }
}

pub fn provide_metadata_context() {
    provide_context(MetadataContext::new());
}

pub fn use_metadata_context() -> MetadataContext {
    use_context::<MetadataContext>()
        .expect("MetadataContext not provided")
}
```

## Testing Strategy

### Step 12: Unit Tests (Days 21-22)

**tests/metadata_tests.rs:**
```rust
use leptos_next_metadata::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn test_metadata_merge() {
    let parent = Metadata {
        title: Some(Title::Static("Parent".into())),
        description: Some("Parent description".into()),
        ..Default::default()
    };
    
    let child = Metadata {
        title: Some(Title::Static("Child".into())),
        ..Default::default()
    };
    
    let merged = child.merge(parent);
    
    assert_eq!(
        merged.title,
        Some(Title::Static("Child".into()))
    );
    assert_eq!(
        merged.description,
        Some("Parent description".into())
    );
}

#[test]
fn test_title_template() {
    let template = Title::Template {
        template: "%s | My Site".into(),
        default: "My Site".into(),
    };
    
    assert_eq!(
        template.resolve(Some("Home")),
        "Home | My Site"
    );
    
    assert_eq!(
        template.resolve(None),
        "My Site"
    );
}
```

### Step 13: Integration Tests (Days 23-24)

**tests/integration_tests.rs:**
```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[test]
fn test_static_metadata_in_component() {
    let runtime = create_runtime();
    
    run_scope(runtime, |cx| {
        let view = view! {
            <MetadataProvider>
                <TestComponent />
            </MetadataProvider>
        };
        
        // Verify metadata was set correctly
        let metadata = use_metadata_context().get_current();
        assert_eq!(metadata.title, Some(Title::Static("Test".into())));
    });
}

#[tokio::test]
async fn test_og_image_generation() {
    let generator = OgImageGenerator::new();
    
    let params = OgImageParams {
        template: "default".into(),
        data: liquid::object!({
            "title": "Test Image",
            "description": "Test Description",
        }),
        size: (1200, 630),
    };
    
    let image = generator.generate(params).await.unwrap();
    
    // Verify PNG header
    assert_eq!(&image[0..8], b"\x89PNG\r\n\x1a\n");
    
    // Verify dimensions
    // ... additional checks
}
```

## Documentation and Examples

### Step 14: Documentation (Days 25-26)

**examples/basic/src/main.rs:**
```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    provide_metadata_context();
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/blog/:slug" view=BlogPost />
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    metadata! {
        title: "Welcome to My Site",
        description: "A Leptos application with Next.js-style metadata",
        openGraph: {
            title: "Welcome",
            type: "website",
            images: ["/og-home.png"],
        }
    }
    
    view! {
        <h1>"Welcome"</h1>
    }
}

#[component]
fn BlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    generate_metadata! {
        async |params, parent| {
            let post = fetch_post(&params.slug).await?;
            
            Metadata {
                title: Title::Template {
                    template: "%s | Blog".into(),
                    default: "Blog".into(),
                },
                description: Some(post.excerpt),
                ..parent.await
            }
        }
    }
    
    view! {
        <article>
            // Content
        </article>
    }
}
```

## Benchmarks and Performance Testing

### Step 15: Performance Benchmarks (Days 27-28)

**benches/og_image_generation.rs:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_next_metadata::og_image::*;

fn benchmark_og_image_generation(c: &mut Criterion) {
    let generator = OgImageGenerator::new();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("og_image_simple", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let params = OgImageParams::simple(
                    black_box("Test Title"),
                    black_box("Test Description"),
                );
                generator.generate(params).await.unwrap()
            })
        })
    });
    
    c.bench_function("og_image_complex", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let params = OgImageParams::complex(/* ... */);
                generator.generate(params).await.unwrap()
            })
        })
    });
}

criterion_group!(benches, benchmark_og_image_generation);
criterion_main!(benches);
```

## Release Preparation

### Step 16: Final Polish (Days 29-30)

1. **API Documentation**: Complete rustdoc comments for all public APIs
2. **README.md**: Comprehensive getting started guide
3. **CHANGELOG.md**: Document all features for v0.1.0
4. **CI/CD Setup**: GitHub Actions for testing and publishing
5. **Examples**: At least 3 complete example applications
6. **Migration Guide**: From Next.js to leptos-next-metadata

### GitHub Actions Workflow

**.github/workflows/ci.yml:**
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    - uses: Swatinem/rust-cache@v2
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run benchmarks
      run: cargo bench --no-run
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings

  publish:
    needs: test
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    
    - name: Publish to crates.io
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
      run: |
        cargo publish -p leptos-next-metadata-macros
        cargo publish -p leptos-next-metadata
```

## Timeline Summary

- **Week 1**: Project setup, core types, and basic metadata
- **Week 2**: Macro implementation and dynamic metadata
- **Week 3**: OG image generation core
- **Week 4**: File conventions and JSON-LD
- **Week 5**: SSR/CSR handling and context management
- **Week 6**: Testing, documentation, and release preparation

## Success Metrics

- ✅ Feature parity with Next.js metadata API
- ✅ Sub-100ms OG image generation
- ✅ Type-safe JSON-LD with compile-time validation
- ✅ Zero-overhead abstractions for static metadata
- ✅ Comprehensive test coverage (>80%)
- ✅ Full documentation with examples
- ✅ Working with all Leptos SSR modes

## Post-Launch Roadmap

1. **v0.2.0**: Custom OG image components with Leptos syntax
2. **v0.3.0**: Visual metadata editor/preview tool
3. **v0.4.0**: Automated SEO auditing and recommendations
4. **v0.5.0**: Integration with popular Rust web frameworks
5. **v1.0.0**: Stable API with backward compatibility guarantee

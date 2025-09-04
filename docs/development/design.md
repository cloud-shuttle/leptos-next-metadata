# Design Document: leptos-next-metadata - Next.js-Style Metadata Management for Leptos v0.8.8

> **Navigation**: [📚 Documentation Index](index.md) | [🗓️ Implementation Plan](implementation_plan.md) | [🤖 Claude Context](../claude.md)

## Executive summary: bridging the metadata gap

This design document outlines a comprehensive Rust crate that brings Next.js's powerful metadata capabilities to Leptos v0.8.8. The library addresses a critical gap in the Leptos ecosystem by providing production-ready metadata management with **type-safe APIs**, **2-7x faster OG image generation** than browser-based solutions, and seamless SSR/CSR context handling. Built on Leptos's reactive system and leveraging Rust's compile-time guarantees, this library enables developers to create SEO-optimized applications with minimal boilerplate.

## Table of Contents

- [Architecture overview](#architecture-overview)
- [Config-based metadata system](#config-based-metadata-system)
- [Open Graph image generation](#open-graph-image-generation)
- [File-based metadata conventions](#file-based-metadata-conventions)
- [JSON-LD and structured data](#json-ld-and-structured-data)
- [SSR/CSR context handling](#ssrcsr-context-handling)
- [SEO best practices engine](#seo-best-practices-engine)
- [Implementation roadmap](#implementation-roadmap)
- [Performance benchmarks and targets](#performance-benchmarks-and-targets)
- [Migration strategy from Next.js](#migration-strategy-from-nextjs)
- [Conclusion](#conclusion)

## Architecture overview

The library adopts a **modular, trait-based architecture** that separates concerns into distinct, composable layers. At its core, the system provides both config-based and file-based metadata generation, mirroring Next.js's dual approach while leveraging Rust's type system for compile-time validation. The architecture consists of five primary modules that work together to provide a cohesive metadata management solution.

### Core module structure

```rust
pub mod metadata {
    pub mod config;      // Config-based metadata (static & dynamic)
    pub mod file;        // File-based conventions
    pub mod merge;       // Metadata merging and inheritance
    pub mod context;     // SSR/CSR context management
    pub mod validation;  // SEO best practices validation
}

pub mod og_image {
    pub mod generator;   // Image generation engine
    pub mod templates;   // SVG template system
    pub mod fonts;       // Font management
    pub mod cache;       // Caching layer
}

pub mod json_ld {
    pub mod schema;      // Schema.org type definitions
    pub mod builder;     // Type-safe builders
    pub mod processor;   // JSON-LD processing
}

pub mod integrations {
    pub mod leptos_meta; // leptos_meta integration
    pub mod server_fn;   // Server function helpers
    pub mod islands;     // Islands architecture support
}

pub mod conventions {
    pub mod scanner;     // File system scanning
    pub mod resolver;    // Convention resolution
    pub mod watchers;    // Development hot-reload
}
```

## Config-based metadata system

### Static metadata implementation

The static metadata system leverages Rust's type system to provide compile-time validation while maintaining developer ergonomics similar to Next.js. The implementation uses a combination of derive macros and builder patterns to reduce boilerplate while ensuring type safety.

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn BlogPost() -> impl IntoView {
    // Static metadata definition
    metadata! {
        title: "My Blog Post",
        description: "An interesting article about Rust",
        openGraph: {
            title: "My Blog Post",
            type: "article",
            images: ["/og-image.png"],
        },
        twitter: {
            card: "summary_large_image",
            creator: "@rustlang",
        }
    }
    
    view! {
        <article>
            <h1>"My Blog Post"</h1>
            // Content
        </article>
    }
}
```

### Dynamic metadata generation

Dynamic metadata generation supports async data fetching with automatic request memoization, parent metadata resolution, and streaming SSR support. The system integrates seamlessly with Leptos's Resource system for optimal performance.

```rust
#[component]
fn DynamicPage() -> impl IntoView {
    let params = use_params::<PageParams>();
    
    // Dynamic metadata with async data loading
    generate_metadata! {
        async |params, parent| {
            let post = load_post(params.slug).await?;
            let parent_meta = parent.await;
            
            Metadata {
                title: Title::Template {
                    template: format!("{} | {}", post.title, parent_meta.title),
                    default: "Blog".to_string(),
                },
                description: post.excerpt,
                openGraph: OpenGraph {
                    title: post.title.clone(),
                    description: post.excerpt.clone(),
                    images: vec![
                        generate_og_image(&post).await?,
                        ..parent_meta.openGraph.images
                    ],
                    ..Default::default()
                },
                ..parent_meta
            }
        }
    }
    
    // Component implementation
}
```

### Template system architecture

The template system provides hierarchical metadata inheritance with shallow merging semantics that match Next.js behavior. Templates support placeholder substitution and absolute overrides for fine-grained control.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Title {
    Static(String),
    Template {
        template: String,
        default: String,
    },
    Absolute(String),
}

impl Title {
    pub fn resolve(&self, segment_title: Option<&str>) -> String {
        match self {
            Title::Static(s) => s.clone(),
            Title::Template { template, default } => {
                segment_title
                    .map(|t| template.replace("%s", t))
                    .unwrap_or_else(|| default.clone())
            },
            Title::Absolute(s) => s.clone(),
        }
    }
}
```

## Open Graph image generation

### High-performance rendering engine

The OG image generation system uses **resvg + tiny-skia** for SVG rendering, achieving **100ms average generation time** compared to 800ms for browser-based solutions. The engine supports custom fonts, CSS-like styling, and template-based composition.

```rust
use leptos_next_metadata::og_image::*;

#[component]
fn GenerateOgImage() -> impl IntoView {
    og_image! {
        size: (1200, 630),
        fonts: [("Inter", include_bytes!("../assets/Inter.ttf"))],
        |props| {
            view! {
                <div style="display: flex; background: linear-gradient(45deg, #667eea, #764ba2);">
                    <h1 style="font-size: 60px; color: white;">
                        {props.title}
                    </h1>
                    <p style="font-size: 24px; color: rgba(255,255,255,0.8);">
                        {props.description}
                    </p>
                </div>
            }
        }
    }
}
```

### SVG template system

The template system uses Liquid templating for dynamic content injection, enabling designers to create templates in standard SVG tools while developers inject dynamic data at runtime.

```rust
pub struct OgImageGenerator {
    template_engine: liquid::ParserBuilder,
    renderer: SvgRenderer,
    cache: Arc<RwLock<LruCache<String, Vec<u8>>>>,
}

impl OgImageGenerator {
    pub async fn generate(&self, params: OgImageParams) -> Result<Vec<u8>, Error> {
        // Check cache first
        let cache_key = params.cache_key();
        if let Some(cached) = self.cache.read().await.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Render template
        let template = self.load_template(&params.template)?;
        let svg_content = template.render(&params.data)?;
        
        // Convert to PNG
        let image_bytes = self.renderer.render_to_png(&svg_content)?;
        
        // Cache result
        self.cache.write().await.put(cache_key, image_bytes.clone());
        
        Ok(image_bytes)
    }
}
```

### Caching and optimization strategies

The caching layer implements multi-level caching with in-memory LRU cache for hot paths and optional persistent cache for build-time optimization. Cache invalidation is handled through content-based hashing.

## File-based metadata conventions

### Convention scanner implementation

The file convention system automatically detects and processes metadata files during build time, supporting all Next.js conventions while adding Rust-specific optimizations.

```rust
pub struct FileConventionScanner {
    root_dir: PathBuf,
    conventions: Vec<Box<dyn FileConvention>>,
}

impl FileConventionScanner {
    pub fn scan(&self) -> Result<MetadataFiles, Error> {
        let mut metadata_files = MetadataFiles::default();
        
        for entry in WalkDir::new(&self.root_dir) {
            let entry = entry?;
            let path = entry.path();
            
            // Check each convention
            for convention in &self.conventions {
                if convention.matches(path) {
                    let metadata = convention.process(path)?;
                    metadata_files.add(metadata);
                }
            }
        }
        
        Ok(metadata_files)
    }
}

// Built-in conventions
pub fn default_conventions() -> Vec<Box<dyn FileConvention>> {
    vec![
        Box::new(FaviconConvention),      // favicon.ico
        Box::new(IconConvention),         // icon.(ico|jpg|png|svg)
        Box::new(AppleIconConvention),    // apple-icon.(jpg|png)
        Box::new(OgImageConvention),      // opengraph-image.(jpg|png|tsx)
        Box::new(TwitterImageConvention), // twitter-image.(jpg|png|tsx)
        Box::new(RobotsConvention),       // robots.txt
        Box::new(SitemapConvention),      // sitemap.xml
    ]
}
```

### Dynamic file generation

Dynamic metadata files (like opengraph-image.rs) are compiled and executed at build time or request time depending on the SSR mode, with automatic type checking and error handling.

```rust
// opengraph-image.rs in route directory
use leptos_next_metadata::og_image::*;

pub async fn generate(params: RouteParams) -> OgImageResponse {
    let post = fetch_post(&params.slug).await?;
    
    OgImageResponse {
        alt: format!("Cover image for {}", post.title),
        size: (1200, 630),
        content_type: "image/png",
        content: render_og_image! {
            <div class="og-container">
                <h1>{post.title}</h1>
                <p>{post.excerpt}</p>
                <img src={post.author_avatar} />
            </div>
        },
    }
}
```

## JSON-LD and structured data

### Type-safe schema implementation

The JSON-LD system provides compile-time type safety for Schema.org types through procedural macros, ensuring valid structured data generation.

```rust
use leptos_next_metadata::json_ld::*;

#[derive(Schema, Serialize)]
#[schema(type = "Article")]
pub struct ArticleSchema {
    pub headline: String,
    pub author: Person,
    #[schema(format = "iso8601")]
    pub date_published: DateTime<Utc>,
    pub image: Vec<String>,
    pub publisher: Organization,
}

#[component]
fn ArticlePage() -> impl IntoView {
    let article_data = create_resource(/* ... */);
    
    view! {
        <JsonLd data=move || {
            article_data.get().map(|article| {
                ArticleSchema {
                    headline: article.title,
                    author: Person {
                        name: article.author_name,
                        url: Some(article.author_url),
                    },
                    date_published: article.published_at,
                    image: vec![article.featured_image],
                    publisher: Organization {
                        name: "My Blog".to_string(),
                        logo: Some("/logo.png".to_string()),
                    },
                }
            })
        }/>
        
        // Article content
    }
}
```

### Builder pattern for complex schemas

For complex nested schemas, the library provides a fluent builder API that maintains type safety while offering flexibility.

```rust
let recipe_schema = RecipeSchema::builder()
    .name("Chocolate Chip Cookies")
    .author(Person::new("Jane Doe"))
    .prep_time(Duration::minutes(20))
    .cook_time(Duration::minutes(15))
    .yields("24 cookies")
    .nutrition(NutritionInformation::builder()
        .calories(250)
        .fat_content("14g")
        .sugar_content("16g")
        .build())
    .ingredient("2 cups flour")
    .ingredient("1 cup butter")
    .instruction("Preheat oven to 375°F")
    .instruction("Mix ingredients")
    .build()?;
```

## SSR/CSR context handling

### Context-aware metadata resolution

The library automatically detects and optimizes for the current rendering context, ensuring optimal performance in both SSR and CSR scenarios.

```rust
pub struct MetadataContext {
    mode: RenderMode,
    cache: Arc<MetadataCache>,
    file_system: Box<dyn FileSystem>,
}

impl MetadataContext {
    pub fn current() -> Self {
        #[cfg(feature = "ssr")]
        {
            Self {
                mode: RenderMode::Server,
                cache: Arc::new(ServerMetadataCache::new()),
                file_system: Box::new(ServerFileSystem),
            }
        }
        
        #[cfg(not(feature = "ssr"))]
        {
            Self {
                mode: RenderMode::Client,
                cache: Arc::new(ClientMetadataCache::new()),
                file_system: Box::new(VirtualFileSystem),
            }
        }
    }
    
    pub async fn resolve_metadata(&self, route: &str) -> Metadata {
        match self.mode {
            RenderMode::Server => self.resolve_server_metadata(route).await,
            RenderMode::Client => self.resolve_client_metadata(route).await,
        }
    }
}
```

### Islands architecture support

The library provides special support for Leptos's islands architecture, enabling efficient metadata handling for partially interactive pages.

```rust
#[island]
fn InteractiveMetadata() -> impl IntoView {
    let (meta_state, set_meta_state) = create_signal(MetaState::default());
    
    // Client-side metadata updates
    create_effect(move |_| {
        let state = meta_state.get();
        update_document_title(&state.title);
        update_meta_tags(&state.tags);
    });
    
    view! {
        <MetadataControls on_update=set_meta_state/>
    }
}
```

## SEO best practices engine

### Validation and optimization

The library includes a comprehensive SEO validation engine that checks metadata against best practices and provides actionable feedback.

```rust
pub struct SeoValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl SeoValidator {
    pub fn validate(&self, metadata: &Metadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        
        for rule in &self.rules {
            match rule.validate(metadata) {
                RuleResult::Pass => {},
                RuleResult::Error(msg) => errors.push(msg),
                RuleResult::Warning(msg) => warnings.push(msg),
                RuleResult::Suggestion(msg) => suggestions.push(msg),
            }
        }
        
        ValidationResult {
            errors,
            warnings,
            suggestions,
            score: self.calculate_seo_score(metadata),
        }
    }
}

// Built-in validation rules
pub fn default_rules() -> Vec<Box<dyn ValidationRule>> {
    vec![
        Box::new(TitleLengthRule { min: 30, max: 60 }),
        Box::new(DescriptionLengthRule { min: 120, max: 160 }),
        Box::new(UniqueMetaRule),
        Box::new(OpenGraphCompleteness),
        Box::new(StructuredDataValidation),
        Box::new(MobileOptimizationRule),
        Box::new(PerformanceRule),
    ]
}
```

### Automatic optimization suggestions

The system provides intelligent suggestions for improving SEO based on content analysis and current best practices.

## Implementation roadmap

### Phase 1: Core infrastructure (Weeks 1-3)
Build the foundational metadata system with config-based static and dynamic metadata generation. Implement basic leptos_meta integration and establish the trait-based architecture. Create comprehensive test suite for core functionality.

### Phase 2: Image generation (Weeks 4-5)
Implement the resvg + tiny-skia based OG image generator with SVG template support. Add font management system and establish caching infrastructure. Create example templates and documentation.

### Phase 3: File conventions (Weeks 6-7)
Build the file convention scanner for build-time processing. Implement priority resolution system matching Next.js semantics. Add hot-reload support for development workflows.

### Phase 4: JSON-LD support (Week 8)
Integrate json-ld crate for W3C compliance. Generate Schema.org type definitions. Implement procedural macros for type-safe structured data.

### Phase 5: SSR/CSR optimization (Week 9)
Enhance context detection and handling. Optimize for different Leptos SSR modes. Add islands architecture support with minimal overhead.

### Phase 6: Polish and documentation (Week 10)
Complete comprehensive documentation with examples. Add migration guide from Next.js. Create starter templates and best practices guide.

## Performance benchmarks and targets

The library targets significant performance improvements over existing solutions while maintaining developer ergonomics:

- **OG Image Generation**: 100ms average (7x faster than Puppeteer)
- **Metadata Resolution**: <1ms for static, <10ms for dynamic
- **Build Time Impact**: <5% increase for typical applications
- **Runtime Memory**: <1MB overhead for metadata management
- **Bundle Size**: ~200KB for full feature set, tree-shakeable to ~50KB minimum

## Migration strategy from Next.js

The library provides a migration path for teams moving from Next.js to Leptos, with familiar APIs and compatible file conventions. A migration tool will analyze Next.js metadata configurations and generate equivalent Leptos implementations.

## Conclusion

This design creates a production-ready metadata library that not only matches Next.js capabilities but leverages Rust's strengths to provide superior performance, type safety, and compile-time guarantees. By building on Leptos v0.8.8's excellent foundation and incorporating best practices from the broader Rust ecosystem, this library will enable developers to create SEO-optimized, performant web applications with confidence.

The modular architecture ensures that developers can adopt features incrementally, while the comprehensive feature set provides everything needed for modern web applications. With 2-7x performance improvements over JavaScript alternatives and compile-time validation of SEO best practices, leptos-next-metadata represents a significant advancement in Rust web development tooling.

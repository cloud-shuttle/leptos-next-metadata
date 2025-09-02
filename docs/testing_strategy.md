# Comprehensive Testing Strategy for leptos-next-metadata

## Testing Philosophy and Goals

### Core Principles
- **Correctness**: Ensure metadata generates correctly across all contexts
- **Performance**: Validate sub-100ms OG image generation target
- **Compatibility**: Verify Next.js API compatibility
- **Reliability**: Test edge cases and error conditions
- **Regression Prevention**: Catch breaking changes early

### Coverage Targets
- Unit Test Coverage: **90%** minimum
- Integration Test Coverage: **80%** minimum
- Critical Path Coverage: **100%** (metadata resolution, OG generation)
- Documentation Examples: **100%** must compile and run

## Test Infrastructure Setup

### Dependencies

```toml
[dev-dependencies]
# Core testing
insta = { version = "1.41", features = ["yaml", "json"] }
pretty_assertions = "1.4"
rstest = "0.23"
proptest = "1.6"

# Async testing
tokio-test = "0.4"
futures-test = "0.3"

# Web testing
leptos_test = "0.1"
wasm-bindgen-test = "0.3"

# Performance testing
criterion = { version = "0.5", features = ["html_reports", "async_tokio"] }
divan = "0.1"

# Mocking & fixtures
mockall = "0.13"
tempfile = "3.14"
wiremock = "0.6"

# Image comparison
image = "0.25"
image-compare = "0.4"

# Coverage
cargo-tarpaulin = "0.31"

# Fuzzing
arbitrary = { version = "1.4", features = ["derive"] }
```

### Test Organization

```
tests/
├── unit/
│   ├── metadata/
│   │   ├── types_test.rs
│   │   ├── merge_test.rs
│   │   ├── resolution_test.rs
│   │   └── validation_test.rs
│   ├── og_image/
│   │   ├── generator_test.rs
│   │   ├── templates_test.rs
│   │   └── cache_test.rs
│   └── json_ld/
│       ├── schema_test.rs
│       └── builder_test.rs
├── integration/
│   ├── ssr_test.rs
│   ├── csr_test.rs
│   ├── hydration_test.rs
│   └── file_conventions_test.rs
├── e2e/
│   ├── nextjs_compatibility_test.rs
│   └── real_world_test.rs
├── benchmarks/
│   ├── metadata_bench.rs
│   ├── og_image_bench.rs
│   └── json_ld_bench.rs
├── property/
│   ├── metadata_props.rs
│   └── merge_props.rs
├── fixtures/
│   ├── metadata/
│   ├── images/
│   └── templates/
└── snapshots/
```

## Unit Testing Strategy

### Metadata Type Testing

```rust
// tests/unit/metadata/types_test.rs
use leptos_next_metadata::metadata::*;
use rstest::*;
use pretty_assertions::assert_eq;

#[rstest]
#[case::static_title(Title::Static("Test".into()), Some("Page"), "Test")]
#[case::template_with_value(
    Title::Template { 
        template: "%s | Site".into(), 
        default: "Site".into() 
    },
    Some("Home"),
    "Home | Site"
)]
#[case::template_without_value(
    Title::Template { 
        template: "%s | Site".into(), 
        default: "Site".into() 
    },
    None,
    "Site"
)]
#[case::absolute(Title::Absolute("Override".into()), Some("Ignored"), "Override")]
fn test_title_resolution(
    #[case] title: Title,
    #[case] segment: Option<&str>,
    #[case] expected: &str,
) {
    assert_eq!(title.resolve(segment), expected);
}

#[test]
fn test_metadata_default() {
    let meta = Metadata::default();
    assert!(meta.title.is_none());
    assert!(meta.description.is_none());
    assert!(meta.keywords.is_empty());
}

#[rstest]
#[case::valid_description("A valid description", true)]
#[case::too_short("Short", false)]
#[case::too_long(&"x".repeat(200), false)]
fn test_description_validation(
    #[case] description: &str,
    #[case] is_valid: bool,
) {
    let validator = DescriptionValidator::new(20, 160);
    assert_eq!(validator.is_valid(description), is_valid);
}
```

### Metadata Merging Tests

```rust
// tests/unit/metadata/merge_test.rs
use leptos_next_metadata::metadata::*;
use insta::assert_yaml_snapshot;

#[test]
fn test_shallow_merge() {
    let parent = Metadata {
        title: Some(Title::Static("Parent".into())),
        description: Some("Parent description".into()),
        keywords: vec!["parent".into()],
        open_graph: Some(OpenGraph {
            title: Some("Parent OG".into()),
            images: vec![OgImage::new("/parent.jpg")],
            ..Default::default()
        }),
        ..Default::default()
    };
    
    let child = Metadata {
        title: Some(Title::Static("Child".into())),
        keywords: vec!["child".into()],
        open_graph: Some(OpenGraph {
            description: Some("Child OG description".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    let merged = child.merge(parent);
    
    // Use snapshot testing for complex structures
    assert_yaml_snapshot!(merged, @r###"
    title:
      Static: Child
    description: Parent description
    keywords:
      - child
    open_graph:
      title: Parent OG
      description: Child OG description
      images:
        - url: /parent.jpg
    "###);
}

#[test]
fn test_deep_merge_prevention() {
    // Verify that merge is shallow, not deep
    let parent = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Parent".into()),
            description: Some("Parent desc".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    let child = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Child".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    let merged = child.merge(parent);
    
    // Child's OpenGraph should completely replace parent's
    assert_eq!(merged.open_graph.unwrap().description, None);
}
```

### OG Image Generation Tests

```rust
// tests/unit/og_image/generator_test.rs
use leptos_next_metadata::og_image::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;

#[tokio::test]
async fn test_basic_og_image_generation() {
    let generator = OgImageGenerator::new();
    
    let params = OgImageParams {
        template: "default".into(),
        data: liquid::object!({
            "title": "Test Title",
            "description": "Test Description",
            "background": "#667eea",
        }),
        size: (1200, 630),
    };
    
    let image_bytes = generator.generate(params).await.unwrap();
    
    // Verify it's a valid PNG
    let img = ImageReader::new(Cursor::new(&image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    
    assert_eq!(img.width(), 1200);
    assert_eq!(img.height(), 630);
}

#[tokio::test]
async fn test_og_image_caching() {
    let generator = OgImageGenerator::new();
    
    let params = OgImageParams::simple("Title", "Description");
    
    let start = std::time::Instant::now();
    let first = generator.generate(params.clone()).await.unwrap();
    let first_duration = start.elapsed();
    
    let start = std::time::Instant::now();
    let second = generator.generate(params).await.unwrap();
    let second_duration = start.elapsed();
    
    // Second should be much faster due to caching
    assert!(second_duration < first_duration / 10);
    assert_eq!(first, second);
}

#[test]
fn test_svg_template_rendering() {
    let engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630">
            <rect fill="{{ background }}" width="1200" height="630"/>
            <text x="60" y="100" font-size="60">{{ title }}</text>
        </svg>
    "#;
    
    engine.register_template("test", template).unwrap();
    
    let result = engine.render("test", liquid::object!({
        "title": "Hello World",
        "background": "#ff0000",
    })).unwrap();
    
    assert!(result.contains("Hello World"));
    assert!(result.contains("#ff0000"));
}
```

## Integration Testing

### SSR/CSR Context Tests

```rust
// tests/integration/ssr_test.rs
use leptos::*;
use leptos_next_metadata::prelude::*;
use leptos_test::*;

#[test]
fn test_ssr_metadata_rendering() {
    let output = render_to_string(|| {
        provide_metadata_context();
        
        view! {
            <MetadataProvider>
                <TestComponent />
            </MetadataProvider>
        }
    });
    
    assert!(output.contains(r#"<title>Test Title</title>"#));
    assert!(output.contains(r#"<meta name="description" content="Test Description">"#));
}

#[component]
fn TestComponent() -> impl IntoView {
    metadata! {
        title: "Test Title",
        description: "Test Description",
    }
    
    view! { <div>"Content"</div> }
}

#[tokio::test]
async fn test_async_metadata_generation() {
    let app = create_test_app(|| {
        generate_metadata! {
            async |params, parent| {
                // Simulate async data fetching
                tokio::time::sleep(Duration::from_millis(10)).await;
                
                Metadata {
                    title: Some(Title::Static("Async Title".into())),
                    ..Default::default()
                }
            }
        }
        
        view! { <div>"Async content"</div> }
    });
    
    let metadata = app.wait_for_metadata().await;
    assert_eq!(
        metadata.title,
        Some(Title::Static("Async Title".into()))
    );
}
```

### File Convention Tests

```rust
// tests/integration/file_conventions_test.rs
use leptos_next_metadata::conventions::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_favicon_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir(&app_dir).unwrap();
    
    // Create favicon
    fs::write(app_dir.join("favicon.ico"), b"fake ico data").unwrap();
    
    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();
    
    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Favicon { path, .. } => {
            assert!(path.ends_with("favicon.ico"));
        }
        _ => panic!("Expected favicon"),
    }
}

#[test]
fn test_multiple_icon_priority() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();
    
    // Create multiple icons
    fs::write(app_dir.join("icon.png"), b"png").unwrap();
    fs::write(app_dir.join("icon.svg"), b"svg").unwrap();
    fs::write(app_dir.join("apple-icon.png"), b"apple").unwrap();
    
    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();
    
    // Verify priority order
    let icon_types: Vec<_> = results.iter().map(|r| r.icon_type()).collect();
    assert_eq!(icon_types, vec!["apple-touch-icon", "icon", "icon"]);
}
```

## Property-Based Testing

```rust
// tests/property/metadata_props.rs
use proptest::prelude::*;
use leptos_next_metadata::metadata::*;

proptest! {
    #[test]
    fn test_merge_associativity(
        parent in arb_metadata(),
        child in arb_metadata(),
        grandchild in arb_metadata(),
    ) {
        // (a.merge(b)).merge(c) == a.merge(b.merge(c))
        let left = grandchild.clone().merge(child.clone().merge(parent.clone()));
        let right = grandchild.clone().merge(child.clone()).merge(parent.clone());
        
        prop_assert_eq!(left, right);
    }
    
    #[test]
    fn test_title_resolution_never_panics(
        title in arb_title(),
        segment in prop::option::of(any::<String>()),
    ) {
        let _ = title.resolve(segment.as_deref());
    }
    
    #[test]
    fn test_cache_key_deterministic(
        params1 in arb_og_params(),
        params2 in arb_og_params(),
    ) {
        let key1a = params1.cache_key();
        let key1b = params1.cache_key();
        let key2 = params2.cache_key();
        
        prop_assert_eq!(key1a, key1b);
        if params1 != params2 {
            prop_assert_ne!(key1a, key2);
        }
    }
}

fn arb_metadata() -> impl Strategy<Value = Metadata> {
    (
        prop::option::of(arb_title()),
        prop::option::of(any::<String>()),
        prop::collection::vec(any::<String>(), 0..5),
    ).prop_map(|(title, description, keywords)| {
        Metadata {
            title,
            description,
            keywords,
            ..Default::default()
        }
    })
}

fn arb_title() -> impl Strategy<Value = Title> {
    prop_oneof![
        any::<String>().prop_map(Title::Static),
        (any::<String>(), any::<String>()).prop_map(|(template, default)| {
            Title::Template { template, default }
        }),
        any::<String>().prop_map(Title::Absolute),
    ]
}
```

## Performance Testing

### Benchmark Suite

```rust
// tests/benchmarks/metadata_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos_next_metadata::metadata::*;

fn benchmark_metadata_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata");
    
    // Benchmark merge operations
    for size in [1, 10, 100].iter() {
        let parent = create_metadata_with_fields(*size);
        let child = create_metadata_with_fields(*size / 2);
        
        group.bench_with_input(
            BenchmarkId::new("merge", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(child.clone()).merge(black_box(parent.clone()))
                })
            },
        );
    }
    
    // Benchmark title resolution
    group.bench_function("title_static", |b| {
        let title = Title::Static("Test".into());
        b.iter(|| title.resolve(black_box(Some("Page"))))
    });
    
    group.bench_function("title_template", |b| {
        let title = Title::Template {
            template: "%s | My Site".into(),
            default: "My Site".into(),
        };
        b.iter(|| title.resolve(black_box(Some("Page"))))
    });
    
    group.finish();
}

fn benchmark_og_image_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("og_image");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let generator = OgImageGenerator::new();
    
    // Benchmark different image complexities
    for complexity in ["simple", "medium", "complex"].iter() {
        let params = match *complexity {
            "simple" => OgImageParams::simple("Title", "Description"),
            "medium" => OgImageParams::with_image("Title", "Desc", "/img.jpg"),
            "complex" => OgImageParams::with_gradient(
                "Long Title Here",
                "Long description with multiple lines",
                vec!["#667eea", "#764ba2"],
            ),
            _ => unreachable!(),
        };
        
        group.bench_function(
            BenchmarkId::new("generate", complexity),
            |b| {
                b.to_async(&runtime).iter(|| async {
                    generator.generate(black_box(params.clone())).await.unwrap()
                })
            },
        );
    }
    
    // Benchmark cache hits
    let params = OgImageParams::simple("Cached", "Image");
    runtime.block_on(generator.generate(params.clone())).unwrap();
    
    group.bench_function("generate_cached", |b| {
        b.to_async(&runtime).iter(|| async {
            generator.generate(black_box(params.clone())).await.unwrap()
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_metadata_operations, benchmark_og_image_generation);
criterion_main!(benches);
```

### Performance Regression Tests

```rust
// tests/benchmarks/regression_test.rs
use divan::{black_box, Bencher};

#[divan::bench(sample_count = 100)]
fn metadata_merge_performance(bencher: Bencher) {
    let parent = create_complex_metadata();
    let child = create_simple_metadata();
    
    bencher
        .with_inputs(|| (parent.clone(), child.clone()))
        .bench_values(|(p, c)| {
            black_box(c.merge(p))
        });
}

#[divan::bench(sample_count = 100)]
fn json_ld_serialization(bencher: Bencher) {
    let article = create_article_schema();
    
    bencher.bench(|| {
        black_box(serde_json::to_string(&article).unwrap())
    });
}

// Performance assertions
#[test]
fn assert_merge_performance() {
    let start = std::time::Instant::now();
    let parent = create_complex_metadata();
    let child = create_simple_metadata();
    
    for _ in 0..1000 {
        let _ = child.clone().merge(parent.clone());
    }
    
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 10,
        "Merge operation too slow: {:?}",
        duration
    );
}
```

## Visual Regression Testing

```rust
// tests/visual/og_image_test.rs
use image_compare::{Algorithm, Metric, Similarity};
use leptos_next_metadata::og_image::*;

#[tokio::test]
async fn test_og_image_visual_regression() {
    let generator = OgImageGenerator::new();
    
    let params = OgImageParams {
        template: "blog".into(),
        data: liquid::object!({
            "title": "Test Blog Post",
            "author": "John Doe",
            "date": "2024-01-15",
        }),
        size: (1200, 630),
    };
    
    let generated = generator.generate(params).await.unwrap();
    let generated_img = image::load_from_memory(&generated).unwrap();
    
    // Load expected image
    let expected = image::open("tests/fixtures/og_images/blog_expected.png").unwrap();
    
    // Compare images
    let result = image_compare::rgba_hybrid_compare(
        &expected.to_rgba8(),
        &generated_img.to_rgba8(),
    ).unwrap();
    
    assert!(
        result.score > 0.95,
        "Visual regression detected: similarity score {}",
        result.score
    );
}
```

## Error Testing

```rust
// tests/unit/error_handling_test.rs
use leptos_next_metadata::prelude::*;

#[test]
fn test_invalid_template_handling() {
    let engine = TemplateEngine::new();
    
    let invalid_template = "{{ unclosed";
    let result = engine.register_template("bad", invalid_template);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::TemplateParseError(msg) => {
            assert!(msg.contains("unclosed"));
        }
        _ => panic!("Wrong error type"),
    }
}

#[tokio::test]
async fn test_og_generation_with_missing_fonts() {
    let mut generator = OgImageGenerator::new();
    generator.clear_fonts();
    
    let params = OgImageParams::with_custom_font(
        "Title",
        "Description", 
        "NonExistentFont",
    );
    
    // Should fallback gracefully
    let result = generator.generate(params).await;
    assert!(result.is_ok(), "Should fallback to system font");
}

#[test]
#[should_panic(expected = "MetadataContext not provided")]
fn test_missing_context_panic() {
    // Don't provide context
    let _ = use_metadata_context();
}
```

## Fuzzing

```rust
// fuzz/fuzz_targets/metadata_merge.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use leptos_next_metadata::metadata::*;

fuzz_target!(|data: (Metadata, Metadata)| {
    let (parent, child) = data;
    
    // Should never panic
    let _ = child.merge(parent);
});
```

```rust
// fuzz/fuzz_targets/og_template.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use leptos_next_metadata::og_image::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(template_str) = std::str::from_utf8(data) {
        let engine = TemplateEngine::new();
        
        // Try to parse as template
        let _ = engine.register_template("fuzz", template_str);
        
        // Try to render with random data
        let _ = engine.render("fuzz", liquid::object!({
            "title": "Fuzz",
            "data": template_str,
        }));
    }
});
```

## Test Helpers and Utilities

```rust
// tests/helpers/mod.rs
use leptos_next_metadata::prelude::*;

pub fn create_test_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Test".into())),
        description: Some("Test description".into()),
        ..Default::default()
    }
}

pub struct MetadataAssert<'a> {
    metadata: &'a Metadata,
}

impl<'a> MetadataAssert<'a> {
    pub fn new(metadata: &'a Metadata) -> Self {
        Self { metadata }
    }
    
    pub fn has_title(self, expected: &str) -> Self {
        assert_eq!(
            self.metadata.title.as_ref().unwrap().resolve(None),
            expected
        );
        self
    }
    
    pub fn has_description(self, expected: &str) -> Self {
        assert_eq!(
            self.metadata.description.as_deref(),
            Some(expected)
        );
        self
    }
    
    pub fn has_og_image(self, url: &str) -> Self {
        let og = self.metadata.open_graph.as_ref().unwrap();
        assert!(og.images.iter().any(|img| img.url == url));
        self
    }
}

pub fn assert_metadata(metadata: &Metadata) -> MetadataAssert {
    MetadataAssert::new(metadata)
}
```

## Continuous Integration Testing

```yaml
# .github/workflows/test.yml
name: Test Suite

on:
  push:
    branches: [ main ]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, nightly]
        features: 
          - "--all-features"
          - "--no-default-features --features ssr"
          - "--no-default-features --features csr"
          
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v4
    
    - uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
    
    - uses: Swatinem/rust-cache@v2
    
    - name: Run tests
      run: cargo test ${{ matrix.features }}
    
    - name: Run doc tests
      run: cargo test --doc ${{ matrix.features }}
    
    - name: Check examples
      run: |
        for example in examples/*/; do
          cargo check --manifest-path "$example/Cargo.toml"
        done

  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    
    - name: Generate coverage
      run: cargo tarpaulin --out Xml --all-features
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        files: ./cobertura.xml

  benchmarks:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    
    - name: Run benchmarks
      run: cargo bench --all-features -- --save-baseline main
    
    - name: Upload benchmark results
      uses: benchmark-action/github-action-benchmark@v1
      with:
        tool: 'cargo'
        output-file-path: target/criterion/main/estimates.json
        github-token: ${{ secrets.GITHUB_TOKEN }}
        auto-push: true

  fuzz:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    
    - name: Install cargo-fuzz
      run: cargo install cargo-fuzz
    
    - name: Run fuzzing
      run: |
        cd fuzz
        cargo fuzz run metadata_merge -- -max_total_time=60
        cargo fuzz run og_template -- -max_total_time=60
```

## Test Documentation

```rust
// src/lib.rs
//! # Testing Guide
//! 
//! ## Running Tests
//! 
//! ```bash
//! # Run all tests
//! cargo test --all-features
//! 
//! # Run specific test suite
//! cargo test --test integration
//! 
//! # Run with coverage
//! cargo tarpaulin --all-features
//! 
//! # Run benchmarks
//! cargo bench
//! 
//! # Run property tests with more iterations
//! PROPTEST_CASES=10000 cargo test
//! ```
//! 
//! ## Writing Tests
//! 
//! Always test:
//! - Happy path
//! - Edge cases
//! - Error conditions
//! - Performance characteristics
//! 
//! Use snapshot testing for complex outputs:
//! ```rust
//! use insta::assert_yaml_snapshot;
//! 
//! #[test]
//! fn test_complex_output() {
//!     let result = generate_complex_metadata();
//!     assert_yaml_snapshot!(result);
//! }
//! ```
```

## Test Metrics Dashboard

```toml
# .cargo/config.toml
[alias]
test-all = "test --all-features --workspace"
test-unit = "test --lib"
test-integration = "test --test '*'"
test-doc = "test --doc"
bench-all = "bench --all-features"
coverage = "tarpaulin --all-features --out Html"
```

## Summary

This comprehensive testing strategy ensures:

1. **Correctness**: Through unit tests and property-based testing
2. **Performance**: Via benchmarks and regression tests
3. **Compatibility**: Using integration tests and Next.js comparison
4. **Reliability**: Through error testing and fuzzing
5. **Visual Quality**: With image comparison tests
6. **Maintainability**: Through snapshots and test helpers

The strategy covers all critical paths and provides confidence that the library works correctly across different platforms, Leptos rendering modes, and edge cases.
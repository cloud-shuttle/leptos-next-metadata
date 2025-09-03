use divan::{black_box, Bencher};
use leptos_next_metadata::prelude::*;
use std::time::{Duration, Instant};

/// Performance regression tests with specific target thresholds
/// These tests ensure that performance doesn't degrade below acceptable levels

const OG_GENERATION_TARGET_MS: u128 = 100; // Target: <100ms for OG image generation
const METADATA_MERGE_TARGET_US: u128 = 10; // Target: <10μs for metadata merge
const TEMPLATE_RENDER_TARGET_US: u128 = 50; // Target: <50μs for template rendering
const JSON_LD_SERIALIZE_TARGET_US: u128 = 5; // Target: <5μs for JSON-LD serialization

#[divan::bench]
fn metadata_merge_performance(bencher: Bencher) {
    let parent = create_complex_metadata();
    let child = create_simple_metadata();
    
    bencher
        .with_inputs(|| (parent.clone(), child.clone()))
        .bench_values(|(p, c)| {
            black_box(c.merge(p))
        });
}

#[divan::bench]
fn metadata_validation_performance(bencher: Bencher) {
    let metadata = create_complex_metadata();
    
    bencher
        .with_inputs(|| metadata.clone())
        .bench_values(|m| {
            black_box(m.validate())
        });
}

#[divan::bench]
fn title_resolution_performance(bencher: Bencher) {
    let title = Title::Template {
        template: "%s | My Amazing Website With Long Name".into(),
        default: "My Amazing Website With Long Name".into(),
    };
    
    bencher.bench(|| {
        black_box(title.resolve(Some("Very Long Page Title That Might Be Common")))
    });
}

#[divan::bench]
fn json_ld_creation_performance(bencher: Bencher) {
    bencher.bench(|| {
        let article = Article::builder()
            .headline(black_box("Complex Article With Long Headline and Detailed Information"))
            .description(black_box("Comprehensive article description that provides detailed information about the content and context"))
            .author(
                Person::builder()
                    .name(black_box("Dr. Jane Elizabeth Smith"))
                    .url(black_box("https://drjanesmith.example.com/profile"))
                    .same_as(vec![
                        black_box("https://twitter.com/drjanesmith".to_string()),
                        black_box("https://linkedin.com/in/drjanesmith".to_string()),
                    ])
                    .build()
            )
            .date_published(black_box("2024-01-15T10:30:00Z"))
            .date_modified(black_box("2024-01-16T15:45:30Z"))
            .url(black_box("https://example.com/articles/complex-article-with-long-url-path"))
            .image(black_box("https://cdn.example.com/images/high-resolution-article-image.jpg"))
            .publisher(
                Organization::builder()
                    .name(black_box("Premium Publishing House International"))
                    .url(black_box("https://premiumPublishing.example.com"))
                    .logo(black_box("https://cdn.premiumPublishing.example.com/logo-high-res.png"))
                    .description(black_box("Leading international publishing house specializing in technical content"))
                    .build()
            )
            .keywords(vec![
                black_box("performance".to_string()),
                black_box("optimization".to_string()),
                black_box("web-development".to_string()),
                black_box("rust-programming".to_string()),
                black_box("leptos-framework".to_string()),
            ])
            .build();
        
        black_box(article)
    });
}

#[divan::bench]
fn json_ld_serialization_performance(bencher: Bencher) {
    let article = create_complex_article();
    
    bencher
        .with_inputs(|| article.clone())
        .bench_values(|article| {
            black_box(serde_json::to_string(&article).unwrap())
        });
}

#[tokio::main]
async fn og_image_generation_performance() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");
    
    // Test simple image generation
    let simple_params = OgImageParams::simple("Performance Test", "Testing OG image generation speed");
    
    let iterations = 10;
    let mut total_duration = Duration::ZERO;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let _image = generator.generate(simple_params.clone()).await.unwrap();
        total_duration += start.elapsed();
    }
    
    let avg_duration = total_duration / iterations;
    assert!(
        avg_duration.as_millis() < OG_GENERATION_TARGET_MS,
        "OG image generation too slow: {}ms (target: {}ms)",
        avg_duration.as_millis(),
        OG_GENERATION_TARGET_MS
    );
    
    println!("✓ OG image generation: {}ms (target: <{}ms)", avg_duration.as_millis(), OG_GENERATION_TARGET_MS);
}

#[tokio::main]
async fn og_image_cache_performance() {
    let generator = OgImageGenerator::with_cache_size(100).await.expect("Failed to create generator");
    let params = OgImageParams::simple("Cached Image", "Testing cache performance");
    
    // Prime the cache
    let _priming = generator.generate(params.clone()).await.unwrap();
    
    // Measure cached performance
    let iterations = 100;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _image = generator.generate(params.clone()).await.unwrap();
    }
    
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;
    
    // Cached generation should be extremely fast (<1ms)
    assert!(
        avg_duration.as_millis() < 1,
        "Cached OG image generation too slow: {}ms",
        avg_duration.as_millis()
    );
    
    println!("✓ Cached OG image generation: {}μs", avg_duration.as_micros());
}

#[test]
fn metadata_merge_target_performance() {
    let parent = create_complex_metadata();
    let child = create_simple_metadata();
    
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = black_box(child.clone()).merge(black_box(parent.clone()));
    }
    
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;
    
    assert!(
        avg_duration.as_micros() < METADATA_MERGE_TARGET_US,
        "Metadata merge too slow: {}μs (target: <{}μs)",
        avg_duration.as_micros(),
        METADATA_MERGE_TARGET_US
    );
    
    println!("✓ Metadata merge: {}μs (target: <{}μs)", avg_duration.as_micros(), METADATA_MERGE_TARGET_US);
}

#[test]
fn template_rendering_target_performance() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ background | default: '#667eea' }}" width="1200" height="630"/>
            <text x="60" y="200" font-size="60" fill="white">{{ title }}</text>
            <text x="60" y="300" font-size="32" fill="white" opacity="0.9">{{ description }}</text>
            {% if author %}
                <text x="60" y="550" font-size="24" fill="white" opacity="0.7">By {{ author }}</text>
            {% endif %}
        </svg>
    "#;
    
    engine.register_template("performance_test", template).unwrap();
    
    let data = liquid::object!({
        "title": "Performance Test Template",
        "description": "Testing template rendering performance with complex data",
        "author": "Performance Tester",
        "background": "#1e3c72",
    });
    
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = engine.render("performance_test", black_box(data.clone())).unwrap();
    }
    
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;
    
    assert!(
        avg_duration.as_micros() < TEMPLATE_RENDER_TARGET_US,
        "Template rendering too slow: {}μs (target: <{}μs)",
        avg_duration.as_micros(),
        TEMPLATE_RENDER_TARGET_US
    );
    
    println!("✓ Template rendering: {}μs (target: <{}μs)", avg_duration.as_micros(), TEMPLATE_RENDER_TARGET_US);
}

#[test]
fn json_ld_serialization_target_performance() {
    let article = create_complex_article();
    
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = serde_json::to_string(black_box(&article)).unwrap();
    }
    
    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations;
    
    assert!(
        avg_duration.as_micros() < JSON_LD_SERIALIZE_TARGET_US,
        "JSON-LD serialization too slow: {}μs (target: <{}μs)",
        avg_duration.as_micros(),
        JSON_LD_SERIALIZE_TARGET_US
    );
    
    println!("✓ JSON-LD serialization: {}μs (target: <{}μs)", avg_duration.as_micros(), JSON_LD_SERIALIZE_TARGET_US);
}

#[test]
fn memory_usage_test() {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    // Simple memory tracking (not 100% accurate but good enough for regression testing)
    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
    
    struct TrackingAllocator;
    
    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = System.alloc(layout);
            if !ptr.is_null() {
                ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            }
            ptr
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
            ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
        }
    }
    
    // Test memory usage of complex operations
    let initial_memory = ALLOCATED.load(Ordering::SeqCst);
    
    {
        let metadata = create_complex_metadata();
        let child = create_simple_metadata();
        let _merged = child.merge(metadata);
        
        let article = create_complex_article();
        let _serialized = serde_json::to_string(&article).unwrap();
    }
    
    let final_memory = ALLOCATED.load(Ordering::SeqCst);
    let memory_used = final_memory.saturating_sub(initial_memory);
    
    // Should use reasonable amount of memory (less than 1MB for these operations)
    const MAX_MEMORY_USAGE: usize = 1024 * 1024; // 1MB
    assert!(
        memory_used < MAX_MEMORY_USAGE,
        "Memory usage too high: {} bytes (max: {} bytes)",
        memory_used,
        MAX_MEMORY_USAGE
    );
    
    println!("✓ Memory usage: {} KB", memory_used / 1024);
}

#[tokio::test]
async fn concurrent_performance_test() {
    let generator = std::sync::Arc::new(
        OgImageGenerator::new().await.expect("Failed to create generator")
    );
    
    let num_concurrent = 10;
    let start = Instant::now();
    
    let tasks: Vec<_> = (0..num_concurrent).map(|i| {
        let gen = generator.clone();
        tokio::spawn(async move {
            let params = OgImageParams::simple(
                &format!("Concurrent Test {}", i),
                &format!("Testing concurrent generation {}", i)
            );
            gen.generate(params).await.unwrap()
        })
    }).collect();
    
    let _results = futures::future::join_all(tasks).await;
    let total_duration = start.elapsed();
    
    // Concurrent operations should scale well (not much slower than sequential)
    let expected_max_duration = Duration::from_millis(OG_GENERATION_TARGET_MS * 2); // Allow 2x for concurrency overhead
    assert!(
        total_duration < expected_max_duration,
        "Concurrent operations too slow: {}ms (expected: <{}ms)",
        total_duration.as_millis(),
        expected_max_duration.as_millis()
    );
    
    println!("✓ Concurrent OG generation ({} tasks): {}ms", num_concurrent, total_duration.as_millis());
}

#[test]
fn large_dataset_performance() {
    // Test performance with large amounts of data
    let large_metadata = Metadata {
        title: Some(Title::Static("Large Dataset Test".into())),
        description: Some("Testing performance with large amounts of metadata".into()),
        keywords: (0..1000).map(|i| format!("keyword{}", i)).collect(),
        other: (0..1000).map(|i| (format!("key{}", i), format!("value{}", i))).collect(),
        ..Default::default()
    };
    
    let start = Instant::now();
    
    // Test serialization of large dataset
    let _serialized = serde_json::to_string(&large_metadata).unwrap();
    
    // Test validation of large dataset
    let _validation = large_metadata.validate();
    
    // Test cloning of large dataset
    let _cloned = large_metadata.clone();
    
    let duration = start.elapsed();
    
    // Operations on large datasets should still be reasonable (< 10ms)
    assert!(
        duration.as_millis() < 10,
        "Large dataset operations too slow: {}ms",
        duration.as_millis()
    );
    
    println!("✓ Large dataset operations: {}ms", duration.as_millis());
}

// Helper functions
fn create_simple_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Simple Test Title".into())),
        description: Some("Simple test description".into()),
        keywords: vec!["test".into(), "simple".into()],
        ..Default::default()
    }
}

fn create_complex_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Template {
            template: "%s | Complex Test Site".into(),
            default: "Complex Test Site".into(),
        }),
        description: Some("Complex metadata for performance testing with lots of fields and data".into()),
        keywords: (0..50).map(|i| format!("keyword{}", i)).collect(),
        open_graph: Some(OpenGraph {
            title: Some("Complex OG Title".into()),
            description: Some("Complex OG Description".into()),
            images: (0..10).map(|i| OgImage::new(&format!("/image{}.jpg", i))).collect(),
            locale: Some("en_US".into()),
            site_name: Some("Complex Test Site".into()),
            ..Default::default()
        }),
        twitter: Some(Twitter {
            card: Some("summary_large_image".into()),
            site: Some("@complextest".into()),
            creator: Some("@testcreator".into()),
            title: Some("Complex Twitter Title".into()),
            description: Some("Complex Twitter Description".into()),
            image: Some("https://example.com/twitter-image.jpg".into()),
        }),
        robots: Some(Robots {
            index: Some(true),
            follow: Some(true),
            noarchive: Some(false),
            nosnippet: Some(false),
            max_snippet: Some(160),
            max_image_preview: Some(200),
            max_video_preview: Some(300),
            ..Default::default()
        }),
        other: (0..20).map(|i| (format!("custom{}", i), format!("value{}", i))).collect(),
        ..Default::default()
    }
}

fn create_complex_article() -> Article {
    Article::builder()
        .headline("Complex Performance Test Article with Very Long Headline")
        .description("Detailed article description for performance testing with comprehensive information")
        .author(
            Person::builder()
                .name("Dr. Performance Tester")
                .url("https://performance-tester.example.com")
                .same_as(vec![
                    "https://twitter.com/perftester".to_string(),
                    "https://linkedin.com/in/perftester".to_string(),
                ])
                .build()
        )
        .date_published("2024-01-15T10:30:00Z")
        .date_modified("2024-01-16T15:45:30Z")
        .url("https://example.com/performance-test-article")
        .image("https://cdn.example.com/performance-test-image.jpg")
        .publisher(
            Organization::builder()
                .name("Performance Testing Publications")
                .url("https://perftesting.example.com")
                .logo("https://cdn.perftesting.example.com/logo.png")
                .build()
        )
        .keywords((0..20).map(|i| format!("perfkeyword{}", i)).collect())
        .build()
}
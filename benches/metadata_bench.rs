use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use leptos_next_metadata::metadata::*;
use std::collections::HashMap;

fn benchmark_metadata_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata_operations");

    // Benchmark merge operations with different sizes
    for size in [1, 10, 50, 100].iter() {
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
        let title = Title::Static("Test Page Title".into());
        b.iter(|| title.resolve(black_box(Some("Page"))))
    });

    group.bench_function("title_template", |b| {
        let title = Title::Template {
            template: "%s | My Amazing Site".into(),
            default: "My Amazing Site".into(),
        };
        b.iter(|| title.resolve(black_box(Some("Page Title"))))
    });

    group.bench_function("title_absolute", |b| {
        let title = Title::Absolute("Override Title".into());
        b.iter(|| title.resolve(black_box(Some("Ignored"))))
    });

    // Benchmark metadata validation
    group.bench_function("validation_simple", |b| {
        let metadata = create_simple_metadata();
        b.iter(|| black_box(&metadata).validate())
    });

    group.bench_function("validation_complex", |b| {
        let metadata = create_complex_metadata();
        b.iter(|| black_box(&metadata).validate())
    });

    // Benchmark serialization
    group.bench_function("serialize_simple", |b| {
        let metadata = create_simple_metadata();
        b.iter(|| serde_json::to_string(black_box(&metadata)).unwrap())
    });

    group.bench_function("serialize_complex", |b| {
        let metadata = create_complex_metadata();
        b.iter(|| serde_json::to_string(black_box(&metadata)).unwrap())
    });

    group.finish();
}

fn benchmark_og_image_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("og_image_generation");
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Create generator once for all tests
    let generator = runtime.block_on(async {
        OgImageGenerator::new().await.expect("Failed to create generator")
    });

    // Benchmark different image complexities
    for complexity in ["simple", "medium", "complex"].iter() {
        let params = match *complexity {
            "simple" => OgImageParams::simple("Simple Title", "Simple Description"),
            "medium" => OgImageParams {
                template: "blog_post".into(),
                data: liquid::object!({
                    "title": "Medium Complexity Blog Post Title",
                    "description": "A more detailed description with additional context and information",
                    "author": "Jane Doe",
                    "date": "January 15, 2024",
                    "background": "#667eea",
                }),
                size: (1200, 630),
            },
            "complex" => OgImageParams {
                template: "advanced".into(),
                data: liquid::object!({
                    "title": "Complex Template with Multiple Elements and Long Text",
                    "subtitle": "Advanced OG Image Generation with Gradients and Custom Fonts",
                    "description": "This is a complex template that includes gradients, multiple text elements, custom fonts, and advanced SVG features",
                    "author": {
                        "name": "John Smith",
                        "avatar": "/avatar.jpg"
                    },
                    "metadata": {
                        "category": "Technology",
                        "tags": ["rust", "leptos", "performance", "web-dev"],
                        "reading_time": "5 min read"
                    },
                    "gradient": {
                        "colors": ["#667eea", "#764ba2", "#f093fb"]
                    },
                }),
                size: (1200, 630),
            },
            _ => unreachable!(),
        };

        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("generate", complexity),
            complexity,
            |b, _| {
                b.to_async(&runtime).iter(|| async {
                    generator.generate(black_box(params.clone())).await.unwrap()
                })
            },
        );
    }

    // Benchmark different sizes
    let sizes = vec![
        (600, 314, "twitter_summary"),
        (1200, 600, "twitter_large"),
        (1200, 630, "facebook"),
        (800, 600, "custom"),
    ];

    for (width, height, name) in sizes {
        let params = OgImageParams {
            template: "default".into(),
            data: liquid::object!({
                "title": format!("Size Test {}x{}", width, height),
                "description": "Testing different image sizes",
            }),
            size: (width, height),
        };

        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", width, height)),
            &name,
            |b, _| {
                b.to_async(&runtime).iter(|| async {
                    generator.generate(black_box(params.clone())).await.unwrap()
                })
            },
        );
    }

    // Benchmark cache hits vs misses
    let params = OgImageParams::simple("Cached Title", "Cached Description");

    // Prime the cache
    runtime.block_on(generator.generate(params.clone())).unwrap();

    group.bench_function("generate_cached", |b| {
        b.to_async(&runtime).iter(|| async {
            generator.generate(black_box(params.clone())).await.unwrap()
        })
    });

    group.bench_function("generate_uncached", |b| {
        b.to_async(&runtime).iter(|| async {
            let unique_params = OgImageParams::simple(
                &format!("Unique Title {}", fastrand::u64(..)),
                "Always unique description"
            );
            generator.generate(black_box(unique_params)).await.unwrap()
        })
    });

    group.finish();
}

fn benchmark_template_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("template_rendering");
    let mut engine = TemplateEngine::new();

    // Register test templates
    let simple_template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ background }}" width="1200" height="630"/>
            <text x="60" y="200" font-size="48" fill="white">{{ title }}</text>
        </svg>
    "#;

    let complex_template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <linearGradient id="bg">
                    {% for color in gradient.colors %}
                        <stop offset="{{ forloop.index0 | times: 50 }}%" stop-color="{{ color }}"/>
                    {% endfor %}
                </linearGradient>
            </defs>
            <rect fill="url(#bg)" width="1200" height="630"/>
            <text x="60" y="150" font-size="60">{{ title | truncate: 40 }}</text>
            {% if subtitle %}
                <text x="60" y="220" font-size="36">{{ subtitle | truncate: 60 }}</text>
            {% endif %}
            {% for tag in tags limit:5 %}
                <text x="{{ 60 | plus: forloop.index0 | times: 120 }}" y="500" font-size="24">{{ tag }}</text>
            {% endfor %}
        </svg>
    "#;

    engine.register_template("simple", simple_template).unwrap();
    engine.register_template("complex", complex_template).unwrap();

    // Simple template rendering
    group.bench_function("render_simple", |b| {
        let data = liquid::object!({
            "title": "Simple Test Title",
            "background": "#667eea",
        });

        b.iter(|| {
            engine.render("simple", black_box(data.clone())).unwrap()
        })
    });

    // Complex template rendering
    group.bench_function("render_complex", |b| {
        let data = liquid::object!({
            "title": "Complex Template with Many Features",
            "subtitle": "Testing advanced template functionality",
            "gradient": {
                "colors": ["#667eea", "#764ba2", "#f093fb"]
            },
            "tags": ["rust", "leptos", "performance", "web", "metadata"],
        });

        b.iter(|| {
            engine.render("complex", black_box(data.clone())).unwrap()
        })
    });

    // Template with loops
    group.bench_function("render_with_loops", |b| {
        let items = (0..50).map(|i| liquid::object!({
            "name": format!("Item {}", i),
            "value": i * 10,
        })).collect::<Vec<_>>();

        let data = liquid::object!({
            "title": "Loop Performance Test",
            "items": items,
        });

        b.iter(|| {
            engine.render("complex", black_box(data.clone())).unwrap()
        })
    });

    group.finish();
}

fn benchmark_json_ld_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_ld_operations");

    // Benchmark JSON-LD creation
    group.bench_function("create_article", |b| {
        b.iter(|| {
            Article::builder()
                .headline(black_box("Test Article Headline"))
                .description(black_box("Article description"))
                .author(
                    Person::builder()
                        .name(black_box("John Doe"))
                        .url(black_box("https://johndoe.com"))
                        .build()
                )
                .date_published(black_box("2024-01-15T10:00:00Z"))
                .date_modified(black_box("2024-01-16T15:30:00Z"))
                .url(black_box("https://example.com/article"))
                .image(black_box("https://example.com/image.jpg"))
                .publisher(
                    Organization::builder()
                        .name(black_box("Example Publisher"))
                        .url(black_box("https://example.com"))
                        .logo(black_box("https://example.com/logo.png"))
                        .build()
                )
                .build()
        })
    });

    group.bench_function("create_organization", |b| {
        b.iter(|| {
            Organization::builder()
                .name(black_box("Test Organization"))
                .url(black_box("https://test.org"))
                .logo(black_box("https://test.org/logo.png"))
                .description(black_box("A test organization"))
                .address(
                    Address::builder()
                        .street_address(black_box("123 Test St"))
                        .locality(black_box("Test City"))
                        .region(black_box("TS"))
                        .postal_code(black_box("12345"))
                        .country(black_box("US"))
                        .build()
                )
                .contact_point(
                    ContactPoint::builder()
                        .telephone(black_box("+1-555-123-4567"))
                        .contact_type(black_box("customer service"))
                        .build()
                )
                .build()
        })
    });

    // Benchmark serialization
    let article = Article::builder()
        .headline("Performance Test Article")
        .description("Testing JSON-LD serialization performance")
        .author(Person::builder().name("Test Author").build())
        .build();

    group.bench_function("serialize_article", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&article)).unwrap()
        })
    });

    group.bench_function("serialize_pretty", |b| {
        b.iter(|| {
            serde_json::to_string_pretty(black_box(&article)).unwrap()
        })
    });

    group.finish();
}

fn benchmark_file_conventions(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_conventions");

    // Create temporary directory structure for testing
    let temp_dir = tempfile::TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    std::fs::create_dir_all(&app_dir).unwrap();

    // Create various convention files
    let files = vec![
        ("favicon.ico", b"favicon data"),
        ("icon-16x16.png", b"16px icon"),
        ("icon-32x32.png", b"32px icon"),
        ("apple-touch-icon.png", b"apple icon"),
        ("manifest.json", b"{}"),
        ("sitemap.xml", b"<urlset></urlset>"),
        ("robots.txt", b"User-agent: *\nAllow: /"),
        ("opengraph-image.png", b"og image"),
        ("twitter-image.jpg", b"twitter image"),
    ];

    for (filename, content) in &files {
        std::fs::write(app_dir.join(filename), content).unwrap();
    }

    // Also create a deep directory structure
    for i in 0..10 {
        let nested_dir = app_dir.join(format!("level{}", i));
        std::fs::create_dir_all(&nested_dir).unwrap();
        std::fs::write(nested_dir.join("favicon.ico"), b"nested favicon").unwrap();
    }

    group.bench_function("scan_shallow", |b| {
        let scanner = ConventionScanner::new(&app_dir);
        b.iter(|| {
            scanner.scan().unwrap()
        })
    });

    group.bench_function("scan_deep", |b| {
        let scanner = ConventionScanner::new(temp_dir.path());
        b.iter(|| {
            scanner.scan().unwrap()
        })
    });

    // Benchmark pattern matching
    group.bench_function("pattern_matching", |b| {
        let scanner = ConventionScanner::new(&app_dir);
        let paths: Vec<_> = (0..100).map(|i| {
            format!("icon-{}x{}.png", i * 16, i * 16)
        }).collect();

        b.iter(|| {
            for path in &paths {
                let _ = scanner.matches_pattern(black_box(path));
            }
        })
    });

    group.finish();
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Benchmark concurrent OG image generation
    group.bench_function("concurrent_og_generation", |b| {
        b.to_async(&runtime).iter(|| async {
            let generator = std::sync::Arc::new(
                OgImageGenerator::new().await.expect("Failed to create generator")
            );

            let tasks: Vec<_> = (0..10).map(|i| {
                let gen = generator.clone();
                tokio::spawn(async move {
                    let params = OgImageParams::simple(
                        &format!("Concurrent Title {}", i),
                        &format!("Concurrent Description {}", i)
                    );
                    gen.generate(params).await.unwrap()
                })
            }).collect();

            futures::future::join_all(tasks).await
        })
    });

    // Benchmark concurrent metadata operations
    group.bench_function("concurrent_metadata_merge", |b| {
        b.to_async(&runtime).iter(|| async {
            let tasks: Vec<_> = (0..100).map(|i| {
                tokio::spawn(async move {
                    let parent = create_metadata_with_fields(10);
                    let child = Metadata {
                        title: Some(Title::Static(format!("Child {}", i))),
                        ..Default::default()
                    };
                    child.merge(parent)
                })
            }).collect();

            futures::future::join_all(tasks).await
        })
    });

    group.finish();
}

// Helper functions
fn create_metadata_with_fields(count: usize) -> Metadata {
    let mut keywords = Vec::new();
    let mut other = HashMap::new();

    for i in 0..count {
        keywords.push(format!("keyword{}", i));
        other.insert(format!("custom{}", i), format!("value{}", i));
    }

    Metadata {
        title: Some(Title::Static("Test Title".into())),
        description: Some("Test description".into()),
        keywords,
        open_graph: Some(OpenGraph {
            title: Some("OG Title".into()),
            description: Some("OG Description".into()),
            images: (0..std::cmp::min(count, 5)).map(|i| {
                OgImage::new(&format!("/image{}.jpg", i))
            }).collect(),
            ..Default::default()
        }),
        other,
        ..Default::default()
    }
}

fn create_simple_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Simple Title".into())),
        description: Some("Simple description".into()),
        keywords: vec!["simple".into()],
        ..Default::default()
    }
}

fn create_complex_metadata() -> Metadata {
    create_metadata_with_fields(50)
}

criterion_group!(
    benches,
    benchmark_metadata_operations,
    benchmark_og_image_generation,
    benchmark_template_rendering,
    benchmark_json_ld_operations,
    benchmark_file_conventions,
    benchmark_concurrent_operations
);
criterion_main!(benches);

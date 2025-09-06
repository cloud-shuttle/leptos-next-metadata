use leptos_next_metadata::og_image::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use tempfile::TempDir;
use tokio_test;

#[tokio::test]
async fn test_basic_og_image_generation() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");

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

    // Verify file size is reasonable
    assert!(image_bytes.len() > 1000); // Should be at least 1KB
    assert!(image_bytes.len() < 500_000); // Should be less than 500KB
}

#[tokio::test]
async fn test_og_image_different_sizes() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");

    let sizes = vec![
        (1200, 630), // Standard OG
        (1200, 600), // Twitter large
        (600, 314),  // Twitter summary
        (800, 600),  // Custom
    ];

    for (width, height) in sizes {
        let params = OgImageParams {
            template: "default".into(),
            data: liquid::object!({
                "title": format!("Test {}x{}", width, height),
                "background": "#667eea",
            }),
            size: (width, height),
        };

        let image_bytes = generator.generate(params).await.unwrap();
        let img = ImageReader::new(Cursor::new(&image_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        assert_eq!(img.width(), width);
        assert_eq!(img.height(), height);
    }
}

#[tokio::test]
async fn test_og_image_caching() {
    let generator = OgImageGenerator::with_cache_size(10).await.expect("Failed to create generator");

    let params = OgImageParams::simple("Cached Title", "Cached Description");

    // First generation
    let start = std::time::Instant::now();
    let first = generator.generate(params.clone()).await.unwrap();
    let first_duration = start.elapsed();

    // Second generation (should be cached)
    let start = std::time::Instant::now();
    let second = generator.generate(params).await.unwrap();
    let second_duration = start.elapsed();

    // Second should be much faster due to caching
    assert!(second_duration < first_duration / 2);
    assert_eq!(first, second);
    assert_eq!(first.len(), second.len());
}

#[tokio::test]
async fn test_og_image_performance_target() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");

    let params = OgImageParams::simple("Performance Test", "Testing generation speed");

    let start = std::time::Instant::now();
    let _image = generator.generate(params).await.unwrap();
    let duration = start.elapsed();

    // Should generate in under 100ms (target performance)
    assert!(
        duration.as_millis() < 100,
        "OG image generation took {}ms, target is <100ms",
        duration.as_millis()
    );
}

#[tokio::test]
async fn test_svg_template_rendering() {
    let mut engine = TemplateEngine::new();

    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ background | default: '#667eea' }}" width="1200" height="630"/>
            <text x="60" y="200" font-family="Arial" font-size="60" fill="white">{{ title }}</text>
            <text x="60" y="300" font-family="Arial" font-size="32" fill="white" opacity="0.9">{{ description }}</text>
        </svg>
    "#;

    engine.register_template("test", template).unwrap();

    let result = engine.render("test", liquid::object!({
        "title": "Hello World",
        "description": "This is a test description",
        "background": "#ff0000",
    })).unwrap();

    assert!(result.contains("Hello World"));
    assert!(result.contains("This is a test description"));
    assert!(result.contains("#ff0000"));
    assert!(result.contains("<svg"));
    assert!(result.contains("</svg>"));
}

#[tokio::test]
async fn test_template_with_missing_data() {
    let mut engine = TemplateEngine::new();

    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ background | default: '#667eea' }}" width="1200" height="630"/>
            <text x="60" y="200" font-size="60" fill="white">{{ title | default: 'Untitled' }}</text>
            <text x="60" y="300" font-size="32" fill="white">{{ missing_field | default: 'Default Text' }}</text>
        </svg>
    "#;

    engine.register_template("defaults", template).unwrap();

    let result = engine.render("defaults", liquid::object!({
        "title": "Test Title",
        // missing_field and background not provided
    })).unwrap();

    assert!(result.contains("Test Title"));
    assert!(result.contains("Default Text"));
    assert!(result.contains("#667eea")); // Default background
}

#[tokio::test]
async fn test_complex_template_with_conditionals() {
    let mut engine = TemplateEngine::new();

    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" style="stop-color:{{ color1 | default: '#667eea' }}"/>
                    <stop offset="100%" style="stop-color:{{ color2 | default: '#764ba2' }}"/>
                </linearGradient>
            </defs>
            <rect fill="url(#bg)" width="1200" height="630"/>

            <text x="60" y="200" font-family="Arial" font-size="60" font-weight="bold" fill="white">
                {{ title | truncate: 40 }}
            </text>

            {% if subtitle %}
                <text x="60" y="260" font-family="Arial" font-size="32" fill="white" opacity="0.9">
                    {{ subtitle | truncate: 60 }}
                </text>
            {% endif %}

            {% if description %}
                <text x="60" y="350" font-family="Arial" font-size="28" fill="white" opacity="0.8">
                    {{ description | truncate: 80 }}
                </text>
            {% endif %}

            {% if author %}
                <text x="60" y="550" font-family="Arial" font-size="24" fill="white" opacity="0.7">
                    By {{ author }}
                </text>
            {% endif %}
        </svg>
    "#;

    engine.register_template("blog_post", template).unwrap();

    let result = engine.render("blog_post", liquid::object!({
        "title": "Advanced Rust Patterns for Web Development",
        "subtitle": "Building High-Performance Applications",
        "description": "Learn advanced Rust patterns that will make your web applications faster and more reliable.",
        "author": "Jane Developer",
        "color1": "#1e3c72",
        "color2": "#2a5298",
    })).unwrap();

    assert!(result.contains("Advanced Rust Patterns"));
    assert!(result.contains("Building High-Performance"));
    assert!(result.contains("Learn advanced Rust"));
    assert!(result.contains("By Jane Developer"));
    assert!(result.contains("#1e3c72"));
    assert!(result.contains("#2a5298"));
}

#[tokio::test]
async fn test_image_with_logo() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");

    // Create a simple SVG logo
    let logo_svg = r#"
        <svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
            <circle cx="50" cy="50" r="40" fill="#667eea"/>
            <text x="50" y="60" text-anchor="middle" fill="white" font-size="32">L</text>
        </svg>
    "#;

    let params = OgImageParams {
        template: "with_logo".into(),
        data: liquid::object!({
            "title": "Leptos Metadata",
            "description": "Next.js style metadata for Leptos",
            "logo_svg": logo_svg,
        }),
        size: (1200, 630),
    };

    let image_bytes = generator.generate(params).await.unwrap();

    let img = ImageReader::new(Cursor::new(&image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(img.width(), 1200);
    assert_eq!(img.height(), 630);
}

#[tokio::test]
async fn test_error_handling_invalid_template() {
    let mut engine = TemplateEngine::new();

    let invalid_template = "{{ unclosed tag";
    let result = engine.register_template("bad", invalid_template);

    assert!(result.is_err());
    match result.unwrap_err() {
        OgImageError::TemplateParseError(msg) => {
            assert!(msg.contains("unclosed") || msg.contains("parse"));
        }
        _ => panic!("Wrong error type"),
    }
}

#[tokio::test]
async fn test_error_handling_invalid_svg() {
    let generator = OgImageGenerator::new().await.expect("Failed to create generator");

    let params = OgImageParams {
        template: "invalid_svg".into(),
        data: liquid::object!({
            "title": "Test",
        }),
        size: (1200, 630),
    };

    // Register an invalid SVG template
    let mut engine = TemplateEngine::new();
    engine.register_template("invalid_svg", "<svg><invalid-tag></svg>").unwrap();

    // This should handle the error gracefully
    let result = generator.generate_with_engine(params, &engine).await;

    // Depending on implementation, this might return an error or a fallback
    // The important thing is that it doesn't panic
    match result {
        Ok(_) => {}, // Fallback worked
        Err(e) => {
            println!("Expected error for invalid SVG: {:?}", e);
            assert!(matches!(e, OgImageError::RenderingError(_)));
        }
    }
}

#[tokio::test]
async fn test_cache_key_generation() {
    let params1 = OgImageParams {
        template: "test".into(),
        data: liquid::object!({"title": "Test"}),
        size: (1200, 630),
    };

    let params2 = OgImageParams {
        template: "test".into(),
        data: liquid::object!({"title": "Test"}),
        size: (1200, 630),
    };

    let params3 = OgImageParams {
        template: "test".into(),
        data: liquid::object!({"title": "Different"}),
        size: (1200, 630),
    };

    // Same parameters should generate same cache key
    assert_eq!(params1.cache_key(), params2.cache_key());

    // Different parameters should generate different cache keys
    assert_ne!(params1.cache_key(), params3.cache_key());
}

#[tokio::test]
async fn test_concurrent_generation() {
    let generator = std::sync::Arc::new(
        OgImageGenerator::new().await.expect("Failed to create generator")
    );

    let tasks: Vec<_> = (0..10).map(|i| {
        let gen = generator.clone();
        tokio::spawn(async move {
            let params = OgImageParams::simple(
                &format!("Title {}", i),
                &format!("Description {}", i)
            );
            gen.generate(params).await
        })
    }).collect();

    let results = futures::future::join_all(tasks).await;

    for (i, result) in results.into_iter().enumerate() {
        let image_bytes = result.unwrap().unwrap();
        assert!(image_bytes.len() > 1000, "Image {} is too small", i);

        let img = ImageReader::new(Cursor::new(&image_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        assert_eq!(img.width(), 1200);
        assert_eq!(img.height(), 630);
    }
}

#[tokio::test]
async fn test_memory_usage_with_large_cache() {
    let generator = OgImageGenerator::with_cache_size(100).await.expect("Failed to create generator");

    // Generate many different images to test cache management
    for i in 0..150 {
        let params = OgImageParams::simple(
            &format!("Title {}", i),
            &format!("Description {}", i)
        );

        let _image = generator.generate(params).await.unwrap();
    }

    // Cache should have evicted older entries and memory should be reasonable
    let cache_size = generator.cache_size();
    assert!(cache_size <= 100, "Cache size {} exceeds limit", cache_size);
}

#[tokio::test]
async fn test_template_inheritance_and_includes() {
    let mut engine = TemplateEngine::new();

    // Base template
    let base_template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ background | default: '#667eea' }}" width="1200" height="630"/>
            {% block content %}{% endblock %}
        </svg>
    "#;

    // Specific template that extends base
    let blog_template = r#"
        {% extends "base" %}
        {% block content %}
            <text x="60" y="200" font-size="60" fill="white">{{ title }}</text>
            <text x="60" y="300" font-size="32" fill="white">{{ description }}</text>
        {% endblock %}
    "#;

    engine.register_template("base", base_template).unwrap();
    engine.register_template("blog", blog_template).unwrap();

    let result = engine.render("blog", liquid::object!({
        "title": "Blog Post",
        "description": "This is a blog post",
        "background": "#123456",
    })).unwrap();

    assert!(result.contains("Blog Post"));
    assert!(result.contains("This is a blog post"));
    assert!(result.contains("#123456"));
}

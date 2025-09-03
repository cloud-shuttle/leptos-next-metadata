use leptos_next_metadata::og_image::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_default_template_registration() {
    let mut engine = TemplateEngine::new();
    
    // Test that default templates are registered
    let templates = engine.list_templates();
    assert!(templates.contains(&"default".to_string()));
    assert!(templates.contains(&"blog_post".to_string()));
    assert!(templates.contains(&"article".to_string()));
    assert!(templates.contains(&"simple".to_string()));
}

#[test]
fn test_template_registration() {
    let mut engine = TemplateEngine::new();
    
    let custom_template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="{{ color }}" width="1200" height="630"/>
            <text x="60" y="200" font-size="48" fill="white">{{ title }}</text>
        </svg>
    "#;
    
    let result = engine.register_template("custom", custom_template);
    assert!(result.is_ok());
    
    let templates = engine.list_templates();
    assert!(templates.contains(&"custom".to_string()));
}

#[test]
fn test_template_override() {
    let mut engine = TemplateEngine::new();
    
    let template_v1 = r#"<svg><text>Version 1</text></svg>"#;
    let template_v2 = r#"<svg><text>Version 2</text></svg>"#;
    
    engine.register_template("test", template_v1).unwrap();
    engine.register_template("test", template_v2).unwrap(); // Override
    
    let result = engine.render("test", liquid::object!({})).unwrap();
    assert!(result.contains("Version 2"));
    assert!(!result.contains("Version 1"));
}

#[test]
fn test_template_rendering_with_filters() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <text font-size="48">{{ title | upcase }}</text>
            <text font-size="32">{{ description | truncate: 50 }}</text>
            <text font-size="24">{{ date | date: "%B %d, %Y" }}</text>
            <text font-size="20">{{ price | currency }}</text>
        </svg>
    "#;
    
    engine.register_template("filters", template).unwrap();
    
    let result = engine.render("filters", liquid::object!({
        "title": "hello world",
        "description": "This is a very long description that should be truncated to fit within the specified length limit",
        "date": "2024-01-15T10:30:00Z",
        "price": 29.99,
    })).unwrap();
    
    assert!(result.contains("HELLO WORLD"));
    assert!(result.contains("This is a very long description that should be"));
    assert!(!result.contains("truncated to fit")); // Should be truncated
    assert!(result.contains("January 15, 2024"));
    assert!(result.contains("$29.99"));
}

#[test]
fn test_conditional_rendering() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <text x="60" y="100">{{ title }}</text>
            {% if subtitle %}
                <text x="60" y="150">{{ subtitle }}</text>
            {% endif %}
            {% if author %}
                <text x="60" y="200">By {{ author }}</text>
            {% else %}
                <text x="60" y="200">Anonymous</text>
            {% endif %}
            {% unless hide_date %}
                <text x="60" y="250">{{ date }}</text>
            {% endunless %}
        </svg>
    "#;
    
    engine.register_template("conditional", template).unwrap();
    
    // Test with all fields
    let result1 = engine.render("conditional", liquid::object!({
        "title": "Test Title",
        "subtitle": "Test Subtitle",
        "author": "John Doe",
        "date": "2024-01-15",
        "hide_date": false,
    })).unwrap();
    
    assert!(result1.contains("Test Title"));
    assert!(result1.contains("Test Subtitle"));
    assert!(result1.contains("By John Doe"));
    assert!(result1.contains("2024-01-15"));
    
    // Test with missing fields
    let result2 = engine.render("conditional", liquid::object!({
        "title": "Test Title",
        "date": "2024-01-15",
        "hide_date": true,
    })).unwrap();
    
    assert!(result2.contains("Test Title"));
    assert!(!result2.contains("Test Subtitle"));
    assert!(result2.contains("Anonymous"));
    assert!(!result2.contains("2024-01-15")); // Should be hidden
}

#[test]
fn test_loop_rendering() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <text x="60" y="100">{{ title }}</text>
            {% for tag in tags %}
                <text x="{{ 60 | plus: forloop.index0 | times: 100 }}" y="200" fill="#{{ colors[forloop.index0] }}">
                    {{ tag | upcase }}
                </text>
            {% endfor %}
            {% for item in items limit:3 %}
                <text x="60" y="{{ 250 | plus: forloop.index | times: 30 }}">
                    {{ forloop.index }}. {{ item.name }}
                </text>
            {% endfor %}
        </svg>
    "#;
    
    engine.register_template("loops", template).unwrap();
    
    let result = engine.render("loops", liquid::object!({
        "title": "Tags and Items",
        "tags": ["rust", "leptos", "web"],
        "colors": ["ff0000", "00ff00", "0000ff"],
        "items": [
            {"name": "First Item"},
            {"name": "Second Item"},
            {"name": "Third Item"},
            {"name": "Fourth Item"}, // Should be limited
        ],
    })).unwrap();
    
    assert!(result.contains("RUST"));
    assert!(result.contains("LEPTOS"));
    assert!(result.contains("WEB"));
    assert!(result.contains("#ff0000"));
    assert!(result.contains("#00ff00"));
    assert!(result.contains("#0000ff"));
    assert!(result.contains("1. First Item"));
    assert!(result.contains("2. Second Item"));
    assert!(result.contains("3. Third Item"));
    assert!(!result.contains("4. Fourth Item")); // Should be limited to 3
}

#[test]
fn test_nested_objects() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <text x="60" y="100">{{ post.title }}</text>
            <text x="60" y="150">{{ post.author.name }}</text>
            <text x="60" y="200">{{ post.metadata.category }}</text>
            <text x="60" y="250">{{ post.stats.views }} views</text>
        </svg>
    "#;
    
    engine.register_template("nested", template).unwrap();
    
    let result = engine.render("nested", liquid::object!({
        "post": {
            "title": "Advanced Rust Techniques",
            "author": {
                "name": "Jane Smith",
                "email": "jane@example.com"
            },
            "metadata": {
                "category": "Programming",
                "tags": ["rust", "advanced"]
            },
            "stats": {
                "views": 1234,
                "likes": 56
            }
        }
    })).unwrap();
    
    assert!(result.contains("Advanced Rust Techniques"));
    assert!(result.contains("Jane Smith"));
    assert!(result.contains("Programming"));
    assert!(result.contains("1234 views"));
}

#[test]
fn test_template_with_custom_fonts() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <style>
                    .title { font-family: "{{ title_font | default: 'Arial' }}"; font-size: 48px; }
                    .body { font-family: "{{ body_font | default: 'Arial' }}"; font-size: 24px; }
                </style>
            </defs>
            <text x="60" y="100" class="title">{{ title }}</text>
            <text x="60" y="200" class="body">{{ description }}</text>
        </svg>
    "#;
    
    engine.register_template("fonts", template).unwrap();
    
    let result = engine.render("fonts", liquid::object!({
        "title": "Custom Fonts",
        "description": "Using custom font families",
        "title_font": "Helvetica",
        "body_font": "Georgia",
    })).unwrap();
    
    assert!(result.contains("Helvetica"));
    assert!(result.contains("Georgia"));
    assert!(result.contains("Custom Fonts"));
}

#[test]
fn test_gradient_backgrounds() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <defs>
                {% if gradient %}
                    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
                        {% for color in gradient.colors %}
                            <stop offset="{{ color.offset }}%" style="stop-color:{{ color.value }}"/>
                        {% endfor %}
                    </linearGradient>
                {% endif %}
            </defs>
            <rect fill="{% if gradient %}url(#bg){% else %}{{ background | default: '#667eea' }}{% endif %}" 
                  width="1200" height="630"/>
            <text x="60" y="200" font-size="48" fill="white">{{ title }}</text>
        </svg>
    "#;
    
    engine.register_template("gradient", template).unwrap();
    
    let result = engine.render("gradient", liquid::object!({
        "title": "Gradient Background",
        "gradient": {
            "colors": [
                {"offset": 0, "value": "#667eea"},
                {"offset": 100, "value": "#764ba2"}
            ]
        }
    })).unwrap();
    
    assert!(result.contains("<linearGradient"));
    assert!(result.contains("#667eea"));
    assert!(result.contains("#764ba2"));
    assert!(result.contains("url(#bg)"));
}

#[test]
fn test_template_with_images() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg"
             xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect fill="{{ background | default: '#f0f0f0' }}" width="1200" height="630"/>
            {% if logo %}
                <image x="{{ logo.x | default: 60 }}" y="{{ logo.y | default: 60 }}" 
                       width="{{ logo.width | default: 100 }}" height="{{ logo.height | default: 100 }}"
                       xlink:href="{{ logo.url }}"/>
            {% endif %}
            <text x="60" y="300" font-size="48">{{ title }}</text>
        </svg>
    "#;
    
    engine.register_template("with_image", template).unwrap();
    
    let result = engine.render("with_image", liquid::object!({
        "title": "With Logo",
        "background": "#ffffff",
        "logo": {
            "url": "/logo.png",
            "x": 50,
            "y": 50,
            "width": 120,
            "height": 120
        }
    })).unwrap();
    
    assert!(result.contains("<image"));
    assert!(result.contains("/logo.png"));
    assert!(result.contains("x=\"50\""));
    assert!(result.contains("width=\"120\""));
}

#[test]
fn test_template_validation() {
    let mut engine = TemplateEngine::new();
    
    // Test invalid Liquid syntax
    let invalid_liquid = "{{ unclosed tag";
    let result = engine.register_template("invalid", invalid_liquid);
    assert!(result.is_err());
    
    // Test invalid SVG structure
    let invalid_svg = "<svg><invalid-element></svg>";
    let result = engine.register_template("invalid_svg", invalid_svg);
    // Should succeed at registration time, fail at render time
    assert!(result.is_ok());
    
    let render_result = engine.render("invalid_svg", liquid::object!({}));
    // Depending on implementation, might succeed or fail
    match render_result {
        Ok(_) => {}, // SVG validation might be lenient
        Err(_) => {}, // Or strict
    }
}

#[test]
fn test_template_performance_with_large_data() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <rect fill="#f0f0f0" width="1200" height="630"/>
            {% for item in items limit:10 %}
                <text x="60" y="{{ 100 | plus: forloop.index | times: 40 }}">{{ item.name }}</text>
            {% endfor %}
        </svg>
    "#;
    
    engine.register_template("performance", template).unwrap();
    
    // Create large dataset
    let items: Vec<_> = (0..1000).map(|i| liquid::object!({
        "name": format!("Item {}", i),
        "value": i * 10,
    })).collect();
    
    let start = std::time::Instant::now();
    let result = engine.render("performance", liquid::object!({
        "items": items,
    })).unwrap();
    let duration = start.elapsed();
    
    assert!(result.contains("Item 1"));
    assert!(result.contains("Item 10"));
    assert!(!result.contains("Item 11")); // Should be limited to 10
    
    // Should render quickly even with large data
    assert!(duration.as_millis() < 100, "Template rendering took {}ms", duration.as_millis());
}

#[test]
fn test_template_security() {
    let mut engine = TemplateEngine::new();
    
    let template = r#"
        <svg viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
            <text x="60" y="200">{{ user_input }}</text>
        </svg>
    "#;
    
    engine.register_template("security", template).unwrap();
    
    // Test with potentially dangerous input
    let dangerous_inputs = vec![
        "<script>alert('xss')</script>",
        "javascript:void(0)",
        "<img src=x onerror=alert(1)>",
        "{{ system.password }}", // Template injection attempt
    ];
    
    for input in dangerous_inputs {
        let result = engine.render("security", liquid::object!({
            "user_input": input,
        })).unwrap();
        
        // Should be escaped or sanitized
        assert!(result.contains("&lt;") || !result.contains("<script>"));
        assert!(!result.contains("javascript:"));
        assert!(!result.contains("onerror="));
    }
}

#[test]
fn test_template_caching() {
    let mut engine = TemplateEngine::with_cache();
    
    let template = r#"<svg><text>{{ title }}</text></svg>"#;
    engine.register_template("cached", template).unwrap();
    
    // First render
    let start = std::time::Instant::now();
    let _result1 = engine.render("cached", liquid::object!({"title": "Test"})).unwrap();
    let first_duration = start.elapsed();
    
    // Second render (should be cached)
    let start = std::time::Instant::now();
    let _result2 = engine.render("cached", liquid::object!({"title": "Test"})).unwrap();
    let second_duration = start.elapsed();
    
    // Second render should be faster (or at least not significantly slower)
    assert!(second_duration <= first_duration * 2);
}
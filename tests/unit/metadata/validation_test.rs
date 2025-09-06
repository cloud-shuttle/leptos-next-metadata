use leptos_next_metadata::metadata::*;
use rstest::*;
use pretty_assertions::assert_eq;

#[test]
fn test_title_validation() {
    let validator = TitleValidator::new(10, 60);

    assert!(validator.is_valid("Good Title"));
    assert!(!validator.is_valid("")); // Too short
    assert!(!validator.is_valid("x".repeat(70).as_str())); // Too long
    assert!(!validator.is_valid("   ")); // Whitespace only
}

#[test]
fn test_description_validation() {
    let validator = DescriptionValidator::new(50, 160);

    assert!(validator.is_valid("This is a good description that meets the length requirements and provides value."));
    assert!(!validator.is_valid("Too short")); // Too short
    assert!(!validator.is_valid(&"x".repeat(200))); // Too long
    assert!(!validator.is_valid("")); // Empty
    assert!(!validator.is_valid("   ")); // Whitespace only
}

#[test]
fn test_keywords_validation() {
    let validator = KeywordsValidator::new(10, 3);

    assert!(validator.is_valid(&["rust", "leptos", "web"]));
    assert!(!validator.is_valid(&["rust", "leptos", "web", "framework"])); // Too many
    assert!(!validator.is_valid(&["rust", "", "web"])); // Empty keyword
    assert!(!validator.is_valid(&["rust", "verylongkeywordthatexceedslimit", "web"])); // Too long keyword
}

#[test]
fn test_url_validation() {
    let validator = UrlValidator::new();

    // Valid URLs
    assert!(validator.is_valid("https://example.com"));
    assert!(validator.is_valid("http://localhost:3000"));
    assert!(validator.is_valid("https://sub.example.com/path?query=1"));
    assert!(validator.is_valid("/relative/path")); // Relative paths are valid

    // Invalid URLs
    assert!(!validator.is_valid("not-a-url"));
    assert!(!validator.is_valid("ftp://example.com")); // Unsupported protocol
    assert!(!validator.is_valid("")); // Empty
    assert!(!validator.is_valid("javascript:alert('xss')")); // Dangerous protocol
}

#[test]
fn test_og_image_validation() {
    let image = OgImage {
        url: "https://example.com/image.jpg".into(),
        width: Some(1200),
        height: Some(630),
        alt: Some("Test image".into()),
        ..Default::default()
    };

    let validator = OgImageValidator::new();
    let result = validator.validate(&image);

    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_og_image_validation_failures() {
    let image = OgImage {
        url: "invalid-url".into(),
        width: Some(50), // Too small
        height: Some(50), // Too small
        alt: Some("".into()), // Empty alt text
        ..Default::default()
    };

    let validator = OgImageValidator::new();
    let result = validator.validate(&image);

    assert!(!result.is_valid);
    assert!(result.errors.len() >= 3);
    assert!(result.errors.iter().any(|e| e.contains("URL")));
    assert!(result.errors.iter().any(|e| e.contains("width")));
    assert!(result.errors.iter().any(|e| e.contains("alt")));
}

#[test]
fn test_metadata_full_validation() {
    let metadata = Metadata {
        title: Some(Title::Static("".into())), // Invalid: empty
        description: Some("Short".into()), // Invalid: too short
        keywords: vec!["".into(), "valid".into()], // Invalid: empty keyword
        open_graph: Some(OpenGraph {
            title: Some("x".repeat(100).into()), // Invalid: too long
            images: vec![OgImage {
                url: "invalid-url".into(), // Invalid URL
                ..Default::default()
            }],
            ..Default::default()
        }),
        robots: Some(Robots {
            max_snippet: Some(-1), // Invalid: negative
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = metadata.validate();

    assert!(!result.is_valid);
    assert!(result.errors.len() >= 5);

    // Check for specific error types
    let error_string = result.errors.join(" ");
    assert!(error_string.contains("title"));
    assert!(error_string.contains("description"));
    assert!(error_string.contains("keyword"));
    assert!(error_string.contains("OpenGraph"));
    assert!(error_string.contains("max_snippet"));
}

#[test]
fn test_validation_result_helper_methods() {
    let mut result = ValidationResult::new();

    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty());

    result.add_error("Test error");
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);

    result.add_warning("Test warning");
    assert_eq!(result.warnings.len(), 1);

    result.add_error_if(true, "Conditional error");
    assert_eq!(result.errors.len(), 2);

    result.add_error_if(false, "Should not be added");
    assert_eq!(result.errors.len(), 2);
}

#[rstest]
#[case::valid_canonical("https://example.com/canonical", true)]
#[case::valid_relative("/canonical", true)]
#[case::invalid_empty("", false)]
#[case::invalid_javascript("javascript:void(0)", false)]
#[case::invalid_malformed("not-a-url", false)]
fn test_canonical_url_validation(
    #[case] url: &str,
    #[case] should_be_valid: bool,
) {
    let validator = CanonicalUrlValidator::new();
    assert_eq!(validator.is_valid(url), should_be_valid);
}

#[test]
fn test_robots_validation() {
    let robots = Robots {
        index: Some(true),
        follow: Some(false),
        max_snippet: Some(150),
        max_image_preview: Some(200),
        max_video_preview: Some(300),
        ..Default::default()
    };

    let validator = RobotsValidator::new();
    let result = validator.validate(&robots);

    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_robots_validation_failures() {
    let robots = Robots {
        max_snippet: Some(-1), // Invalid: negative
        max_image_preview: Some(0), // Invalid: zero
        max_video_preview: Some(10000), // Invalid: too large
        ..Default::default()
    };

    let validator = RobotsValidator::new();
    let result = validator.validate(&robots);

    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 3);
}

#[test]
fn test_icon_validation() {
    let icon = Icon {
        url: "https://example.com/icon.png".into(),
        sizes: Some("32x32".into()),
        icon_type: Some("image/png".into()),
        rel: Some("icon".into()),
        ..Default::default()
    };

    let validator = IconValidator::new();
    let result = validator.validate(&icon);

    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_icon_validation_failures() {
    let icon = Icon {
        url: "".into(), // Invalid: empty
        sizes: Some("invalid".into()), // Invalid: not in format "WxH"
        icon_type: Some("text/plain".into()), // Invalid: not an image type
        ..Default::default()
    };

    let validator = IconValidator::new();
    let result = validator.validate(&icon);

    assert!(!result.is_valid);
    assert!(result.errors.len() >= 3);
}

#[test]
fn test_viewport_validation() {
    let viewport = Viewport {
        width: Some("device-width".into()),
        initial_scale: Some(1.0),
        maximum_scale: Some(2.0),
        minimum_scale: Some(0.5),
        user_scalable: Some(true),
        ..Default::default()
    };

    let validator = ViewportValidator::new();
    let result = validator.validate(&viewport);

    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_viewport_validation_warnings() {
    let viewport = Viewport {
        initial_scale: Some(0.1), // Warning: very small
        maximum_scale: Some(10.0), // Warning: very large
        user_scalable: Some(false), // Warning: accessibility concern
        ..Default::default()
    };

    let validator = ViewportValidator::new();
    let result = validator.validate(&viewport);

    assert!(result.is_valid); // Should be valid but have warnings
    assert!(result.warnings.len() >= 2);
}

#[test]
fn test_seo_recommendations() {
    let metadata = Metadata {
        title: Some(Title::Static("Page".into())), // Too short
        description: None, // Missing
        keywords: vec![], // Empty
        open_graph: None, // Missing
        ..Default::default()
    };

    let recommendations = metadata.get_seo_recommendations();

    assert!(recommendations.len() >= 4);
    assert!(recommendations.iter().any(|r| r.contains("title") && r.contains("longer")));
    assert!(recommendations.iter().any(|r| r.contains("description")));
    assert!(recommendations.iter().any(|r| r.contains("keywords")));
    assert!(recommendations.iter().any(|r| r.contains("Open Graph")));
}

#[test]
fn test_performance_validation() {
    let metadata = Metadata {
        keywords: vec!["keyword".into(); 50], // Too many keywords
        open_graph: Some(OpenGraph {
            images: vec![OgImage::new("/image.jpg"); 20], // Too many images
            ..Default::default()
        }),
        icons: Some(Icons {
            other: (0..100).map(|i| Icon::new(&format!("/icon-{}.png", i), "32x32")).collect(), // Too many icons
            ..Default::default()
        }),
        ..Default::default()
    };

    let performance_result = metadata.validate_performance();

    assert!(!performance_result.is_valid);
    assert!(performance_result.warnings.iter().any(|w| w.contains("keywords")));
    assert!(performance_result.warnings.iter().any(|w| w.contains("images")));
    assert!(performance_result.warnings.iter().any(|w| w.contains("icons")));
}

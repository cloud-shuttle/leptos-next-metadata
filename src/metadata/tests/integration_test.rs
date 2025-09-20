//! Integration tests for metadata functionality
//!
//! This module contains integration tests that test the interaction between
//! different metadata components and serialization/deserialization.

use super::super::types::*;

#[test]
fn test_title_from_impls() {
    let title1: Title = "String Title".into();
    let title2: Title = "String Title".to_string().into();

    assert!(matches!(title1, Title::Static(_)));
    assert!(matches!(title2, Title::Static(_)));
}

#[test]
fn test_keywords_from_impls() {
    let keywords1: Keywords = "single".into();
    let keywords2: Keywords = vec!["one".to_string(), "two".to_string()].into();
    let keywords3: Keywords = ["one", "two"][..].into();

    assert!(matches!(keywords1, Keywords::Single(_)));
    assert!(matches!(keywords2, Keywords::Multiple(_)));
    assert!(matches!(keywords3, Keywords::Multiple(_)));
}

#[test]
fn test_authors_from_impls() {
    let author = Author {
        name: "John Doe".to_string(),
        url: None,
        email: None,
        image: None,
    };

    let authors1: Authors = author.clone().into();
    let authors2: Authors = vec![author.clone(), author].into();

    assert!(matches!(authors1, Authors::Single(_)));
    assert!(matches!(authors2, Authors::Multiple(_)));
}

#[test]
#[cfg(feature = "json-ld")]
fn test_metadata_serialization() {
    let metadata = Metadata::new()
        .title("Test Title")
        .description("Test Description")
        .keywords(vec!["test".to_string(), "metadata".to_string()])
        .canonical("https://example.com");

    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: Metadata = serde_json::from_str(&json).unwrap();

    assert_eq!(metadata.title, deserialized.title);
    assert_eq!(metadata.description, deserialized.description);
    assert_eq!(metadata.keywords, deserialized.keywords);
    assert_eq!(metadata.canonical, deserialized.canonical);
}

#[test]
#[cfg(not(feature = "json-ld"))]
fn test_metadata_serialization_fallback() {
    let metadata = Metadata::new()
        .title("Test Title")
        .description("Test Description")
        .keywords(vec!["test".to_string(), "metadata".to_string()])
        .canonical("https://example.com");

    // When json-ld feature is disabled, we can't test serialization
    // but we can test that the metadata was created correctly
    assert_eq!(
        metadata.title,
        Some(Title::Static("Test Title".to_string()))
    );
    assert_eq!(metadata.description, Some("Test Description".to_string()));
    assert_eq!(
        metadata.keywords,
        Some(Keywords::Multiple(vec![
            "test".to_string(),
            "metadata".to_string()
        ]))
    );
    assert_eq!(metadata.canonical, Some("https://example.com".to_string()));
}

#[test]
fn test_complex_metadata_construction() {
    let author = Author {
        name: "John Doe".to_string(),
        url: Some("https://example.com/author".to_string()),
        email: Some("john@example.com".to_string()),
        image: Some("https://example.com/avatar.jpg".to_string()),
    };

    let og_image = OgImage::with_dimensions("https://example.com/og-image.jpg", 1200, 630);

    let open_graph = OpenGraph {
        title: Some("Test OG Title".to_string()),
        description: Some("Test OG Description".to_string()),
        url: Some("https://example.com".to_string()),
        r#type: Some("website".to_string()),
        site_name: Some("Test Site".to_string()),
        locale: Some("en_US".to_string()),
        images: vec![og_image],
        ..Default::default()
    };

    let twitter = Twitter {
        card: Some(TwitterCard::SummaryLargeImage),
        site: Some("@testsite".to_string()),
        creator: Some("@testcreator".to_string()),
        title: Some("Test Twitter Title".to_string()),
        description: Some("Test Twitter Description".to_string()),
        image: Some("https://example.com/twitter-image.jpg".to_string()),
        image_alt: Some("Test Twitter Image Alt".to_string()),
    };

    let viewport = Viewport {
        width: Some("device-width".to_string()),
        height: Some("device-height".to_string()),
        initial_scale: Some(1.0),
        minimum_scale: Some(0.5),
        maximum_scale: Some(2.0),
        user_scalable: Some(true),
        viewport_fit: Some("cover".to_string()),
    };

    let format_detection = FormatDetection {
        telephone: Some(true),
        email: Some(false),
        address: Some(true),
    };

    let metadata = Metadata::new()
        .title("Test Title")
        .description("Test Description")
        .keywords(vec!["test".to_string(), "metadata".to_string()])
        .authors(author)
        .robots(Robots::all())
        .open_graph(open_graph)
        .twitter(twitter)
        .canonical("https://example.com")
        .alternate("en", "https://example.com")
        .alternate("es", "https://example.com/es")
        .viewport(viewport)
        .theme_color("#ffffff")
        .color_scheme(ColorScheme::Light)
        .referrer(ReferrerPolicy::NoReferrer)
        .format_detection(format_detection);

    // Verify all fields are set correctly
    assert_eq!(
        metadata.title,
        Some(Title::Static("Test Title".to_string()))
    );
    assert_eq!(metadata.description, Some("Test Description".to_string()));
    assert_eq!(
        metadata.keywords,
        Some(Keywords::Multiple(vec![
            "test".to_string(),
            "metadata".to_string()
        ]))
    );
    assert!(metadata.authors.is_some());
    assert!(metadata.robots.is_some());
    assert!(metadata.open_graph.is_some());
    assert!(metadata.twitter.is_some());
    assert_eq!(metadata.canonical, Some("https://example.com".to_string()));
    assert!(metadata.alternates.is_some());
    assert!(metadata.viewport.is_some());
    assert_eq!(metadata.theme_color, Some("#ffffff".to_string()));
    assert_eq!(metadata.color_scheme, Some(ColorScheme::Light));
    assert_eq!(metadata.referrer, Some(ReferrerPolicy::NoReferrer));
    assert!(metadata.format_detection.is_some());

    // Verify alternate links
    let alternates = metadata.alternates.unwrap();
    assert_eq!(alternates.len(), 2);
    assert_eq!(alternates["en"].href, "https://example.com");
    assert_eq!(alternates["es"].href, "https://example.com/es");

    // Verify Open Graph
    let og = metadata.open_graph.unwrap();
    assert_eq!(og.title, Some("Test OG Title".to_string()));
    assert_eq!(og.images.len(), 1);
    assert_eq!(og.images[0].width, Some(1200));
    assert_eq!(og.images[0].height, Some(630));

    // Verify Twitter
    let twitter_data = metadata.twitter.unwrap();
    assert_eq!(twitter_data.card, Some(TwitterCard::SummaryLargeImage));
    assert_eq!(twitter_data.site, Some("@testsite".to_string()));
}

//! Tests for metadata builder pattern
//!
//! This module contains unit tests for the builder pattern implementations
//! and fluent API functionality.

use super::super::types::*;

#[test]
fn test_metadata_with_title() {
    let metadata = Metadata::with_title("Test Title");
    assert_eq!(
        metadata.title,
        Some(Title::Static("Test Title".to_string()))
    );
    assert!(metadata.description.is_none());
}

#[test]
fn test_metadata_with_title_and_description() {
    let metadata = Metadata::with_title_and_description("Test Title", "Test Description");
    assert_eq!(
        metadata.title,
        Some(Title::Static("Test Title".to_string()))
    );
    assert_eq!(metadata.description, Some("Test Description".to_string()));
}

#[test]
fn test_metadata_builder_pattern() {
    let metadata = Metadata::new()
        .title("Test Title")
        .description("Test Description")
        .keywords(vec!["test".to_string(), "metadata".to_string()])
        .canonical("https://example.com")
        .theme_color("#ffffff")
        .color_scheme(ColorScheme::Light)
        .referrer(ReferrerPolicy::NoReferrer);

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
    assert_eq!(metadata.theme_color, Some("#ffffff".to_string()));
    assert_eq!(metadata.color_scheme, Some(ColorScheme::Light));
    assert_eq!(metadata.referrer, Some(ReferrerPolicy::NoReferrer));
}

#[test]
fn test_metadata_alternate_links() {
    let metadata = Metadata::new()
        .alternate("en", "https://example.com")
        .alternate("es", "https://example.com/es")
        .alternate("fr", "https://example.com/fr");

    assert!(metadata.alternates.is_some());
    let alternates = metadata.alternates.unwrap();
    assert_eq!(alternates.len(), 3);
    assert!(alternates.contains_key("en"));
    assert!(alternates.contains_key("es"));
    assert!(alternates.contains_key("fr"));
    assert_eq!(alternates["en"].href, "https://example.com");
    assert_eq!(alternates["es"].href, "https://example.com/es");
    assert_eq!(alternates["fr"].href, "https://example.com/fr");
}

#[test]
#[cfg(feature = "json-ld")]
fn test_metadata_additional_fields() {
    let metadata = Metadata::new()
        .additional("custom_field", serde_json::json!("custom_value"))
        .additional("number_field", serde_json::json!(42))
        .additional("bool_field", serde_json::json!(true));

    assert_eq!(metadata.additional.len(), 3);
    assert_eq!(
        metadata.additional["custom_field"],
        AdditionalValue::Json(serde_json::json!("custom_value"))
    );
    assert_eq!(
        metadata.additional["number_field"],
        AdditionalValue::Json(serde_json::json!(42))
    );
    assert_eq!(
        metadata.additional["bool_field"],
        AdditionalValue::Json(serde_json::json!(true))
    );
}

#[test]
#[cfg(not(feature = "json-ld"))]
fn test_metadata_additional_fields_fallback() {
    let metadata = Metadata::new()
        .additional("custom_field", "custom_value".to_string())
        .additional("number_field", "42".to_string())
        .additional("bool_field", "true".to_string());

    assert_eq!(metadata.additional.len(), 3);
    assert_eq!(
        metadata.additional["custom_field"],
        AdditionalValue::String("custom_value".to_string())
    );
    assert_eq!(
        metadata.additional["number_field"],
        AdditionalValue::String("42".to_string())
    );
    assert_eq!(
        metadata.additional["bool_field"],
        AdditionalValue::String("true".to_string())
    );
}

// Property-based testing for builder patterns
#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn builder_fluent_api_chain(title in any::<String>(), description in any::<String>()) {
            let metadata = Metadata::builder()
                .title(Title::Static(title.clone()))
                .description(description.clone())
                .build();

            assert_eq!(metadata.title, Some(Title::Static(title)));
            assert_eq!(metadata.description, Some(description));
        }

        #[test]
        fn builder_keywords_roundtrip(keywords_vec in prop::collection::vec(any::<String>(), 1..5)) {
            let keywords = Keywords::Multiple(keywords_vec.clone());
            let metadata = Metadata::builder()
                .keywords(keywords.clone())
                .build();

            assert_eq!(metadata.keywords, Some(keywords));
        }

        #[test]
        fn builder_authors_roundtrip(name in any::<String>(), url in any::<Option<String>>(), email in any::<Option<String>>()) {
            let author = Author { name: name.clone(), url, email };
            let authors = Authors::Single(author.clone());
            let metadata = Metadata::builder()
                .authors(authors.clone())
                .build();

            assert_eq!(metadata.authors, Some(authors));
        }

        #[test]
        fn builder_robots_roundtrip(index in any::<Option<bool>>(), follow in any::<Option<bool>>()) {
            let robots = Robots {
                index,
                follow,
                google_bot: None,
                other: std::collections::HashMap::new(),
            };
            let metadata = Metadata::builder()
                .robots(robots.clone())
                .build();

            assert_eq!(metadata.robots, Some(robots));
        }
    }
}

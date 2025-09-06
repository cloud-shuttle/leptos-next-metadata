use proptest::prelude::*;
use leptos_next_metadata::metadata::*;
use leptos_next_metadata::og_image::*;

// Property-based tests for metadata operations

proptest! {
    #[test]
    fn test_merge_associativity(
        parent in arb_metadata(),
        child in arb_metadata(),
        grandchild in arb_metadata(),
    ) {
        // Test that merge is associative: (a.merge(b)).merge(c) == a.merge(b.merge(c))
        let left = grandchild.clone().merge(child.clone().merge(parent.clone()));
        let right = grandchild.clone().merge(child.clone()).merge(parent.clone());

        prop_assert_eq!(left, right);
    }

    #[test]
    fn test_merge_identity(
        metadata in arb_metadata(),
    ) {
        // Test that merging with default is identity: a.merge(default) == a
        let default_metadata = Metadata::default();
        let result = metadata.clone().merge(default_metadata);

        prop_assert_eq!(result, metadata);
    }

    #[test]
    fn test_title_resolution_never_panics(
        title in arb_title(),
        segment in prop::option::of(any::<String>()),
    ) {
        // Title resolution should never panic, regardless of input
        let _ = title.resolve(segment.as_deref());
    }

    #[test]
    fn test_title_resolution_consistency(
        title in arb_title(),
        segment in prop::option::of(any::<String>()),
    ) {
        // Title resolution should be deterministic
        let result1 = title.resolve(segment.as_deref());
        let result2 = title.resolve(segment.as_deref());

        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn test_static_title_ignores_segment(
        title_text in any::<String>(),
        segment in prop::option::of(any::<String>()),
    ) {
        let title = Title::Static(title_text.clone());
        let result = title.resolve(segment.as_deref());

        prop_assert_eq!(result, title_text);
    }

    #[test]
    fn test_absolute_title_ignores_segment(
        title_text in any::<String>(),
        segment in prop::option::of(any::<String>()),
    ) {
        let title = Title::Absolute(title_text.clone());
        let result = title.resolve(segment.as_deref());

        prop_assert_eq!(result, title_text);
    }

    #[test]
    fn test_template_title_uses_default_when_no_segment(
        template in any::<String>(),
        default in any::<String>(),
    ) {
        let title = Title::Template { template, default: default.clone() };
        let result = title.resolve(None);

        prop_assert_eq!(result, default);
    }

    #[test]
    fn test_cache_key_deterministic(
        params1 in arb_og_params(),
        params2 in arb_og_params(),
    ) {
        // Cache keys should be deterministic
        let key1a = params1.cache_key();
        let key1b = params1.cache_key();
        let key2 = params2.cache_key();

        prop_assert_eq!(key1a, key1b);
        if params1 != params2 {
            prop_assert_ne!(key1a, key2);
        }
    }

    #[test]
    fn test_og_image_url_resolution(
        url in any::<String>(),
        base_url in any::<String>(),
    ) {
        let og_image = OgImage::new(&url);
        let _ = og_image.resolve_url(&base_url); // Should not panic
    }

    #[test]
    fn test_metadata_serialization_roundtrip(
        metadata in arb_metadata(),
    ) {
        // Test that metadata can be serialized and deserialized correctly
        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: Metadata = serde_json::from_str(&serialized).unwrap();

        prop_assert_eq!(metadata, deserialized);
    }

    #[test]
    fn test_keywords_ordering_preserved(
        keywords in prop::collection::vec(any::<String>(), 0..20),
    ) {
        let metadata = Metadata {
            keywords: keywords.clone(),
            ..Default::default()
        };

        prop_assert_eq!(metadata.keywords, keywords);
    }

    #[test]
    fn test_merge_preserves_child_priority(
        parent_title in any::<String>(),
        child_title in any::<String>(),
        parent_desc in any::<String>(),
    ) {
        let parent = Metadata {
            title: Some(Title::Static(parent_title)),
            description: Some(parent_desc.clone()),
            ..Default::default()
        };

        let child = Metadata {
            title: Some(Title::Static(child_title.clone())),
            description: None, // Child has no description
            ..Default::default()
        };

        let merged = child.merge(parent);

        // Child's title should take precedence
        match &merged.title {
            Some(Title::Static(title)) => prop_assert_eq!(title, &child_title),
            _ => prop_assert!(false, "Expected static title"),
        }

        // Parent's description should be used since child has none
        prop_assert_eq!(merged.description.as_deref(), Some(parent_desc.as_str()));
    }

    #[test]
    fn test_validation_consistency(
        metadata in arb_metadata(),
    ) {
        // Validation should be consistent
        let result1 = metadata.validate();
        let result2 = metadata.validate();

        prop_assert_eq!(result1.is_valid, result2.is_valid);
        prop_assert_eq!(result1.errors, result2.errors);
    }

    #[test]
    fn test_icon_rel_attribute_consistency(
        url in any::<String>(),
        rel in prop::option::of(any::<String>()),
    ) {
        let icon = Icon {
            url,
            rel: rel.clone(),
            ..Default::default()
        };

        let result = icon.get_rel();

        // Should return provided rel or default
        if let Some(r) = rel {
            prop_assert_eq!(result, r);
        } else {
            prop_assert_eq!(result, "icon".to_string());
        }
    }

    #[test]
    fn test_robots_directives_generation(
        index in any::<bool>(),
        follow in any::<bool>(),
        noarchive in any::<bool>(),
    ) {
        let robots = Robots {
            index: Some(index),
            follow: Some(follow),
            noarchive: Some(noarchive),
            ..Default::default()
        };

        let directives = robots.to_directives();

        // Check that directives contain expected values
        if index {
            prop_assert!(directives.contains("index"));
        } else {
            prop_assert!(directives.contains("noindex"));
        }

        if follow {
            prop_assert!(directives.contains("follow"));
        } else {
            prop_assert!(directives.contains("nofollow"));
        }

        if noarchive {
            prop_assert!(directives.contains("noarchive"));
        }
    }
}

// Property test generators

fn arb_title() -> impl Strategy<Value = Title> {
    prop_oneof![
        any::<String>().prop_map(Title::Static),
        (any::<String>(), any::<String>()).prop_map(|(template, default)| {
            Title::Template { template, default }
        }),
        any::<String>().prop_map(Title::Absolute),
    ]
}

fn arb_og_image() -> impl Strategy<Value = OgImage> {
    (
        any::<String>(),
        prop::option::of(1u32..5000),
        prop::option::of(1u32..5000),
        prop::option::of(any::<String>()),
    ).prop_map(|(url, width, height, alt)| {
        OgImage {
            url,
            width,
            height,
            alt,
            ..Default::default()
        }
    })
}

fn arb_open_graph() -> impl Strategy<Value = OpenGraph> {
    (
        prop::option::of(any::<String>()),
        prop::option::of(any::<String>()),
        prop::collection::vec(arb_og_image(), 0..5),
        prop::option::of(any::<String>()),
        prop::option::of(any::<String>()),
    ).prop_map(|(title, description, images, og_type, url)| {
        OpenGraph {
            title,
            description,
            images,
            og_type,
            url,
            ..Default::default()
        }
    })
}

fn arb_metadata() -> impl Strategy<Value = Metadata> {
    (
        prop::option::of(arb_title()),
        prop::option::of(any::<String>()),
        prop::collection::vec(any::<String>(), 0..10),
        prop::option::of(arb_open_graph()),
    ).prop_map(|(title, description, keywords, open_graph)| {
        Metadata {
            title,
            description,
            keywords,
            open_graph,
            ..Default::default()
        }
    })
}

fn arb_og_params() -> impl Strategy<Value = OgImageParams> {
    (
        any::<String>(),
        any::<String>(),
        any::<String>(),
        100u32..2000,
        100u32..2000,
    ).prop_map(|(template, title, description, width, height)| {
        OgImageParams {
            template,
            data: liquid::object!({
                "title": title,
                "description": description,
            }),
            size: (width, height),
        }
    })
}

// Edge case testing

proptest! {
    #[test]
    fn test_empty_strings_handling(
        empty_title in prop::regex::string_regex("").unwrap(),
        empty_desc in prop::regex::string_regex("").unwrap(),
    ) {
        let metadata = Metadata {
            title: Some(Title::Static(empty_title)),
            description: Some(empty_desc),
            ..Default::default()
        };

        // Should handle empty strings without panicking
        let _ = metadata.validate();
        let _ = serde_json::to_string(&metadata);
    }

    #[test]
    fn test_very_long_strings(
        long_title in prop::regex::string_regex(".{1000,2000}").unwrap(),
        long_desc in prop::regex::string_regex(".{2000,5000}").unwrap(),
    ) {
        let metadata = Metadata {
            title: Some(Title::Static(long_title)),
            description: Some(long_desc),
            ..Default::default()
        };

        // Should handle very long strings without panicking
        let _ = metadata.validate();
        let _ = serde_json::to_string(&metadata);
    }

    #[test]
    fn test_unicode_handling(
        unicode_title in "\\PC{100}",
        unicode_desc in "\\PC{200}",
    ) {
        let metadata = Metadata {
            title: Some(Title::Static(unicode_title)),
            description: Some(unicode_desc),
            ..Default::default()
        };

        // Should handle unicode correctly
        let _ = metadata.validate();
        let serialized = serde_json::to_string(&metadata).unwrap();
        let _: Metadata = serde_json::from_str(&serialized).unwrap();
    }

    #[test]
    fn test_special_characters_in_urls(
        url_with_special_chars in r"https?://[a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,}(/[^?\s]*)?(\?[^#\s]*)?(#[^\s]*)?",
    ) {
        let og_image = OgImage::new(&url_with_special_chars);
        let base_url = "https://example.com";

        // URL resolution should handle special characters
        let _ = og_image.resolve_url(base_url);
    }

    #[test]
    fn test_template_with_special_characters(
        template in r"[^%]*%s[^%]*",
        segment in any::<String>(),
        default in any::<String>(),
    ) {
        let title = Title::Template { template, default };

        // Template resolution should handle special characters
        let _ = title.resolve(Some(&segment));
        let _ = title.resolve(None);
    }
}

// Performance property tests

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))] // Fewer cases for performance tests

    #[test]
    fn test_merge_performance_scales(
        metadatas in prop::collection::vec(arb_metadata(), 1..20),
    ) {
        let start = std::time::Instant::now();

        // Chain multiple merges
        let result = metadatas.into_iter().reduce(|acc, meta| meta.merge(acc));

        let duration = start.elapsed();

        // Should complete within reasonable time
        prop_assert!(duration.as_millis() < 100, "Merge took too long: {:?}", duration);
        prop_assert!(result.is_some());
    }

    #[test]
    fn test_validation_performance(
        large_metadata in (
            arb_title(),
            any::<String>(),
            prop::collection::vec(any::<String>(), 50..200), // Large keyword list
        ).prop_map(|(title, desc, keywords)| {
            Metadata {
                title: Some(title),
                description: Some(desc),
                keywords,
                ..Default::default()
            }
        }),
    ) {
        let start = std::time::Instant::now();

        let _ = large_metadata.validate();

        let duration = start.elapsed();

        // Validation should be fast even with large data
        prop_assert!(duration.as_millis() < 50, "Validation took too long: {:?}", duration);
    }
}

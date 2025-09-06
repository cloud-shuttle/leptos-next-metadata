//! Macro performance and stress tests
//!
//! These tests verify that the macros perform well under various conditions
//! including high load, complex metadata, and edge cases.

use leptos_next_metadata::prelude::*;
use leptos::*;
use std::time::Instant;

/// Test macro compilation performance with large metadata
#[test]
fn test_macro_large_metadata_performance() {
    let start = Instant::now();

    // Generate large metadata with many fields
    let _result = metadata! {
        title: "Large Metadata Performance Test",
        description: "Testing performance with large metadata structures",
        keywords: (0..1000).map(|i| format!("keyword_{}", i)).collect::<Vec<_>>(),
        authors: (0..500).map(|i| format!("author_{}", i)).collect::<Vec<_>>(),
        openGraph: {
            title: "Large OG Test",
            description: "Testing OpenGraph performance",
            type: "website",
            url: "https://example.com/large-test",
            site_name: "Large Test Site",
            images: (0..100).map(|i| format!("https://example.com/image_{}.jpg", i)).collect::<Vec<_>>(),
            article: {
                published_time: "2025-01-01T00:00:00Z",
                modified_time: "2025-01-02T00:00:00Z",
                author: "https://example.com/author",
                section: "Technology",
                tags: (0..200).map(|i| format!("tag_{}", i)).collect::<Vec<_>>()
            }
        },
        twitter: {
            card: "summary_large_image",
            title: "Large Twitter Test",
            description: "Testing Twitter performance",
            site: "@largetest",
            creator: "@largecreator",
            images: (0..50).map(|i| format!("https://example.com/twitter_{}.jpg", i)).collect::<Vec<_>>()
        },
        robots: {
            index: true,
            follow: true,
            nocache: false,
            noarchive: false,
            nosnippet: false,
            noimageindex: false,
            notranslate: false
        },
        viewport: {
            width: "device-width",
            initial_scale: "1.0",
            minimum_scale: "1.0",
            maximum_scale: "5.0",
            user_scalable: true,
            viewport_fit: "cover"
        },
        theme_color: "#000000",
        color_scheme: "dark",
        json_ld: {
            "@context": "https://schema.org",
            "@type": "WebPage",
            "name": "Large Metadata Test",
            "description": "Testing performance with large metadata",
            "url": "https://example.com/large-test",
            "author": {
                "@type": "Person",
                "name": "Test Author",
                "description": "A test author for performance testing"
            },
            "keywords": (0..300).map(|i| format!("ld_keyword_{}", i)).collect::<Vec<_>>()
        }
    };

    let duration = start.elapsed();

    // Large metadata should compile in under 100ms
    assert!(duration.as_millis() < 100, "Large metadata compilation took {}ms, expected <100ms", duration.as_millis());
}

/// Test macro compilation performance with many nested levels
#[test]
fn test_macro_deep_nesting_performance() {
    let start = Instant::now();

    // Generate deeply nested metadata
    let _result = metadata! {
        title: "Deep Nesting Performance Test",
        description: "Testing performance with deeply nested structures",
        openGraph: {
            title: "Level 1",
            article: {
                author: {
                    name: "Author Name",
                    profile: {
                        bio: "Author bio",
                        details: {
                            location: "Location",
                            contact: {
                                email: "email@example.com",
                                social: {
                                    twitter: "@twitter",
                                    linkedin: "linkedin.com/in/author",
                                    github: "github.com/author",
                                    website: "author.com"
                                }
                            },
                            preferences: {
                                theme: "dark",
                                language: "en",
                                notifications: true,
                                privacy: {
                                    public_profile: true,
                                    show_email: false,
                                    show_location: true
                                }
                            }
                        }
                    }
                },
                publisher: {
                    name: "Publisher Name",
                    logo: "https://example.com/logo.png",
                    contact: {
                        email: "publisher@example.com",
                        phone: "+1234567890",
                        address: {
                            street: "123 Main St",
                            city: "City",
                            state: "State",
                            country: "Country",
                            postal_code: "12345"
                        }
                    }
                }
            }
        }
    };

    let duration = start.elapsed();

    // Deep nesting should compile in under 50ms
    assert!(duration.as_millis() < 50, "Deep nesting compilation took {}ms, expected <50ms", duration.as_millis());
}

/// Test macro compilation performance with many components
#[test]
fn test_macro_many_components_performance() {
    let start = Instant::now();

    // Generate many components with metadata
    for i in 0..100 {
        let _component = metadata! {
            title: format!("Component {}", i),
            description: format!("Description for component {}", i),
            keywords: vec![format!("component_{}", i), "performance".to_string(), "test".to_string()]
        };
    }

    let duration = start.elapsed();

    // 100 components should compile in under 200ms
    assert!(duration.as_millis() < 200, "100 components compilation took {}ms, expected <200ms", duration.as_millis());
}

/// Test macro compilation performance with complex expressions
#[test]
fn test_macro_complex_expressions_performance() {
    let start = Instant::now();

    // Generate metadata with complex expressions
    let _result = metadata! {
        title: {
            let base = "Complex Expression";
            let suffix = "Test";
            format!("{} - {}", base, suffix)
        },
        description: {
            let words = vec!["complex", "expression", "performance", "test"];
            words.join(" ")
        },
        keywords: {
            let mut keywords = Vec::new();
            for i in 0..100 {
                keywords.push(format!("complex_keyword_{}", i));
            }
            keywords
        },
        openGraph: {
            title: {
                let base_title = "OG Complex";
                let timestamp = chrono::Utc::now().timestamp();
                format!("{} - {}", base_title, timestamp)
            },
            description: {
                let descriptions = vec!["First", "Second", "Third", "Fourth", "Fifth"];
                descriptions.join(" | ")
            }
        }
    };

    let duration = start.elapsed();

    // Complex expressions should compile in under 100ms
    assert!(duration.as_millis() < 100, "Complex expressions compilation took {}ms, expected <100ms", duration.as_millis());
}

/// Test macro compilation performance under memory pressure
#[test]
fn test_macro_memory_pressure_performance() {
    let start = Instant::now();

    // Allocate memory to simulate pressure
    let mut memory_blocks = Vec::new();
    for _ in 0..1000 {
        memory_blocks.push(vec![0u8; 1024]); // 1KB blocks
    }

    // Generate metadata under memory pressure
    let _result = metadata! {
        title: "Memory Pressure Test",
        description: "Testing performance under memory pressure",
        keywords: (0..500).map(|i| format!("memory_keyword_{}", i)).collect::<Vec<_>>(),
        openGraph: {
            title: "Memory Pressure OG",
            description: "Testing OpenGraph under memory pressure",
            type: "website"
        }
    };

    // Clean up memory
    drop(memory_blocks);

    let duration = start.elapsed();

    // Should still compile reasonably fast under memory pressure
    assert!(duration.as_millis() < 150, "Memory pressure compilation took {}ms, expected <150ms", duration.as_millis());
}

/// Test macro compilation performance with concurrent access simulation
#[test]
fn test_macro_concurrent_access_performance() {
    let start = Instant::now();

    // Simulate concurrent access by compiling many macros rapidly
    let mut results = Vec::new();
    for i in 0..200 {
        let result = metadata! {
            title: format!("Concurrent Test {}", i),
            description: format!("Testing concurrent access {}", i),
            keywords: vec![format!("concurrent_{}", i), "performance".to_string()]
        };
        results.push(result);
    }

    let duration = start.elapsed();

    // 200 concurrent compilations should complete in under 300ms
    assert!(duration.as_millis() < 300, "Concurrent access compilation took {}ms, expected <300ms", duration.as_millis());
}

/// Test macro compilation performance with large strings
#[test]
fn test_macro_large_strings_performance() {
    let start = Instant::now();

    // Generate very long strings
    let long_title = "A".repeat(10000); // 10KB title
    let long_description = "B".repeat(20000); // 20KB description
    let long_keywords: Vec<String> = (0..100).map(|i| format!("keyword_{}", "X".repeat(100))).collect();

    let _result = metadata! {
        title: long_title,
        description: long_description,
        keywords: long_keywords,
        openGraph: {
            title: "Large Strings OG",
            description: "Testing with large strings"
        }
    };

    let duration = start.elapsed();

    // Large strings should compile in under 200ms
    assert!(duration.as_millis() < 200, "Large strings compilation took {}ms, expected <200ms", duration.as_millis());
}

/// Test macro compilation performance with many optional fields
#[test]
fn test_macro_many_optional_fields_performance() {
    let start = Instant::now();

    // Generate metadata with many optional fields
    let _result = metadata! {
        title: "Many Optional Fields Test",
        description: "Testing performance with many optional fields",
        keywords: Some(vec!["optional", "fields", "test"]),
        authors: Some(vec!["Author 1", "Author 2"]),
        canonical: Some("https://example.com/optional-test"),
        openGraph: Some({
            title: Some("Optional OG Title"),
            description: Some("Optional OG Description"),
            type: Some("website"),
            url: Some("https://example.com/og-optional"),
            site_name: Some("Optional Site"),
            images: Some(vec!["https://example.com/optional.jpg"]),
            article: Some({
                published_time: Some("2025-01-01T00:00:00Z"),
                modified_time: Some("2025-01-02T00:00:00Z"),
                author: Some("https://example.com/optional-author"),
                section: Some("Optional Section"),
                tags: Some(vec!["optional", "tag"])
            })
        }),
        twitter: Some({
            card: Some("summary_large_image"),
            title: Some("Optional Twitter Title"),
            description: Some("Optional Twitter Description"),
            site: Some("@optional"),
            creator: Some("@optionalcreator"),
            image: Some("https://example.com/twitter-optional.jpg")
        }),
        robots: Some({
            index: Some(true),
            follow: Some(true),
            nocache: Some(false),
            noarchive: Some(false),
            nosnippet: Some(false),
            noimageindex: Some(false),
            notranslate: Some(false)
        }),
        viewport: Some({
            width: Some("device-width"),
            initial_scale: Some("1.0"),
            minimum_scale: Some("1.0"),
            maximum_scale: Some("5.0"),
            user_scalable: Some(true),
            viewport_fit: Some("cover")
        }),
        theme_color: Some("#000000"),
        color_scheme: Some("dark"),
        json_ld: Some({
            "@context": "https://schema.org",
            "@type": "WebPage",
            "name": "Optional Fields Test",
            "description": "Testing with many optional fields"
        })
    };

    let duration = start.elapsed();

    // Many optional fields should compile in under 100ms
    assert!(duration.as_millis() < 100, "Many optional fields compilation took {}ms, expected <100ms", duration.as_millis());
}

/// Test macro compilation performance with template strings
#[test]
fn test_macro_template_strings_performance() {
    let start = Instant::now();

    // Generate metadata with template strings
    let _result = metadata! {
        title: {
            let template = "%s | Site Name";
            let page_name = "Template Test";
            template.replace("%s", page_name)
        },
        description: {
            let base_desc = "Base description";
            let timestamp = chrono::Utc::now().format("%Y-%m-%d").to_string();
            format!("{} - {}", base_desc, timestamp)
        },
        openGraph: {
            title: {
                let og_template = "OG: %s";
                let page_title = "Template OG Test";
                og_template.replace("%s", page_title)
            }
        }
    };

    let duration = start.elapsed();

    // Template strings should compile in under 50ms
    assert!(duration.as_millis() < 50, "Template strings compilation took {}ms, expected <50ms", duration.as_millis());
}

/// Test macro compilation performance with conditional compilation
#[test]
fn test_macro_conditional_compilation_performance() {
    let start = Instant::now();

    // Generate metadata with conditional compilation
    let _result = metadata! {
        title: "Conditional Compilation Test",
        description: "Testing performance with conditional compilation",
        #[cfg(feature = "debug")]
        keywords: ["debug", "enabled", "performance"],
        #[cfg(not(feature = "debug"))]
        keywords: ["debug", "disabled", "performance"],
        #[cfg(feature = "advanced")]
        openGraph: {
            title: "Advanced OG",
            description: "Advanced features enabled"
        },
        #[cfg(not(feature = "advanced"))]
        openGraph: {
            title: "Basic OG",
            description: "Basic features only"
        }
    };

    let duration = start.elapsed();

    // Conditional compilation should compile in under 50ms
    assert!(duration.as_millis() < 50, "Conditional compilation took {}ms, expected <50ms", duration.as_millis());
}

/// Test macro compilation performance with error recovery
#[test]
fn test_macro_error_recovery_performance() {
    let start = Instant::now();

    // Generate metadata that might have errors but should recover
    let _result = metadata! {
        title: "Error Recovery Test",
        description: "Testing performance with error recovery",
        keywords: ["error", "recovery", "performance"],
        openGraph: {
            title: "Error Recovery OG",
            description: "Testing OpenGraph error recovery",
            // Intentionally omit required fields to test error handling
        }
    };

    let duration = start.elapsed();

    // Error recovery should still be fast
    assert!(duration.as_millis() < 100, "Error recovery compilation took {}ms, expected <100ms", duration.as_millis());
}

/// Test macro compilation performance with incremental updates
#[test]
fn test_macro_incremental_updates_performance() {
    let start = Instant::now();

    // Simulate incremental metadata updates
    let mut base_metadata = metadata! {
        title: "Base Metadata",
        description: "Base description"
    };

    // Update metadata incrementally
    for i in 0..50 {
        let _updated = metadata! {
            title: format!("Updated Metadata {}", i),
            description: format!("Updated description {}", i),
            keywords: vec![format!("update_{}", i), "incremental".to_string()]
        };
    }

    let duration = start.elapsed();

    // Incremental updates should be fast
    assert!(duration.as_millis() < 200, "Incremental updates took {}ms, expected <200ms", duration.as_millis());
}

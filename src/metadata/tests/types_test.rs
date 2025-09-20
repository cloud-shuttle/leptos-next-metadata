//! Tests for core metadata types
//!
//! This module contains unit tests for the basic metadata types and their
//! fundamental functionality.

use super::super::types::*;
use chrono::Utc;

#[test]
fn test_metadata_creation() {
    let metadata = Metadata::new();
    assert!(metadata.title.is_none());
    assert!(metadata.description.is_none());
    assert!(metadata.keywords.is_none());
    assert!(metadata.authors.is_none());
    assert!(metadata.robots.is_none());
    assert!(metadata.open_graph.is_none());
    assert!(metadata.twitter.is_none());
    assert!(metadata.json_ld.is_none());
    assert!(metadata.canonical.is_none());
    assert!(metadata.alternates.is_none());
    assert!(metadata.viewport.is_none());
    assert!(metadata.theme_color.is_none());
    assert!(metadata.color_scheme.is_none());
    assert!(metadata.referrer.is_none());
    assert!(metadata.format_detection.is_none());
    assert!(metadata.additional.is_empty());
}

#[test]
fn test_title_enum() {
    let static_title = Title::Static("Static Title".to_string());
    let template_title = Title::Template {
        template: "%s | My Site".to_string(),
        default: "My Site".to_string(),
    };

    match static_title {
        Title::Static(s) => assert_eq!(s, "Static Title"),
        _ => panic!("Expected Static variant"),
    }

    match template_title {
        Title::Template { template, default } => {
            assert_eq!(template, "%s | My Site");
            assert_eq!(default, "My Site");
        }
        _ => panic!("Expected Template variant"),
    }
}

#[test]
fn test_keywords_enum() {
    let single_keyword = Keywords::Single("single".to_string());
    let multiple_keywords = Keywords::Multiple(vec!["one".to_string(), "two".to_string()]);

    match single_keyword {
        Keywords::Single(s) => assert_eq!(s, "single"),
        _ => panic!("Expected Single variant"),
    }

    match multiple_keywords {
        Keywords::Multiple(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], "one");
            assert_eq!(v[1], "two");
        }
        _ => panic!("Expected Multiple variant"),
    }
}

#[test]
fn test_author_struct() {
    let author = Author {
        name: "John Doe".to_string(),
        url: Some("https://example.com".to_string()),
        email: Some("john@example.com".to_string()),
        image: Some("https://example.com/avatar.jpg".to_string()),
    };

    assert_eq!(author.name, "John Doe");
    assert_eq!(author.url, Some("https://example.com".to_string()));
    assert_eq!(author.email, Some("john@example.com".to_string()));
    assert_eq!(
        author.image,
        Some("https://example.com/avatar.jpg".to_string())
    );
}

#[test]
fn test_authors_enum() {
    let single_author = Authors::Single(Author {
        name: "John Doe".to_string(),
        url: None,
        email: None,
        image: None,
    });

    let multiple_authors = Authors::Multiple(vec![
        Author {
            name: "John Doe".to_string(),
            url: None,
            email: None,
            image: None,
        },
        Author {
            name: "Jane Smith".to_string(),
            url: None,
            email: None,
            image: None,
        },
    ]);

    match single_author {
        Authors::Single(a) => assert_eq!(a.name, "John Doe"),
        _ => panic!("Expected Single variant"),
    }

    match multiple_authors {
        Authors::Multiple(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0].name, "John Doe");
            assert_eq!(v[1].name, "Jane Smith");
        }
        _ => panic!("Expected Multiple variant"),
    }
}

#[test]
fn test_robots_struct() {
    let robots = Robots::all();
    assert!(robots.index);
    assert!(robots.follow);
    assert!(robots.archive);
    assert!(robots.image_index);
    assert!(robots.snippet);
    assert!(robots.crawl_delay.is_none());
    assert!(robots.additional.is_empty());

    let robots_none = Robots::none();
    assert!(!robots_none.index);
    assert!(!robots_none.follow);
    assert!(!robots_none.archive);
    assert!(!robots_none.image_index);
    assert!(!robots_none.snippet);

    let robots_noindex = Robots::noindex();
    assert!(!robots_noindex.index);
    assert!(robots_noindex.follow);
    assert!(!robots_noindex.archive);
    assert!(!robots_noindex.image_index);
    assert!(!robots_noindex.snippet);
}

#[test]
fn test_og_image_struct() {
    let og_image = OgImage::new("https://example.com/image.jpg");
    assert_eq!(og_image.url, "https://example.com/image.jpg");
    assert!(og_image.width.is_none());
    assert!(og_image.height.is_none());
    assert!(og_image.alt.is_none());
    assert!(og_image.r#type.is_none());

    let og_image_with_dimensions =
        OgImage::with_dimensions("https://example.com/image.jpg", 1200, 630);
    assert_eq!(
        og_image_with_dimensions.url,
        "https://example.com/image.jpg"
    );
    assert_eq!(og_image_with_dimensions.width, Some(1200));
    assert_eq!(og_image_with_dimensions.height, Some(630));
}

#[test]
fn test_open_graph_struct() {
    let og = OpenGraph {
        title: Some("Test Title".to_string()),
        description: Some("Test Description".to_string()),
        url: Some("https://example.com".to_string()),
        r#type: Some("website".to_string()),
        site_name: Some("Test Site".to_string()),
        locale: Some("en_US".to_string()),
        images: vec![OgImage::new("https://example.com/image.jpg")],
        ..Default::default()
    };

    assert_eq!(og.title, Some("Test Title".to_string()));
    assert_eq!(og.description, Some("Test Description".to_string()));
    assert_eq!(og.url, Some("https://example.com".to_string()));
    assert_eq!(og.r#type, Some("website".to_string()));
    assert_eq!(og.site_name, Some("Test Site".to_string()));
    assert_eq!(og.locale, Some("en_US".to_string()));
    assert_eq!(og.images.len(), 1);
    assert_eq!(og.images[0].url, "https://example.com/image.jpg");
}

#[test]
fn test_twitter_struct() {
    let twitter = Twitter {
        card: Some(TwitterCard::SummaryLargeImage),
        site: Some("@testsite".to_string()),
        creator: Some("@testcreator".to_string()),
        title: Some("Test Title".to_string()),
        description: Some("Test Description".to_string()),
        image: Some("https://example.com/image.jpg".to_string()),
        image_alt: Some("Test Image Alt".to_string()),
    };

    assert_eq!(twitter.card, Some(TwitterCard::SummaryLargeImage));
    assert_eq!(twitter.site, Some("@testsite".to_string()));
    assert_eq!(twitter.creator, Some("@testcreator".to_string()));
    assert_eq!(twitter.title, Some("Test Title".to_string()));
    assert_eq!(twitter.description, Some("Test Description".to_string()));
    assert_eq!(
        twitter.image,
        Some("https://example.com/image.jpg".to_string())
    );
    assert_eq!(twitter.image_alt, Some("Test Image Alt".to_string()));
}

#[test]
fn test_twitter_card_enum() {
    let summary = TwitterCard::Summary;
    let summary_large = TwitterCard::SummaryLargeImage;
    let app = TwitterCard::App;
    let player = TwitterCard::Player;

    assert_ne!(summary, summary_large);
    assert_ne!(summary, app);
    assert_ne!(summary, player);
    assert_ne!(summary_large, app);
    assert_ne!(summary_large, player);
    assert_ne!(app, player);
}

#[test]
fn test_article_struct() {
    let now = Utc::now();
    let article = Article {
        published_time: Some(now),
        modified_time: Some(now),
        expiration_time: None,
        author: Some("https://example.com/author".to_string()),
        section: Some("Technology".to_string()),
        tags: Some(vec!["rust".to_string(), "leptos".to_string()]),
    };

    assert_eq!(article.published_time, Some(now));
    assert_eq!(article.modified_time, Some(now));
    assert!(article.expiration_time.is_none());
    assert_eq!(
        article.author,
        Some("https://example.com/author".to_string())
    );
    assert_eq!(article.section, Some("Technology".to_string()));
    assert_eq!(
        article.tags,
        Some(vec!["rust".to_string(), "leptos".to_string()])
    );
}

#[test]
fn test_profile_struct() {
    let profile = Profile {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        username: Some("johndoe".to_string()),
        gender: Some("male".to_string()),
    };

    assert_eq!(profile.first_name, Some("John".to_string()));
    assert_eq!(profile.last_name, Some("Doe".to_string()));
    assert_eq!(profile.username, Some("johndoe".to_string()));
    assert_eq!(profile.gender, Some("male".to_string()));
}

#[test]
fn test_book_struct() {
    let now = Utc::now();
    let book = Book {
        author: Some("John Doe".to_string()),
        isbn: Some("978-0-123456-47-2".to_string()),
        release_date: Some(now),
        tags: Some(vec!["fiction".to_string(), "adventure".to_string()]),
    };

    assert_eq!(book.author, Some("John Doe".to_string()));
    assert_eq!(book.isbn, Some("978-0-123456-47-2".to_string()));
    assert_eq!(book.release_date, Some(now));
    assert_eq!(
        book.tags,
        Some(vec!["fiction".to_string(), "adventure".to_string()])
    );
}

#[test]
fn test_alternate_link_struct() {
    let alternate = AlternateLink {
        href: "https://example.com/es".to_string(),
        hreflang: "es".to_string(),
        media: Some("only screen and (max-width: 640px)".to_string()),
    };

    assert_eq!(alternate.href, "https://example.com/es".to_string());
    assert_eq!(alternate.hreflang, "es".to_string());
    assert_eq!(
        alternate.media,
        Some("only screen and (max-width: 640px)".to_string())
    );
}

#[test]
fn test_viewport_struct() {
    let viewport = Viewport {
        width: Some("device-width".to_string()),
        height: Some("device-height".to_string()),
        initial_scale: Some(1.0),
        minimum_scale: Some(0.5),
        maximum_scale: Some(2.0),
        user_scalable: Some(true),
        viewport_fit: Some("cover".to_string()),
    };

    assert_eq!(viewport.width, Some("device-width".to_string()));
    assert_eq!(viewport.height, Some("device-height".to_string()));
    assert_eq!(viewport.initial_scale, Some(1.0));
    assert_eq!(viewport.minimum_scale, Some(0.5));
    assert_eq!(viewport.maximum_scale, Some(2.0));
    assert_eq!(viewport.user_scalable, Some(true));
    assert_eq!(viewport.viewport_fit, Some("cover".to_string()));
}

#[test]
fn test_color_scheme_enum() {
    let light = ColorScheme::Light;
    let dark = ColorScheme::Dark;
    let auto = ColorScheme::Auto;

    assert_ne!(light, dark);
    assert_ne!(light, auto);
    assert_ne!(dark, auto);
}

#[test]
fn test_referrer_policy_enum() {
    let no_referrer = ReferrerPolicy::NoReferrer;
    let no_referrer_when_downgrade = ReferrerPolicy::NoReferrerWhenDowngrade;
    let origin = ReferrerPolicy::Origin;
    let origin_when_cross_origin = ReferrerPolicy::OriginWhenCrossOrigin;
    let same_origin = ReferrerPolicy::SameOrigin;
    let strict_origin = ReferrerPolicy::StrictOrigin;
    let strict_origin_when_cross_origin = ReferrerPolicy::StrictOriginWhenCrossOrigin;
    let unsafe_url = ReferrerPolicy::UnsafeUrl;

    assert_ne!(no_referrer, no_referrer_when_downgrade);
    assert_ne!(no_referrer, origin);
    assert_ne!(origin, origin_when_cross_origin);
    assert_ne!(same_origin, strict_origin);
    assert_ne!(strict_origin_when_cross_origin, unsafe_url);
}

#[test]
fn test_format_detection_struct() {
    let format_detection = FormatDetection {
        telephone: Some(true),
        email: Some(false),
        address: Some(true),
    };

    assert_eq!(format_detection.telephone, Some(true));
    assert_eq!(format_detection.email, Some(false));
    assert_eq!(format_detection.address, Some(true));
}

// Property-based testing
#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn metadata_serde_roundtrip(title in any::<String>()) {
            let original = Metadata {
                title: Some(Title::Static(title)),
                description: Some("Test Description".to_string()),
                ..Default::default()
            };

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Metadata = serde_json::from_str(&json).unwrap();

            assert_eq!(original.title, deserialized.title);
            assert_eq!(original.description, deserialized.description);
        }

        #[test]
        fn title_serde_roundtrip(title_str in any::<String>()) {
            let original = Title::Static(title_str.clone());

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Title = serde_json::from_str(&json).unwrap();

            assert_eq!(original, deserialized);
        }

        #[test]
        fn keywords_serde_roundtrip(keywords_vec in prop::collection::vec(any::<String>(), 0..10)) {
            let original = if keywords_vec.len() == 1 {
                Keywords::Single(keywords_vec[0].clone())
            } else {
                Keywords::Multiple(keywords_vec.clone())
            };

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Keywords = serde_json::from_str(&json).unwrap();

            assert_eq!(original, deserialized);
        }

        #[test]
        fn author_serde_roundtrip(name in any::<String>(), url in any::<Option<String>>(), email in any::<Option<String>>()) {
            let original = Author {
                name,
                url,
                email,
            };

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Author = serde_json::from_str(&json).unwrap();

            assert_eq!(original, deserialized);
        }

        #[test]
        fn robots_serde_roundtrip(index in any::<Option<bool>>(), follow in any::<Option<bool>>()) {
            let original = Robots {
                index,
                follow,
                google_bot: None,
                other: std::collections::HashMap::new(),
            };

            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Robots = serde_json::from_str(&json).unwrap();

            assert_eq!(original, deserialized);
        }
    }
}

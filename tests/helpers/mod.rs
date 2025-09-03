use leptos_next_metadata::prelude::*;
use std::collections::HashMap;

/// Create a basic test metadata instance
pub fn create_test_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Test Title".into())),
        description: Some("Test description for unit tests".into()),
        keywords: vec!["test".into(), "rust".into(), "leptos".into()],
        ..Default::default()
    }
}

/// Create metadata with complex nested structures for testing
pub fn create_complex_test_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Template {
            template: "%s | Test Site".into(),
            default: "Test Site".into(),
        }),
        description: Some("Complex test metadata with all fields populated".into()),
        keywords: vec!["test".into(), "complex".into(), "metadata".into(), "rust".into()],
        open_graph: Some(OpenGraph {
            title: Some("OG Test Title".into()),
            description: Some("OpenGraph description for testing".into()),
            images: vec![
                OgImage {
                    url: "https://example.com/og-image-1200x630.jpg".into(),
                    width: Some(1200),
                    height: Some(630),
                    alt: Some("Primary OG image".into()),
                    ..Default::default()
                },
                OgImage {
                    url: "https://example.com/og-image-600x314.jpg".into(), 
                    width: Some(600),
                    height: Some(314),
                    alt: Some("Secondary OG image".into()),
                    ..Default::default()
                },
            ],
            locale: Some("en_US".into()),
            site_name: Some("Test Site".into()),
            og_type: Some("article".into()),
            url: Some("https://example.com/test-article".into()),
        }),
        twitter: Some(Twitter {
            card: Some("summary_large_image".into()),
            site: Some("@testsite".into()),
            creator: Some("@testcreator".into()),
            title: Some("Twitter Test Title".into()),
            description: Some("Twitter card description for testing".into()),
            image: Some("https://example.com/twitter-image.jpg".into()),
        }),
        robots: Some(Robots {
            index: Some(true),
            follow: Some(true),
            noarchive: Some(false),
            nosnippet: Some(false),
            noimageindex: Some(false),
            nocache: Some(false),
            max_snippet: Some(160),
            max_image_preview: Some(200),
            max_video_preview: Some(300),
            ..Default::default()
        }),
        icons: Some(Icons {
            icon: vec![
                Icon {
                    url: "/favicon.ico".into(),
                    sizes: Some("16x16".into()),
                    icon_type: Some("image/x-icon".into()),
                    rel: Some("icon".into()),
                    ..Default::default()
                },
                Icon {
                    url: "/favicon-32x32.png".into(),
                    sizes: Some("32x32".into()),
                    icon_type: Some("image/png".into()),
                    rel: Some("icon".into()),
                    ..Default::default()
                },
            ],
            apple: vec![
                Icon {
                    url: "/apple-touch-icon.png".into(),
                    sizes: Some("180x180".into()),
                    icon_type: Some("image/png".into()),
                    rel: Some("apple-touch-icon".into()),
                    ..Default::default()
                },
            ],
            shortcut: vec![],
            other: vec![],
        }),
        alternates: Some(Alternates {
            canonical: Some("https://example.com/test-canonical".into()),
            languages: {
                let mut langs = HashMap::new();
                langs.insert("en".into(), "https://example.com/en/test".into());
                langs.insert("es".into(), "https://example.com/es/test".into());
                langs.insert("fr".into(), "https://example.com/fr/test".into());
                langs
            },
            media: vec![
                AlternateMedia {
                    url: "/mobile".into(),
                    media: "(max-width: 768px)".into(),
                }
            ],
            types: vec![
                AlternateType {
                    url: "/rss.xml".into(),
                    media_type: "application/rss+xml".into(),
                    title: Some("RSS Feed".into()),
                },
                AlternateType {
                    url: "/sitemap.xml".into(),
                    media_type: "application/xml".into(),
                    title: Some("Sitemap".into()),
                },
            ],
        }),
        viewport: Some(Viewport {
            width: Some("device-width".into()),
            initial_scale: Some(1.0),
            maximum_scale: Some(2.0),
            minimum_scale: Some(0.5),
            user_scalable: Some(true),
            viewport_fit: Some("cover".into()),
        }),
        manifest: Some("/manifest.json".into()),
        other: {
            let mut other = HashMap::new();
            other.insert("theme-color".into(), "#667eea".into());
            other.insert("msapplication-TileColor".into(), "#667eea".into());
            other.insert("msapplication-TileImage".into(), "/mstile-144x144.png".into());
            other
        },
    }
}

/// Fluent assertion helper for metadata testing
pub struct MetadataAssert<'a> {
    metadata: &'a Metadata,
}

impl<'a> MetadataAssert<'a> {
    pub fn new(metadata: &'a Metadata) -> Self {
        Self { metadata }
    }
    
    /// Assert title matches expected value (resolving templates if needed)
    pub fn has_title(self, expected: &str) -> Self {
        match &self.metadata.title {
            Some(title) => {
                let resolved = title.resolve(None);
                assert_eq!(resolved, expected, "Title mismatch");
            }
            None => panic!("Expected title '{}', but metadata has no title", expected),
        }
        self
    }
    
    /// Assert title matches with specific segment for template resolution
    pub fn has_title_with_segment(self, expected: &str, segment: &str) -> Self {
        match &self.metadata.title {
            Some(title) => {
                let resolved = title.resolve(Some(segment));
                assert_eq!(resolved, expected, "Title with segment mismatch");
            }
            None => panic!("Expected title '{}', but metadata has no title", expected),
        }
        self
    }
    
    /// Assert description matches expected value
    pub fn has_description(self, expected: &str) -> Self {
        assert_eq!(
            self.metadata.description.as_deref(),
            Some(expected),
            "Description mismatch"
        );
        self
    }
    
    /// Assert keywords contain all expected values
    pub fn has_keywords(self, expected: &[&str]) -> Self {
        let keywords: Vec<&str> = self.metadata.keywords.iter().map(|k| k.as_str()).collect();
        for keyword in expected {
            assert!(
                keywords.contains(keyword),
                "Missing keyword: '{}'. Found: {:?}",
                keyword, keywords
            );
        }
        self
    }
    
    /// Assert OpenGraph title matches expected value
    pub fn has_og_title(self, expected: &str) -> Self {
        match &self.metadata.open_graph {
            Some(og) => {
                assert_eq!(
                    og.title.as_deref(),
                    Some(expected),
                    "OpenGraph title mismatch"
                );
            }
            None => panic!("Expected OG title '{}', but no OpenGraph data", expected),
        }
        self
    }
    
    /// Assert OpenGraph description matches expected value
    pub fn has_og_description(self, expected: &str) -> Self {
        match &self.metadata.open_graph {
            Some(og) => {
                assert_eq!(
                    og.description.as_deref(),
                    Some(expected),
                    "OpenGraph description mismatch"
                );
            }
            None => panic!("Expected OG description '{}', but no OpenGraph data", expected),
        }
        self
    }
    
    /// Assert OpenGraph has an image with the specified URL
    pub fn has_og_image(self, url: &str) -> Self {
        match &self.metadata.open_graph {
            Some(og) => {
                let found = og.images.iter().any(|img| img.url == url);
                assert!(found, "OpenGraph image '{}' not found. Images: {:?}", 
                        url, og.images.iter().map(|img| &img.url).collect::<Vec<_>>());
            }
            None => panic!("Expected OG image '{}', but no OpenGraph data", url),
        }
        self
    }
    
    /// Assert Twitter card type matches expected value
    pub fn has_twitter_card(self, expected: &str) -> Self {
        match &self.metadata.twitter {
            Some(twitter) => {
                assert_eq!(
                    twitter.card.as_deref(),
                    Some(expected),
                    "Twitter card mismatch"
                );
            }
            None => panic!("Expected Twitter card '{}', but no Twitter data", expected),
        }
        self
    }
    
    /// Assert robots directive is set correctly
    pub fn has_robots_directive(self, directive: &str, expected: bool) -> Self {
        match &self.metadata.robots {
            Some(robots) => {
                let actual = match directive {
                    "index" => robots.index.unwrap_or(true),
                    "follow" => robots.follow.unwrap_or(true),
                    "noarchive" => robots.noarchive.unwrap_or(false),
                    "nosnippet" => robots.nosnippet.unwrap_or(false),
                    "noimageindex" => robots.noimageindex.unwrap_or(false),
                    "nocache" => robots.nocache.unwrap_or(false),
                    _ => panic!("Unknown robots directive: {}", directive),
                };
                assert_eq!(actual, expected, "Robots directive '{}' mismatch", directive);
            }
            None => panic!("Expected robots directive '{}', but no robots data", directive),
        }
        self
    }
    
    /// Assert canonical URL matches expected value
    pub fn has_canonical(self, expected: &str) -> Self {
        match &self.metadata.alternates {
            Some(alternates) => {
                assert_eq!(
                    alternates.canonical.as_deref(),
                    Some(expected),
                    "Canonical URL mismatch"
                );
            }
            None => panic!("Expected canonical '{}', but no alternates data", expected),
        }
        self
    }
    
    /// Assert alternate language exists
    pub fn has_alternate_language(self, lang: &str, url: &str) -> Self {
        match &self.metadata.alternates {
            Some(alternates) => {
                assert_eq!(
                    alternates.languages.get(lang).map(|s| s.as_str()),
                    Some(url),
                    "Alternate language '{}' mismatch", lang
                );
            }
            None => panic!("Expected alternate language '{}', but no alternates data", lang),
        }
        self
    }
    
    /// Assert metadata is valid (no validation errors)
    pub fn is_valid(self) -> Self {
        let validation = self.metadata.validate();
        assert!(validation.is_valid, "Metadata validation failed: {:?}", validation.errors);
        self
    }
    
    /// Assert metadata has validation errors
    pub fn has_validation_errors(self) -> Self {
        let validation = self.metadata.validate();
        assert!(!validation.is_valid, "Expected validation errors, but metadata is valid");
        self
    }
    
    /// Assert validation contains specific error message
    pub fn has_validation_error_containing(self, text: &str) -> Self {
        let validation = self.metadata.validate();
        let found = validation.errors.iter().any(|error| error.contains(text));
        assert!(found, "Expected validation error containing '{}', but found: {:?}", 
                text, validation.errors);
        self
    }
}

/// Create fluent assertion helper
pub fn assert_metadata(metadata: &Metadata) -> MetadataAssert {
    MetadataAssert::new(metadata)
}

/// Test utility for creating OG image parameters
pub fn create_test_og_params() -> OgImageParams {
    OgImageParams {
        template: "test_template".into(),
        data: liquid::object!({
            "title": "Test OG Image",
            "description": "Testing OG image generation",
            "background": "#667eea",
        }),
        size: (1200, 630),
    }
}

/// Test utility for creating JSON-LD Article
pub fn create_test_article() -> Article {
    Article::builder()
        .headline("Test Article Headline")
        .description("Test article description for structured data testing")
        .author(
            Person::builder()
                .name("Test Author")
                .url("https://testauthor.example.com")
                .build()
        )
        .date_published("2024-01-15T10:30:00Z")
        .url("https://example.com/test-article")
        .image("https://example.com/test-article-image.jpg")
        .publisher(
            Organization::builder()
                .name("Test Publisher")
                .url("https://testpublisher.example.com")
                .logo("https://testpublisher.example.com/logo.png")
                .build()
        )
        .build()
}

/// Test utility for comparing metadata structures while ignoring order-sensitive fields
pub fn metadata_equals_ignore_order(a: &Metadata, b: &Metadata) -> bool {
    // Compare titles
    if a.title != b.title {
        return false;
    }
    
    // Compare descriptions
    if a.description != b.description {
        return false;
    }
    
    // Compare keywords (order-insensitive)
    let mut a_keywords = a.keywords.clone();
    let mut b_keywords = b.keywords.clone();
    a_keywords.sort();
    b_keywords.sort();
    if a_keywords != b_keywords {
        return false;
    }
    
    // Compare OpenGraph (complex structure, but order matters for images)
    if a.open_graph != b.open_graph {
        return false;
    }
    
    // Compare other fields
    a.twitter == b.twitter &&
    a.robots == b.robots &&
    a.icons == b.icons &&
    a.alternates == b.alternates &&
    a.viewport == b.viewport &&
    a.manifest == b.manifest &&
    a.other == b.other
}

/// Mock HTTP client for testing external dependencies
#[cfg(feature = "http")]
pub struct MockHttpClient {
    responses: std::collections::HashMap<String, (u16, String)>,
}

#[cfg(feature = "http")]
impl MockHttpClient {
    pub fn new() -> Self {
        Self {
            responses: std::collections::HashMap::new(),
        }
    }
    
    pub fn mock_response(&mut self, url: &str, status: u16, body: &str) {
        self.responses.insert(url.to_string(), (status, body.to_string()));
    }
    
    pub async fn get(&self, url: &str) -> Result<(u16, String), Box<dyn std::error::Error>> {
        match self.responses.get(url) {
            Some((status, body)) => Ok((*status, body.clone())),
            None => Err(format!("No mock response for URL: {}", url).into()),
        }
    }
}

/// Snapshot testing helper
pub fn assert_metadata_snapshot(metadata: &Metadata, snapshot_name: &str) {
    use insta::assert_yaml_snapshot;
    assert_yaml_snapshot!(snapshot_name, metadata);
}

/// Performance testing helper
pub fn measure_operation<F, R>(operation: F) -> (R, std::time::Duration)
where 
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

/// Async performance testing helper  
pub async fn measure_async_operation<F, Fut, R>(operation: F) -> (R, std::time::Duration)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = R>,
{
    let start = std::time::Instant::now();
    let result = operation().await;
    let duration = start.elapsed();
    (result, duration)
}

/// Test data generators for property-based testing
#[cfg(feature = "proptest")]
pub mod generators {
    use super::*;
    use proptest::prelude::*;
    
    pub fn arb_title() -> impl Strategy<Value = Title> {
        prop_oneof![
            any::<String>().prop_map(Title::Static),
            (any::<String>(), any::<String>()).prop_map(|(template, default)| {
                Title::Template { template, default }
            }),
            any::<String>().prop_map(Title::Absolute),
        ]
    }
    
    pub fn arb_metadata() -> impl Strategy<Value = Metadata> {
        (
            prop::option::of(arb_title()),
            prop::option::of(any::<String>()),
            prop::collection::vec(any::<String>(), 0..10),
        ).prop_map(|(title, description, keywords)| {
            Metadata {
                title,
                description,
                keywords,
                ..Default::default()
            }
        })
    }
    
    pub fn arb_og_image_params() -> impl Strategy<Value = OgImageParams> {
        (
            any::<String>(),
            any::<String>(),
            any::<String>(),
            1u32..2000,
            1u32..2000,
        ).prop_map(|(template, title, desc, width, height)| {
            OgImageParams {
                template,
                data: liquid::object!({
                    "title": title,
                    "description": desc,
                }),
                size: (width, height),
            }
        })
    }
}
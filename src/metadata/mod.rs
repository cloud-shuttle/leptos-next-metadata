//! Core metadata types and functionality for leptos-next-metadata
//! 
//! This module provides the foundational types for managing page metadata,
//! including titles, descriptions, Open Graph tags, Twitter cards, and more.

pub mod context;
pub mod merge;
pub mod validation;

pub use context::*;
pub use merge::*;
pub use validation::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main metadata struct representing page metadata
/// 
/// This struct follows the Next.js metadata API structure while providing
/// type safety and additional features specific to Leptos applications.
/// 
/// # Examples
/// 
/// ```rust
/// use leptos_next_metadata::metadata::{Metadata, Title};
/// 
/// let meta = Metadata {
///     title: Some(Title::Static("My Page".into())),
///     description: Some("Description".into()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// The page title
    pub title: Option<Title>,
    
    /// The page description
    pub description: Option<String>,
    
    /// Keywords for SEO
    pub keywords: Option<Keywords>,
    
    /// Authors of the page
    pub authors: Option<Authors>,
    
    /// Robots directives
    pub robots: Option<Robots>,
    
    /// Open Graph metadata
    pub open_graph: Option<OpenGraph>,
    
    /// Twitter Card metadata
    pub twitter: Option<Twitter>,
    
    /// JSON-LD structured data
    #[cfg(feature = "json-ld")]
    pub json_ld: Option<JsonLd>,
    
    /// JSON-LD structured data (fallback when json-ld feature is disabled)
    #[cfg(not(feature = "json-ld"))]
    pub json_ld: Option<String>,
    
    /// Canonical URL
    pub canonical: Option<CanonicalUrl>,
    
    /// Alternate language versions
    pub alternates: Option<HashMap<String, AlternateLink>>,
    
    /// Viewport settings
    pub viewport: Option<Viewport>,
    
    /// Theme color
    pub theme_color: Option<ThemeColor>,
    
    /// Color scheme
    pub color_scheme: Option<ColorScheme>,
    
    /// Referrer policy
    pub referrer: Option<ReferrerPolicy>,
    
    /// Format detection
    pub format_detection: Option<FormatDetection>,
    
    /// Additional metadata fields
    #[serde(flatten)]
    pub additional: HashMap<String, AdditionalValue>,
}

/// Page title with support for templates and dynamic values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Title {
    /// Static title string
    Static(String),
    
    /// Template-based title with default fallback
    Template {
        /// Template string (e.g., "%s | My Site")
        template: String,
        /// Default value if template variables are missing
        default: String,
    },
    

}

/// Page description
pub type Description = String;

/// Keywords for SEO optimization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keywords {
    /// Single keyword
    Single(String),
    /// Multiple keywords
    Multiple(Vec<String>),
}

/// Authors of the page
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Authors {
    /// Single author
    Single(Author),
    /// Multiple authors
    Multiple(Vec<Author>),
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    /// Author name
    pub name: String,
    
    /// Author URL
    pub url: Option<String>,
    
    /// Author email
    pub email: Option<String>,
    
    /// Author image
    pub image: Option<String>,
}

/// Robots directives for search engine crawling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robots {
    /// Allow indexing
    pub index: bool,
    
    /// Allow following links
    pub follow: bool,
    
    /// Allow archiving
    pub archive: bool,
    
    /// Allow image indexing
    pub image_index: bool,
    
    /// Allow snippet previews
    pub snippet: bool,
    
    /// Crawl delay in seconds
    pub crawl_delay: Option<u64>,
    
    /// Additional directives
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

impl Robots {
    /// Create robots directive that allows all
    pub fn all() -> Self {
        Self {
            index: true,
            follow: true,
            archive: true,
            image_index: true,
            snippet: true,
            crawl_delay: None,
            additional: HashMap::new(),
        }
    }
    
    /// Create robots directive that blocks all
    pub fn none() -> Self {
        Self {
            index: false,
            follow: false,
            archive: false,
            image_index: false,
            snippet: false,
            crawl_delay: None,
            additional: HashMap::new(),
        }
    }
    
    /// Create robots directive that blocks indexing but allows following
    pub fn noindex() -> Self {
        Self {
            index: false,
            follow: true,
            archive: false,
            image_index: false,
            snippet: false,
            crawl_delay: None,
            additional: HashMap::new(),
        }
    }
}

/// Open Graph metadata for social media sharing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenGraph {
    /// Title for social sharing
    pub title: Option<String>,
    
    /// Description for social sharing
    pub description: Option<String>,
    
    /// URL of the page
    pub url: Option<String>,
    
    /// Type of content
    pub r#type: Option<String>,
    
    /// Site name
    pub site_name: Option<String>,
    
    /// Locale
    pub locale: Option<String>,
    
    /// Alternate locales
    pub locale_alternate: Option<Vec<String>>,
    
    /// Images for social sharing
    pub images: Vec<OgImage>,
    
    /// Videos for social sharing
    pub videos: Option<Vec<OgVideo>>,
    
    /// Audio for social sharing
    pub audio: Option<Vec<OgAudio>>,
    
    /// Article-specific metadata
    pub article: Option<Article>,
    
    /// Profile-specific metadata
    pub profile: Option<Profile>,
    
    /// Book-specific metadata
    pub book: Option<Book>,
    
    /// Additional Open Graph properties
    #[serde(flatten)]
    pub additional: HashMap<String, AdditionalValue>,
}

/// Open Graph image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgImage {
    /// Image URL
    pub url: String,
    
    /// Image width
    pub width: Option<u32>,
    
    /// Image height
    pub height: Option<u32>,
    
    /// Image alt text
    pub alt: Option<String>,
    
    /// Image type
    pub r#type: Option<String>,
}

impl OgImage {
    /// Create a new OG image with URL
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            width: None,
            height: None,
            alt: None,
            r#type: None,
        }
    }
    
    /// Create a new OG image with dimensions
    pub fn with_dimensions(url: &str, width: u32, height: u32) -> Self {
        Self {
            url: url.to_string(),
            width: Some(width),
            height: Some(height),
            alt: None,
            r#type: None,
        }
    }
}

/// Open Graph video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgVideo {
    /// Video URL
    pub url: String,
    
    /// Video width
    pub width: Option<u32>,
    
    /// Video height
    pub height: Option<u32>,
    
    /// Video type
    pub r#type: Option<String>,
}

/// Open Graph audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgAudio {
    /// Audio URL
    pub url: String,
    
    /// Audio type
    pub r#type: Option<String>,
}

/// Article-specific Open Graph metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Publication time
    pub published_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Modification time
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Expiration time
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Author URL
    pub author: Option<String>,
    
    /// Section
    pub section: Option<String>,
    
    /// Tags
    pub tags: Option<Vec<String>>,
}

/// Profile-specific Open Graph metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// First name
    pub first_name: Option<String>,
    
    /// Last name
    pub last_name: Option<String>,
    
    /// Username
    pub username: Option<String>,
    
    /// Gender
    pub gender: Option<String>,
}

/// Book-specific Open Graph metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    /// Author
    pub author: Option<String>,
    
    /// ISBN
    pub isbn: Option<String>,
    
    /// Release date
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Tags
    pub tags: Option<Vec<String>>,
}

/// Twitter Card metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Twitter {
    /// Card type
    pub card: Option<TwitterCard>,
    
    /// Site username
    pub site: Option<String>,
    
    /// Creator username
    pub creator: Option<String>,
    
    /// Title
    pub title: Option<String>,
    
    /// Description
    pub description: Option<String>,
    
    /// Image
    pub image: Option<String>,
    
    /// Image alt text
    pub image_alt: Option<String>,
}

/// Twitter Card types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwitterCard {
    /// Summary card
    Summary,
    
    /// Summary large image card
    SummaryLargeImage,
    
    /// App card
    App,
    
    /// Player card
    Player,
}

/// JSON-LD structured data
#[cfg(feature = "json-ld")]
pub type JsonLd = serde_json::Value;

/// JSON-LD structured data (fallback when json-ld feature is disabled)
#[cfg(not(feature = "json-ld"))]
pub type JsonLd = String;

/// Additional metadata value that can handle both json-ld and fallback cases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdditionalValue {
    String(String),
    #[cfg(feature = "json-ld")]
    Json(serde_json::Value),
}

impl From<String> for AdditionalValue {
    fn from(s: String) -> Self {
        AdditionalValue::String(s)
    }
}

impl From<&str> for AdditionalValue {
    fn from(s: &str) -> Self {
        AdditionalValue::String(s.to_string())
    }
}

#[cfg(feature = "json-ld")]
impl From<serde_json::Value> for AdditionalValue {
    fn from(v: serde_json::Value) -> Self {
        AdditionalValue::Json(v)
    }
}



/// Canonical URL
pub type CanonicalUrl = String;

/// Alternate language link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateLink {
    /// URL of the alternate version
    pub href: String,
    
    /// Language code
    pub hreflang: String,
    
    /// Media type
    pub media: Option<String>,
}

/// Viewport settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    /// Width
    pub width: Option<String>,
    
    /// Height
    pub height: Option<String>,
    
    /// Initial scale
    pub initial_scale: Option<f32>,
    
    /// Minimum scale
    pub minimum_scale: Option<f32>,
    
    /// Maximum scale
    pub maximum_scale: Option<f32>,
    
    /// User scalable
    pub user_scalable: Option<bool>,
    
    /// Viewport fit
    pub viewport_fit: Option<String>,
}

/// Theme color
pub type ThemeColor = String;

/// Color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Light theme
    Light,
    
    /// Dark theme
    Dark,
    
    /// Auto (follows system preference)
    Auto,
}

/// Referrer policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferrerPolicy {
    /// No referrer
    NoReferrer,
    
    /// No referrer when downgrade
    NoReferrerWhenDowngrade,
    
    /// Origin
    Origin,
    
    /// Origin when cross-origin
    OriginWhenCrossOrigin,
    
    /// Same origin
    SameOrigin,
    
    /// Strict origin
    StrictOrigin,
    
    /// Strict origin when cross-origin
    StrictOriginWhenCrossOrigin,
    
    /// Unsafe URL
    UnsafeUrl,
}

/// Format detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatDetection {
    /// Detect telephone numbers
    pub telephone: Option<bool>,
    
    /// Detect email addresses
    pub email: Option<bool>,
    
    /// Detect addresses
    pub address: Option<bool>,
}

impl Metadata {
    /// Create a new empty metadata instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new metadata instance with title
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: Some(Title::Static(title.into())),
            ..Default::default()
        }
    }
    
    /// Create a new metadata instance with title and description
    pub fn with_title_and_description(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            title: Some(Title::Static(title.into())),
            description: Some(description.into()),
            ..Default::default()
        }
    }
    
    /// Set the title
    pub fn title(mut self, title: impl Into<Title>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Set the keywords
    pub fn keywords(mut self, keywords: impl Into<Keywords>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }
    
    /// Set the authors
    pub fn authors(mut self, authors: impl Into<Authors>) -> Self {
        self.authors = Some(authors.into());
        self
    }
    
    /// Set the robots directive
    pub fn robots(mut self, robots: Robots) -> Self {
        self.robots = Some(robots);
        self
    }
    
    /// Set the Open Graph metadata
    pub fn open_graph(mut self, open_graph: OpenGraph) -> Self {
        self.open_graph = Some(open_graph);
        self
    }
    
    /// Set the Twitter metadata
    pub fn twitter(mut self, twitter: Twitter) -> Self {
        self.twitter = Some(twitter);
        self
    }
    
    /// Set the JSON-LD data
    pub fn json_ld(mut self, json_ld: JsonLd) -> Self {
        self.json_ld = Some(json_ld);
        self
    }
    
    /// Set the canonical URL
    pub fn canonical(mut self, canonical: impl Into<String>) -> Self {
        self.canonical = Some(canonical.into());
        self
    }
    
    /// Add an alternate language link
    pub fn alternate(mut self, hreflang: impl Into<String>, href: impl Into<String>) -> Self {
        let alternate = AlternateLink {
            href: href.into(),
            hreflang: hreflang.into(),
            media: None,
        };
        
        if let Some(ref mut alternates) = self.alternates {
            alternates.insert(alternate.hreflang.clone(), alternate);
        } else {
            let mut alternates = HashMap::new();
            alternates.insert(alternate.hreflang.clone(), alternate);
            self.alternates = Some(alternates);
        }
        
        self
    }
    
    /// Set the viewport
    pub fn viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = Some(viewport);
        self
    }
    
    /// Set the theme color
    pub fn theme_color(mut self, theme_color: impl Into<String>) -> Self {
        self.theme_color = Some(theme_color.into());
        self
    }
    
    /// Set the color scheme
    pub fn color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.color_scheme = Some(color_scheme);
        self
    }
    
    /// Set the referrer policy
    pub fn referrer(mut self, referrer: ReferrerPolicy) -> Self {
        self.referrer = Some(referrer);
        self
    }
    
    /// Set the format detection
    pub fn format_detection(mut self, format_detection: FormatDetection) -> Self {
        self.format_detection = Some(format_detection);
        self
    }
    
    /// Add additional metadata field
    pub fn additional(mut self, key: impl Into<String>, value: impl Into<AdditionalValue>) -> Self {
        self.additional.insert(key.into(), value.into());
        self
    }
}

impl From<String> for Title {
    fn from(s: String) -> Self {
        Title::Static(s)
    }
}

impl From<&str> for Title {
    fn from(s: &str) -> Self {
        Title::Static(s.to_string())
    }
}

impl From<String> for Keywords {
    fn from(s: String) -> Self {
        Keywords::Single(s)
    }
}

impl From<&str> for Keywords {
    fn from(s: &str) -> Self {
        Keywords::Single(s.to_string())
    }
}

impl From<Vec<String>> for Keywords {
    fn from(v: Vec<String>) -> Self {
        Keywords::Multiple(v)
    }
}

impl From<&[&str]> for Keywords {
    fn from(v: &[&str]) -> Self {
        Keywords::Multiple(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<Author> for Authors {
    fn from(author: Author) -> Self {
        Authors::Single(author)
    }
}

impl From<Vec<Author>> for Authors {
    fn from(authors: Vec<Author>) -> Self {
        Authors::Multiple(authors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

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
    fn test_metadata_with_title() {
        let metadata = Metadata::with_title("Test Title");
        assert_eq!(metadata.title, Some(Title::Static("Test Title".to_string())));
        assert!(metadata.description.is_none());
    }

    #[test]
    fn test_metadata_with_title_and_description() {
        let metadata = Metadata::with_title_and_description("Test Title", "Test Description");
        assert_eq!(metadata.title, Some(Title::Static("Test Title".to_string())));
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

        assert_eq!(metadata.title, Some(Title::Static("Test Title".to_string())));
        assert_eq!(metadata.description, Some("Test Description".to_string()));
        assert_eq!(metadata.keywords, Some(Keywords::Multiple(vec!["test".to_string(), "metadata".to_string()])));
        assert_eq!(metadata.canonical, Some("https://example.com".to_string()));
        assert_eq!(metadata.theme_color, Some("#ffffff".to_string()));
        assert_eq!(metadata.color_scheme, Some(ColorScheme::Light));
        assert_eq!(metadata.referrer, Some(ReferrerPolicy::NoReferrer));
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
    fn test_title_from_impls() {
        let title1: Title = "String Title".into();
        let title2: Title = "String Title".to_string().into();

        assert!(matches!(title1, Title::Static(_)));
        assert!(matches!(title2, Title::Static(_)));
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
    fn test_keywords_from_impls() {
        let keywords1: Keywords = "single".into();
        let keywords2: Keywords = vec!["one".to_string(), "two".to_string()].into();
        let keywords3: Keywords = ["one", "two"][..].into();

        assert!(matches!(keywords1, Keywords::Single(_)));
        assert!(matches!(keywords2, Keywords::Multiple(_)));
        assert!(matches!(keywords3, Keywords::Multiple(_)));
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
        assert_eq!(author.image, Some("https://example.com/avatar.jpg".to_string()));
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

        let og_image_with_dimensions = OgImage::with_dimensions("https://example.com/image.jpg", 1200, 630);
        assert_eq!(og_image_with_dimensions.url, "https://example.com/image.jpg");
        assert_eq!(og_image_with_dimensions.width, Some(1200));
        assert_eq!(og_image_with_dimensions.height, Some(630));
    }

    #[test]
    fn test_open_graph_struct() {
        let mut og = OpenGraph::default();
        og.title = Some("Test Title".to_string());
        og.description = Some("Test Description".to_string());
        og.url = Some("https://example.com".to_string());
        og.r#type = Some("website".to_string());
        og.site_name = Some("Test Site".to_string());
        og.locale = Some("en_US".to_string());
        og.images = vec![OgImage::new("https://example.com/image.jpg")];

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
        let mut twitter = Twitter::default();
        twitter.card = Some(TwitterCard::SummaryLargeImage);
        twitter.site = Some("@testsite".to_string());
        twitter.creator = Some("@testcreator".to_string());
        twitter.title = Some("Test Title".to_string());
        twitter.description = Some("Test Description".to_string());
        twitter.image = Some("https://example.com/image.jpg".to_string());
        twitter.image_alt = Some("Test Image Alt".to_string());

        assert_eq!(twitter.card, Some(TwitterCard::SummaryLargeImage));
        assert_eq!(twitter.site, Some("@testsite".to_string()));
        assert_eq!(twitter.creator, Some("@testcreator".to_string()));
        assert_eq!(twitter.title, Some("Test Title".to_string()));
        assert_eq!(twitter.description, Some("Test Description".to_string()));
        assert_eq!(twitter.image, Some("https://example.com/image.jpg".to_string()));
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
        assert_eq!(article.author, Some("https://example.com/author".to_string()));
        assert_eq!(article.section, Some("Technology".to_string()));
        assert_eq!(article.tags, Some(vec!["rust".to_string(), "leptos".to_string()]));
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
        assert_eq!(book.tags, Some(vec!["fiction".to_string(), "adventure".to_string()]));
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
        assert_eq!(alternate.media, Some("only screen and (max-width: 640px)".to_string()));
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
        assert_eq!(metadata.additional["custom_field"], AdditionalValue::Json(serde_json::json!("custom_value")));
        assert_eq!(metadata.additional["number_field"], AdditionalValue::Json(serde_json::json!(42)));
        assert_eq!(metadata.additional["bool_field"], AdditionalValue::Json(serde_json::json!(true)));
    }
    
    #[test]
    #[cfg(not(feature = "json-ld"))]
    fn test_metadata_additional_fields_fallback() {
        let metadata = Metadata::new()
            .additional("custom_field", "custom_value".to_string())
            .additional("number_field", "42".to_string())
            .additional("bool_field", "true".to_string());

        assert_eq!(metadata.additional.len(), 3);
        assert_eq!(metadata.additional["custom_field"], AdditionalValue::String("custom_value".to_string()));
        assert_eq!(metadata.additional["number_field"], AdditionalValue::String("42".to_string()));
        assert_eq!(metadata.additional["bool_field"], AdditionalValue::String("true".to_string()));
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
        assert_eq!(metadata.title, Some(Title::Static("Test Title".to_string())));
        assert_eq!(metadata.description, Some("Test Description".to_string()));
        assert_eq!(metadata.keywords, Some(Keywords::Multiple(vec!["test".to_string(), "metadata".to_string()])));
        assert_eq!(metadata.canonical, Some("https://example.com".to_string()));
    }
}

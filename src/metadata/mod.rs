//! Core metadata types and functionality for leptos-next-metadata
//! 
//! This module provides the foundational types for managing page metadata,
//! including titles, descriptions, Open Graph tags, Twitter cards, and more.

mod context;
mod merge;
mod validation;

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
/// use leptos_next_metadata::metadata::Metadata;
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
    pub json_ld: Option<JsonLd>,
    
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
    pub additional: HashMap<String, serde_json::Value>,
}

/// Page title with support for templates and dynamic values
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    
    /// Dynamic title that can be computed at runtime
    Dynamic {
        /// Function to generate title
        #[serde(skip)]
        generator: fn() -> String,
    },
}

/// Page description
pub type Description = String;

/// Keywords for SEO optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub additional: HashMap<String, serde_json::Value>,
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
pub type JsonLd = serde_json::Value;

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
    pub fn additional(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.additional.insert(key.into(), value);
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

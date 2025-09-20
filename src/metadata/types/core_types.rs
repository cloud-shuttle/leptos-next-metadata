//! Core metadata types and main structures
//!
//! This module contains the primary metadata container and basic types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import types from other modules
use super::browser_types::{
    AlternateLink, ColorScheme, FormatDetection, JsonLd, ReferrerPolicy, Viewport,
};
use super::open_graph_types::OpenGraph;
use super::twitter_types::Twitter;

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

    /// Canonical URL
    pub canonical: Option<CanonicalUrl>,

    /// Alternate links for different languages/regions
    pub alternate_links: Vec<AlternateLink>,

    /// Viewport configuration
    pub viewport: Option<Viewport>,

    /// Theme color for mobile browsers
    pub theme_color: Option<ThemeColor>,

    /// Color scheme preference
    pub color_scheme: Option<ColorScheme>,

    /// Referrer policy
    pub referrer: Option<ReferrerPolicy>,

    /// Format detection settings
    pub format_detection: Option<FormatDetection>,

    /// Additional metadata fields
    pub additional: HashMap<String, AdditionalValue>,
}

/// Title with template support
///
/// Supports both static titles and dynamic templates with default values.
/// Templates use Liquid syntax for variable substitution.
///
/// # Examples
///
/// ```rust
/// use leptos_next_metadata::metadata::Title;
///
/// // Static title
/// let static_title = Title::Static("My Page".to_string());
///
/// // Template title with default
/// let template_title = Title::Template {
///     template: "{{ page.title }} - {{ site.name }}".to_string(),
///     default: "My Site".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Title {
    /// Static title string
    Static(String),
    /// Template with default fallback
    Template { template: String, default: String },
}

/// Page description type alias
pub type Description = String;

/// Keywords for SEO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Keywords {
    /// Single keyword string
    Single(String),
    /// Multiple keywords
    Multiple(Vec<String>),
}

/// Authors of the page
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Authors {
    /// Single author
    Single(Author),
    /// Multiple authors
    Multiple(Vec<Author>),
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    /// Author name
    pub name: String,
    /// Author URL
    pub url: Option<String>,
    /// Author email
    pub email: Option<String>,
}

/// Robots directives for search engines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Robots {
    /// Allow indexing
    pub index: Option<bool>,
    /// Allow following links
    pub follow: Option<bool>,
    /// Allow Google to show cached version
    pub google_bot: Option<GoogleBot>,
    /// Additional directives
    pub other: HashMap<String, String>,
}

/// Google Bot specific directives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoogleBot {
    /// Allow indexing
    pub index: Option<bool>,
    /// Allow following links
    pub follow: Option<bool>,
    /// Maximum snippet length
    pub max_snippet: Option<i32>,
    /// Maximum image preview size
    pub max_image_preview: Option<MaxImagePreview>,
    /// Maximum video preview length
    pub max_video_preview: Option<i32>,
}

/// Maximum image preview size for Google
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaxImagePreview {
    /// No image preview
    None,
    /// Standard preview
    Standard,
    /// Large preview
    Large,
}

/// Canonical URL type alias
pub type CanonicalUrl = String;

/// Theme color type alias
pub type ThemeColor = String;

/// Additional metadata value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdditionalValue {
    /// String value
    String(String),
    /// Array of strings
    Array(Vec<String>),
    /// Object value
    Object(HashMap<String, String>),
}

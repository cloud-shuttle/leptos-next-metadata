//! Builder pattern implementations for metadata types
//!
//! This module provides builder methods and implementations for constructing
//! metadata objects with a fluent API.

use super::types::*;
use std::collections::HashMap;

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
            url: href.into(),
            hreflang: Some(hreflang.into()),
            media: None,
            type_: None,
        };

        self.alternate_links.push(alternate);

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

impl Robots {
    /// Create robots directive that allows all
    pub fn all() -> Self {
        Self {
            index: Some(true),
            follow: Some(true),
            google_bot: None,
            other: HashMap::new(),
        }
    }

    /// Create robots directive that blocks all
    pub fn none() -> Self {
        Self {
            index: Some(false),
            follow: Some(false),
            google_bot: None,
            other: HashMap::new(),
        }
    }

    /// Create robots directive that blocks indexing but allows following
    pub fn noindex() -> Self {
        Self {
            index: Some(false),
            follow: Some(true),
            google_bot: None,
            other: HashMap::new(),
        }
    }
}

impl OgImage {
    /// Create a new OG image with URL
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            width: None,
            height: None,
            alt: None,
            type_: None,
        }
    }

    /// Create a new OG image with dimensions
    pub fn with_dimensions(url: &str, width: u32, height: u32) -> Self {
        Self {
            url: url.to_string(),
            width: Some(width),
            height: Some(height),
            alt: None,
            type_: None,
        }
    }
}

//! Display and Debug implementations for metadata types
//!
//! This module provides custom Display and Debug implementations for
//! better formatting and debugging of metadata structures.

use super::types::*;
use std::fmt;

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Title::Static(s) => write!(f, "{}", s),
            Title::Template { template, default } => {
                write!(f, "Template: {} (default: {})", template, default)
            }
        }
    }
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keywords::Single(s) => write!(f, "{}", s),
            Keywords::Multiple(v) => write!(f, "{}", v.join(", ")),
        }
    }
}

impl fmt::Display for Authors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Authors::Single(author) => write!(f, "{}", author.name),
            Authors::Multiple(authors) => {
                let names: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();
                write!(f, "{}", names.join(", "))
            }
        }
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ref url) = self.url {
            write!(f, " ({})", url)?;
        }
        Ok(())
    }
}

impl fmt::Display for Robots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut directives = Vec::new();

        if self.index.unwrap_or(false) {
            directives.push("index".to_string());
        } else {
            directives.push("noindex".to_string());
        }

        if self.follow.unwrap_or(false) {
            directives.push("follow".to_string());
        } else {
            directives.push("nofollow".to_string());
        }

        // Note: Additional robots directives removed as they don't exist in current struct

        write!(f, "{}", directives.join(", "))
    }
}

impl fmt::Display for TwitterCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TwitterCard::Summary => write!(f, "summary"),
            TwitterCard::SummaryLargeImage => write!(f, "summary_large_image"),
            TwitterCard::App => write!(f, "app"),
            TwitterCard::Player => write!(f, "player"),
        }
    }
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorScheme::Light => write!(f, "light"),
            ColorScheme::Dark => write!(f, "dark"),
            ColorScheme::Normal => write!(f, "normal"),
        }
    }
}

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferrerPolicy::NoReferrer => write!(f, "no-referrer"),
            ReferrerPolicy::NoReferrerWhenDowngrade => write!(f, "no-referrer-when-downgrade"),
            ReferrerPolicy::Origin => write!(f, "origin"),
            ReferrerPolicy::OriginWhenCrossOrigin => write!(f, "origin-when-cross-origin"),
            ReferrerPolicy::SameOrigin => write!(f, "same-origin"),
            ReferrerPolicy::StrictOrigin => write!(f, "strict-origin"),
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                write!(f, "strict-origin-when-cross-origin")
            }
            ReferrerPolicy::UnsafeUrl => write!(f, "unsafe-url"),
        }
    }
}

impl fmt::Display for OgImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)?;
        if let (Some(width), Some(height)) = (self.width, self.height) {
            write!(f, " ({}x{})", width, height)?;
        }
        if let Some(ref alt) = self.alt {
            write!(f, " - {}", alt)?;
        }
        Ok(())
    }
}

impl fmt::Display for OgVideo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)?;
        if let (Some(width), Some(height)) = (self.width, self.height) {
            write!(f, " ({}x{})", width, height)?;
        }
        Ok(())
    }
}

impl fmt::Display for OgAudio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref published) = self.published_time {
            write!(f, "Published: {}", published)?;
        }
        if let Some(ref author) = self.author {
            write!(f, ", Author: {}", author)?;
        }
        if let Some(ref section) = self.section {
            write!(f, ", Section: {}", section)?;
        }
        Ok(())
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref first) = self.first_name {
            parts.push(first.clone());
        }
        if let Some(ref last) = self.last_name {
            parts.push(last.clone());
        }
        if parts.is_empty() {
            if let Some(ref username) = self.username {
                write!(f, "@{}", username)?;
            }
        } else {
            write!(f, "{}", parts.join(" "))?;
        }
        Ok(())
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref author) = self.author {
            write!(f, "Author: {}", author)?;
        }
        if let Some(ref isbn) = self.isbn {
            write!(f, ", ISBN: {}", isbn)?;
        }
        if let Some(ref release) = self.release_date {
            write!(f, ", Released: {}", release)?;
        }
        Ok(())
    }
}

impl fmt::Display for ViewportWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewportWidth::DeviceWidth => write!(f, "device-width"),
            ViewportWidth::Pixels(width) => write!(f, "{}", width),
        }
    }
}

impl fmt::Display for ViewportHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewportHeight::DeviceHeight => write!(f, "device-height"),
            ViewportHeight::Pixels(height) => write!(f, "{}", height),
        }
    }
}

impl fmt::Display for ViewportFit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewportFit::Auto => write!(f, "auto"),
            ViewportFit::Contain => write!(f, "contain"),
            ViewportFit::Cover => write!(f, "cover"),
        }
    }
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(ref width) = self.width {
            parts.push(format!("width={}", width));
        }
        if let Some(ref height) = self.height {
            parts.push(format!("height={}", height));
        }
        if let Some(scale) = self.initial_scale {
            parts.push(format!("initial-scale={}", scale));
        }
        if let Some(scale) = self.minimum_scale {
            parts.push(format!("minimum-scale={}", scale));
        }
        if let Some(scale) = self.maximum_scale {
            parts.push(format!("maximum-scale={}", scale));
        }
        if let Some(scalable) = self.user_scalable {
            parts.push(format!(
                "user-scalable={}",
                if scalable { "yes" } else { "no" }
            ));
        }
        if let Some(ref fit) = self.viewport_fit {
            parts.push(format!("viewport-fit={}", fit));
        }

        write!(f, "{}", parts.join(", "))
    }
}

impl fmt::Display for FormatDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(telephone) = self.telephone {
            parts.push(format!(
                "telephone={}",
                if telephone { "yes" } else { "no" }
            ));
        }
        if let Some(email) = self.email {
            parts.push(format!("email={}", if email { "yes" } else { "no" }));
        }
        if let Some(address) = self.address {
            parts.push(format!("address={}", if address { "yes" } else { "no" }));
        }

        write!(f, "{}", parts.join(", "))
    }
}

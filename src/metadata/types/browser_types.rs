//! Browser and viewport related metadata types
//!
//! This module contains types for browser-specific metadata like viewport,
//! theme colors, and format detection.

use serde::{Deserialize, Serialize};

/// Viewport configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Viewport {
    /// Viewport width
    pub width: Option<ViewportWidth>,
    /// Viewport height
    pub height: Option<ViewportHeight>,
    /// Initial scale
    pub initial_scale: Option<f32>,
    /// Minimum scale
    pub minimum_scale: Option<f32>,
    /// Maximum scale
    pub maximum_scale: Option<f32>,
    /// User scalable
    pub user_scalable: Option<bool>,
    /// Viewport fit
    pub viewport_fit: Option<ViewportFit>,
}

/// Viewport width
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewportWidth {
    /// Device width
    DeviceWidth,
    /// Specific width in pixels
    Pixels(u32),
}

/// Viewport height
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewportHeight {
    /// Device height
    DeviceHeight,
    /// Specific height in pixels
    Pixels(u32),
}

/// Viewport fit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewportFit {
    /// Auto fit
    Auto,
    /// Contain
    Contain,
    /// Cover
    Cover,
}

/// Color scheme preference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorScheme {
    /// Light mode only
    Light,
    /// Dark mode only
    Dark,
    /// Both light and dark
    Normal,
}

/// Referrer policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferrerPolicy {
    /// No referrer
    NoReferrer,
    /// No referrer when downgrade
    NoReferrerWhenDowngrade,
    /// Origin
    Origin,
    /// Origin when cross origin
    OriginWhenCrossOrigin,
    /// Same origin
    SameOrigin,
    /// Strict origin
    StrictOrigin,
    /// Strict origin when cross origin
    StrictOriginWhenCrossOrigin,
    /// Unsafe URL
    UnsafeUrl,
}

/// Format detection settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormatDetection {
    /// Detect email addresses
    pub email: Option<bool>,
    /// Detect phone numbers
    pub telephone: Option<bool>,
    /// Detect addresses
    pub address: Option<bool>,
    /// Detect dates
    pub date: Option<bool>,
}

/// Alternate link for different languages/regions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlternateLink {
    /// Link URL
    pub url: String,
    /// Language code
    pub hreflang: Option<String>,
    /// Media type
    pub media: Option<String>,
    /// Link type
    pub type_: Option<String>,
}

/// JSON-LD structured data
#[cfg(feature = "json-ld")]
pub type JsonLd = serde_json::Value;

#[cfg(not(feature = "json-ld"))]
pub type JsonLd = String;

//! Open Graph metadata types
//!
//! This module contains all Open Graph related types and structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenGraph metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenGraph {
    /// OpenGraph title
    pub title: Option<String>,
    /// OpenGraph description
    pub description: Option<String>,
    /// OpenGraph image
    pub image: Option<OgImage>,
    /// OpenGraph video
    pub video: Option<OgVideo>,
    /// OpenGraph audio
    pub audio: Option<OgAudio>,
    /// OpenGraph URL
    pub url: Option<String>,
    /// Site name
    pub site_name: Option<String>,
    /// Locale
    pub locale: Option<String>,
    /// Article metadata
    pub article: Option<Article>,
    /// Profile metadata
    pub profile: Option<Profile>,
    /// Book metadata
    pub book: Option<Book>,
    /// Additional OpenGraph properties
    pub other: HashMap<String, String>,
}

/// OpenGraph image
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub type_: Option<String>,
}

/// OpenGraph video
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OgVideo {
    /// Video URL
    pub url: String,
    /// Video width
    pub width: Option<u32>,
    /// Video height
    pub height: Option<u32>,
    /// Video type
    pub type_: Option<String>,
    /// Video alt text
    pub alt: Option<String>,
}

/// OpenGraph audio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OgAudio {
    /// Audio URL
    pub url: String,
    /// Audio type
    pub type_: Option<String>,
}

/// Article metadata for OpenGraph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    /// Publication time
    pub published_time: Option<String>,
    /// Modification time
    pub modified_time: Option<String>,
    /// Expiration time
    pub expiration_time: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Section
    pub section: Option<String>,
    /// Tags
    pub tag: Option<Vec<String>>,
}

/// Profile metadata for OpenGraph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Book metadata for OpenGraph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    /// Author
    pub author: Option<String>,
    /// ISBN
    pub isbn: Option<String>,
    /// Release date
    pub release_date: Option<String>,
    /// Tags
    pub tag: Option<Vec<String>>,
}

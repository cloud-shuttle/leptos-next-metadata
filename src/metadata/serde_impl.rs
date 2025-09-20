//! Serialization and From trait implementations for metadata types
//!
//! This module provides From trait implementations and serialization helpers
//! for converting between different types in the metadata system.

use super::types::*;

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
        AdditionalValue::Object(
            v.as_object()
                .unwrap_or(&serde_json::Map::new())
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        )
    }
}

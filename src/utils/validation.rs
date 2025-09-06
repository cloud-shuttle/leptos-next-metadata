//! Validation utilities for metadata fields

use crate::Result;
use regex::Regex;
use once_cell::sync::Lazy;

/// Email validation regex
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// URL validation regex
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap()
});

/// Validate email address
pub fn validate_email(email: &str) -> Result<()> {
    if EMAIL_REGEX.is_match(email) {
        Ok(())
    } else {
        Err(crate::Error::ValidationError(format!("Invalid email: {}", email)))
    }
}

/// Validate URL
pub fn validate_url(url: &str) -> Result<()> {
    if URL_REGEX.is_match(url) || url.starts_with('/') {
        Ok(())
    } else {
        Err(crate::Error::ValidationError(format!("Invalid URL: {}", url)))
    }
}

/// Validate string length
pub fn validate_length(value: &str, min: usize, max: usize, field: &str) -> Result<()> {
    let len = value.len();
    if len < min {
        Err(crate::Error::ValidationError(format!(
            "{} too short: {} characters (minimum {})",
            field, len, min
        )))
    } else if len > max {
        Err(crate::Error::ValidationError(format!(
            "{} too long: {} characters (maximum {})",
            field, len, max
        )))
    } else {
        Ok(())
    }
}

/// Validate that a string is not empty
pub fn validate_not_empty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(crate::Error::ValidationError(format!("{} cannot be empty", field)))
    } else {
        Ok(())
    }
}

/// Validate image dimensions
pub fn validate_image_dimensions(width: u32, height: u32) -> Result<()> {
    if width == 0 || height == 0 {
        return Err(crate::Error::ValidationError(
            "Image dimensions must be greater than 0".to_string(),
        ));
    }

    if width > 4096 || height > 4096 {
        return Err(crate::Error::ValidationError(
            "Image dimensions too large (maximum 4096x4096)".to_string(),
        ));
    }

    Ok(())
}

/// Validate keywords list
pub fn validate_keywords(keywords: &[String]) -> Result<()> {
    if keywords.is_empty() {
        return Ok(());
    }

    for keyword in keywords {
        validate_not_empty(keyword, "keyword")?;
        validate_length(keyword, 1, 50, "keyword")?;
    }

    Ok(())
}

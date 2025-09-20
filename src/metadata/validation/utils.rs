//! Validation utility functions
//!
//! This module provides utility functions for metadata validation,
//! including URL validation and specific field validators.

use super::types::*;
use crate::metadata::*;

/// Utility functions for validation
pub struct ValidationUtils;

impl ValidationUtils {
    /// Check if a URL is valid
    pub fn is_valid_url(url: &str) -> bool {
        if url.starts_with("http://") || url.starts_with("https://") {
            url::Url::parse(url).is_ok()
        } else if url.starts_with('/') {
            true // Relative URLs are valid
        } else {
            false
        }
    }
}

/// Validator for specific metadata types
pub struct MetadataValidator;

impl MetadataValidator {
    /// Validate a title string
    pub fn validate_title(title: &str) -> Vec<ValidationWarning> {
        let mut warnings = Vec::new();

        if title.len() < 10 {
            warnings.push(ValidationWarning {
                code: ValidationWarningCode::CouldImprove,
                message: "Title is quite short".to_string(),
                field: Some("title".to_string()),
                suggestion: Some(
                    "Consider making the title more descriptive (10-60 characters)".to_string(),
                ),
            });
        }

        if title.len() > 60 {
            warnings.push(ValidationWarning {
                code: ValidationWarningCode::CouldImprove,
                message: "Title is quite long".to_string(),
                field: Some("title".to_string()),
                suggestion: Some(
                    "Consider shortening the title to under 60 characters".to_string(),
                ),
            });
        }

        warnings
    }

    /// Validate a description string
    pub fn validate_description(description: &str) -> Vec<ValidationWarning> {
        let mut warnings = Vec::new();

        if description.len() < 50 {
            warnings.push(ValidationWarning {
                code: ValidationWarningCode::CouldImprove,
                message: "Description is quite short".to_string(),
                field: Some("description".to_string()),
                suggestion: Some(
                    "Consider making the description more detailed (50-160 characters)".to_string(),
                ),
            });
        }

        if description.len() > 160 {
            warnings.push(ValidationWarning {
                code: ValidationWarningCode::CouldImprove,
                message: "Description is quite long".to_string(),
                field: Some("description".to_string()),
                suggestion: Some(
                    "Consider shortening the description to under 160 characters".to_string(),
                ),
            });
        }

        warnings
    }

    /// Validate an Open Graph image
    pub fn validate_og_image(image: &OgImage) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if url::Url::parse(&image.url).is_err() {
            errors.push(ValidationError {
                code: ValidationErrorCode::InvalidUrl,
                message: format!("Invalid Open Graph image URL: {}", image.url),
                field: Some("url".to_string()),
                suggestion: Some("Provide a valid absolute URL".to_string()),
            });
        }

        if image.width.is_none() || image.height.is_none() {
            errors.push(ValidationError {
                code: ValidationErrorCode::MissingRequired,
                message: "Open Graph image dimensions are missing".to_string(),
                field: Some("dimensions".to_string()),
                suggestion: Some("Add width and height for better performance".to_string()),
            });
        }

        errors
    }
}

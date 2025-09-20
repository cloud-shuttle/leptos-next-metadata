//! Core validation logic for metadata
//!
//! This module provides the main validation functionality for metadata
//! to ensure it follows SEO best practices and is properly formatted.

use super::types::*;
use super::utils::ValidationUtils;
use crate::metadata::*;

impl Metadata {
    /// Validate this metadata instance
    ///
    /// Returns a `ValidationResult` with any issues found and an overall score.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_next_metadata::metadata::Metadata;
    ///
    /// let metadata = Metadata::with_title("My Page");
    /// let result = metadata.validate();
    ///
    /// if !result.is_valid() {
    ///     for error in &result.errors {
    ///         eprintln!("Error: {}", error.message);
    ///     }
    /// }
    /// ```
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate title
        self.validate_title(&mut result);

        // Validate description
        self.validate_description(&mut result);

        // Validate Open Graph
        if let Some(ref og) = self.open_graph {
            self.validate_open_graph(og, &mut result);
        }

        // Validate Twitter
        if let Some(ref twitter) = self.twitter {
            self.validate_twitter(twitter, &mut result);
        }

        // Validate JSON-LD
        if let Some(ref json_ld) = self.json_ld {
            self.validate_json_ld(json_ld, &mut result);
        }

        // Validate URLs
        self.validate_urls(&mut result);

        // Validate robots
        if let Some(ref robots) = self.robots {
            self.validate_robots(robots, &mut result);
        }

        // Check for missing recommended fields
        self.check_missing_recommended(&mut result);

        result
    }

    /// Validate the title field
    fn validate_title(&self, result: &mut ValidationResult) {
        if let Some(ref title) = self.title {
            match title {
                Title::Static(s) => {
                    if s.is_empty() {
                        result.add_error(ValidationError {
                            code: ValidationErrorCode::MissingRequired,
                            message: "Title cannot be empty".to_string(),
                            field: Some("title".to_string()),
                            suggestion: Some(
                                "Provide a descriptive title for the page".to_string(),
                            ),
                        });
                    } else if s.len() < 10 {
                        result.add_warning(ValidationWarning {
                            code: ValidationWarningCode::CouldImprove,
                            message: "Title is quite short".to_string(),
                            field: Some("title".to_string()),
                            suggestion: Some(
                                "Consider making the title more descriptive (10-60 characters)"
                                    .to_string(),
                            ),
                        });
                    } else if s.len() > 60 {
                        result.add_warning(ValidationWarning {
                            code: ValidationWarningCode::CouldImprove,
                            message: "Title is quite long".to_string(),
                            field: Some("title".to_string()),
                            suggestion: Some(
                                "Consider shortening the title to under 60 characters".to_string(),
                            ),
                        });
                    }
                }
                Title::Template { template, default } => {
                    if template.is_empty() {
                        result.add_error(ValidationError {
                            code: ValidationErrorCode::InvalidFormat,
                            message: "Title template cannot be empty".to_string(),
                            field: Some("title.template".to_string()),
                            suggestion: Some("Provide a valid template string".to_string()),
                        });
                    }
                    if default.is_empty() {
                        result.add_error(ValidationError {
                            code: ValidationErrorCode::MissingRequired,
                            message: "Title default value cannot be empty".to_string(),
                            field: Some("title.default".to_string()),
                            suggestion: Some("Provide a fallback title".to_string()),
                        });
                    }
                }
            }
        } else {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Title is missing".to_string(),
                field: Some("title".to_string()),
                suggestion: Some("Add a title for better SEO".to_string()),
            });
        }
    }

    /// Validate the description field
    fn validate_description(&self, result: &mut ValidationResult) {
        if let Some(ref description) = self.description {
            if description.is_empty() {
                result.add_error(ValidationError {
                    code: ValidationErrorCode::MissingRequired,
                    message: "Description cannot be empty".to_string(),
                    field: Some("description".to_string()),
                    suggestion: Some("Provide a meaningful description".to_string()),
                });
            } else if description.len() < 50 {
                result.add_warning(ValidationWarning {
                    code: ValidationWarningCode::CouldImprove,
                    message: "Description is quite short".to_string(),
                    field: Some("description".to_string()),
                    suggestion: Some(
                        "Consider making the description more detailed (50-160 characters)"
                            .to_string(),
                    ),
                });
            } else if description.len() > 160 {
                result.add_warning(ValidationWarning {
                    code: ValidationWarningCode::CouldImprove,
                    message: "Description is quite long".to_string(),
                    field: Some("description".to_string()),
                    suggestion: Some(
                        "Consider shortening the description to under 160 characters".to_string(),
                    ),
                });
            }
        } else {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Description is missing".to_string(),
                field: Some("description".to_string()),
                suggestion: Some("Add a description for better SEO".to_string()),
            });
        }
    }

    /// Validate Open Graph metadata
    fn validate_open_graph(&self, og: &OpenGraph, result: &mut ValidationResult) {
        // Check for required OG fields
        if og.title.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Open Graph title is missing".to_string(),
                field: Some("openGraph.title".to_string()),
                suggestion: Some("Add an Open Graph title for better social sharing".to_string()),
            });
        }

        if og.description.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Open Graph description is missing".to_string(),
                field: Some("openGraph.description".to_string()),
                suggestion: Some(
                    "Add an Open Graph description for better social sharing".to_string(),
                ),
            });
        }

        // Validate image
        if og.image.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Open Graph image is missing".to_string(),
                field: Some("openGraph.image".to_string()),
                suggestion: Some("Add an Open Graph image for better social sharing".to_string()),
            });
        } else if let Some(image) = &og.image {
            if !ValidationUtils::is_valid_url(&image.url) {
                result.add_error(ValidationError {
                    code: ValidationErrorCode::InvalidUrl,
                    message: format!("Invalid Open Graph image URL: {}", image.url),
                    field: Some("openGraph.image.url".to_string()),
                    suggestion: Some("Provide a valid absolute URL".to_string()),
                });
            }

            if image.width.is_none() || image.height.is_none() {
                result.add_warning(ValidationWarning {
                    code: ValidationWarningCode::CouldImprove,
                    message: "Open Graph image dimensions are missing".to_string(),
                    field: Some("openGraph.image".to_string()),
                    suggestion: Some("Add width and height for better performance".to_string()),
                });
            }
        }

        // Note: OpenGraph type validation removed as the current struct doesn't have a type field
    }

    /// Validate Twitter metadata
    fn validate_twitter(&self, twitter: &Twitter, result: &mut ValidationResult) {
        if twitter.card.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Twitter card type is missing".to_string(),
                field: Some("twitter.card".to_string()),
                suggestion: Some("Add a Twitter card type for better Twitter sharing".to_string()),
            });
        }

        if let Some(ref image) = twitter.image {
            if !ValidationUtils::is_valid_url(image) {
                result.add_error(ValidationError {
                    code: ValidationErrorCode::InvalidUrl,
                    message: format!("Invalid Twitter image URL: {}", image),
                    field: Some("twitter.image".to_string()),
                    suggestion: Some("Provide a valid absolute URL".to_string()),
                });
            }
        }
    }

    /// Validate JSON-LD structured data
    #[cfg(feature = "json-ld")]
    fn validate_json_ld(&self, json_ld: &crate::metadata::JsonLd, result: &mut ValidationResult) {
        // Basic JSON-LD validation
        if let Some(schema_type) = json_ld.get("@type") {
            if let Some(type_str) = schema_type.as_str() {
                let valid_types = [
                    "Article",
                    "BlogPosting",
                    "WebPage",
                    "Product",
                    "Organization",
                    "Person",
                    "Event",
                    "Recipe",
                    "Review",
                ];
                if !valid_types.contains(&type_str) {
                    result.add_warning(ValidationWarning {
                        code: ValidationWarningCode::CouldImprove,
                        message: format!("JSON-LD type '{}' may not be standard", type_str),
                        field: Some("jsonLd.@type".to_string()),
                        suggestion: Some("Consider using a standard Schema.org type".to_string()),
                    });
                }
            }
        } else {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::CouldImprove,
                message: "JSON-LD @type is missing".to_string(),
                field: Some("jsonLd.@type".to_string()),
                suggestion: Some("Add @type for proper structured data".to_string()),
            });
        }
    }

    /// Validate JSON-LD structured data (fallback when json-ld feature is disabled)
    #[cfg(not(feature = "json-ld"))]
    fn validate_json_ld(&self, _json_ld: &crate::metadata::JsonLd, _result: &mut ValidationResult) {
        // No validation when json-ld feature is disabled
    }

    /// Validate URLs in the metadata
    fn validate_urls(&self, result: &mut ValidationResult) {
        if let Some(ref canonical) = self.canonical {
            if !ValidationUtils::is_valid_url(canonical) {
                result.add_error(ValidationError {
                    code: ValidationErrorCode::InvalidUrl,
                    message: format!("Invalid canonical URL: {}", canonical),
                    field: Some("canonical".to_string()),
                    suggestion: Some("Provide a valid absolute URL".to_string()),
                });
            }
        }

        if !self.alternate_links.is_empty() {
            for link in &self.alternate_links {
                if !ValidationUtils::is_valid_url(&link.url) {
                    result.add_error(ValidationError {
                        code: ValidationErrorCode::InvalidUrl,
                        message: format!("Invalid alternate URL: {}", link.url),
                        field: Some("alternate_links.url".to_string()),
                        suggestion: Some("Provide a valid absolute URL".to_string()),
                    });
                }
            }
        }
    }

    /// Validate robots directives
    fn validate_robots(&self, _robots: &Robots, _result: &mut ValidationResult) {
        // Note: robots validation simplified as the current struct has limited fields
        // Could add validation for index/follow values if needed
    }

    /// Check for missing recommended fields
    fn check_missing_recommended(&self, result: &mut ValidationResult) {
        if self.keywords.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Keywords are missing".to_string(),
                field: Some("keywords".to_string()),
                suggestion: Some("Add relevant keywords for better SEO".to_string()),
            });
        }

        if self.authors.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Authors are missing".to_string(),
                field: Some("authors".to_string()),
                suggestion: Some("Add author information for better attribution".to_string()),
            });
        }

        if self.viewport.is_none() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Viewport is missing".to_string(),
                field: Some("viewport".to_string()),
                suggestion: Some("Add viewport settings for better mobile experience".to_string()),
            });
        }
    }
}

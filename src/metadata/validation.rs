//! Metadata validation functionality for leptos-next-metadata
//!
//! This module provides validation for metadata to ensure it follows
//! SEO best practices and is properly formatted.

use super::*;

/// Validation result containing warnings and errors
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Validation errors that should be fixed
    pub errors: Vec<ValidationError>,

    /// Validation warnings that could be improved
    pub warnings: Vec<ValidationWarning>,

    /// Overall validation score (0-100)
    pub score: u8,
}

/// Validation error that should be fixed
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error code for programmatic handling
    pub code: ValidationErrorCode,

    /// Human-readable error message
    pub message: String,

    /// Field that caused the error
    pub field: Option<String>,

    /// Suggested fix for the error
    pub suggestion: Option<String>,
}

/// Validation warning that could be improved
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Warning code for programmatic handling
    pub code: ValidationWarningCode,

    /// Human-readable warning message
    pub message: String,

    /// Field that caused the warning
    pub field: Option<String>,

    /// Suggested improvement
    pub suggestion: Option<String>,
}

/// Error codes for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorCode {
    /// Missing required field
    MissingRequired,

    /// Invalid URL format
    InvalidUrl,

    /// Invalid email format
    InvalidEmail,

    /// Invalid date format
    InvalidDate,

    /// Field too long
    FieldTooLong,

    /// Field too short
    FieldTooShort,

    /// Invalid character in field
    InvalidCharacters,

    /// Duplicate value
    DuplicateValue,

    /// Invalid format
    InvalidFormat,
}

/// Warning codes for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationWarningCode {
    /// Field could be optimized
    CouldOptimize,

    /// Missing recommended field
    MissingRecommended,

    /// Field value could be improved
    CouldImprove,

    /// Performance consideration
    PerformanceConsideration,

    /// Accessibility consideration
    AccessibilityConsideration,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    /// Add an error to the validation result
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.update_score();
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
        self.update_score();
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if validation has warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the overall validation status
    pub fn status(&self) -> ValidationStatus {
        if self.errors.is_empty() && self.warnings.is_empty() {
            ValidationStatus::Perfect
        } else if self.errors.is_empty() {
            ValidationStatus::Good
        } else if self.score >= 70 {
            ValidationStatus::Fair
        } else {
            ValidationStatus::Poor
        }
    }

    /// Update the validation score based on errors and warnings
    fn update_score(&mut self) {
        let error_penalty = self.errors.len() * 15; // Each error costs 15 points
        let warning_penalty = self.warnings.len() * 5; // Each warning costs 5 points

        let total_penalty = error_penalty + warning_penalty;
        self.score = if total_penalty >= 100 {
            0
        } else {
            100 - total_penalty as u8
        };
    }
}

/// Overall validation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    /// Perfect score (100)
    Perfect,

    /// Good score (80-99)
    Good,

    /// Fair score (70-79)
    Fair,

    /// Poor score (0-69)
    Poor,
}

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

        // Validate images
        if og.images.is_empty() {
            result.add_warning(ValidationWarning {
                code: ValidationWarningCode::MissingRecommended,
                message: "Open Graph images are missing".to_string(),
                field: Some("openGraph.images".to_string()),
                suggestion: Some(
                    "Add at least one Open Graph image for better social sharing".to_string(),
                ),
            });
        } else {
            for (i, image) in og.images.iter().enumerate() {
                if !self.is_valid_url(&image.url) {
                    result.add_error(ValidationError {
                        code: ValidationErrorCode::InvalidUrl,
                        message: format!("Invalid Open Graph image URL: {}", image.url),
                        field: Some(format!("openGraph.images[{}].url", i)),
                        suggestion: Some("Provide a valid absolute URL".to_string()),
                    });
                }

                if image.width.is_none() || image.height.is_none() {
                    result.add_warning(ValidationWarning {
                        code: ValidationWarningCode::CouldImprove,
                        message: "Open Graph image dimensions are missing".to_string(),
                        field: Some(format!("openGraph.images[{}]", i)),
                        suggestion: Some("Add width and height for better performance".to_string()),
                    });
                }
            }
        }

        // Validate type
        if let Some(ref og_type) = og.r#type {
            let valid_types = [
                "website",
                "article",
                "book",
                "profile",
                "music.song",
                "music.album",
                "music.playlist",
                "music.radio_station",
                "video.movie",
                "video.episode",
                "video.tv_show",
                "video.other",
            ];
            if !valid_types.contains(&og_type.as_str()) {
                result.add_warning(ValidationWarning {
                    code: ValidationWarningCode::CouldImprove,
                    message: format!("Open Graph type '{}' may not be standard", og_type),
                    field: Some("openGraph.type".to_string()),
                    suggestion: Some("Consider using a standard Open Graph type".to_string()),
                });
            }
        }
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
            if !self.is_valid_url(image) {
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
            if !self.is_valid_url(canonical) {
                result.add_error(ValidationError {
                    code: ValidationErrorCode::InvalidUrl,
                    message: format!("Invalid canonical URL: {}", canonical),
                    field: Some("canonical".to_string()),
                    suggestion: Some("Provide a valid absolute URL".to_string()),
                });
            }
        }

        if let Some(ref alternates) = self.alternates {
            for (hreflang, link) in alternates {
                if !self.is_valid_url(&link.href) {
                    result.add_error(ValidationError {
                        code: ValidationErrorCode::InvalidUrl,
                        message: format!("Invalid alternate URL for {}: {}", hreflang, link.href),
                        field: Some(format!("alternates.{}.href", hreflang)),
                        suggestion: Some("Provide a valid absolute URL".to_string()),
                    });
                }
            }
        }
    }

    /// Validate robots directives
    fn validate_robots(&self, robots: &Robots, result: &mut ValidationResult) {
        if let Some(delay) = robots.crawl_delay {
            if delay > 10 {
                result.add_warning(ValidationWarning {
                    code: ValidationWarningCode::PerformanceConsideration,
                    message: "Crawl delay is quite high".to_string(),
                    field: Some("robots.crawlDelay".to_string()),
                    suggestion: Some(
                        "Consider reducing crawl delay for better indexing".to_string(),
                    ),
                });
            }
        }
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

    /// Check if a URL is valid
    fn is_valid_url(&self, url: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_validation() {
        let metadata = Metadata::with_title("");
        let result = metadata.validate();

        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.field.as_ref().unwrap() == "title"));
    }

    #[test]
    fn test_description_validation() {
        let metadata = Metadata::with_title_and_description("Title", "Short");
        let result = metadata.validate();

        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.field.as_ref().unwrap() == "description"));
    }

    #[test]
    fn test_open_graph_validation() {
        let og = OpenGraph {
            images: vec![OgImage::new("invalid-url")],
            ..Default::default()
        };

        let metadata = Metadata::default().open_graph(og);
        let result = metadata.validate();

        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e
            .field
            .as_ref()
            .unwrap()
            .contains("openGraph.images")));
    }

    #[test]
    fn test_validation_score() {
        let metadata = Metadata::default();
        let result = metadata.validate();

        // Should have warnings but no errors
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result.score < 100);
    }

    #[test]
    fn test_validator_utility_functions() {
        let title_warnings = MetadataValidator::validate_title("Short");
        assert!(!title_warnings.is_empty());

        let desc_warnings = MetadataValidator::validate_description("Short description");
        assert!(!desc_warnings.is_empty());

        let image_errors = MetadataValidator::validate_og_image(&OgImage::new("invalid-url"));
        assert!(!image_errors.is_empty());
    }
}

//! API contract validation and middleware
//!
//! This module provides contract-first development support for the metadata API,
//! including request/response validation, OpenAPI schema validation, and
//! runtime contract enforcement.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

/// API contract validation middleware
pub struct ContractValidator {
    /// OpenAPI schema for validation
    #[allow(dead_code)]
    schema: openapiv3::OpenAPI,
    /// Validation rules
    rules: ValidationRules,
}

/// Validation rules configuration
#[derive(Debug, Clone)]
pub struct ValidationRules {
    /// Strict mode - fail on any validation error
    pub strict: bool,
    /// Allow additional properties not in schema
    pub allow_additional_properties: bool,
    /// Maximum request size in bytes
    pub max_request_size: usize,
    /// Maximum response size in bytes
    pub max_response_size: usize,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            strict: true,
            allow_additional_properties: false,
            max_request_size: 1024 * 1024,       // 1MB
            max_response_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl ContractValidator {
    /// Create a new contract validator with OpenAPI schema
    pub fn new(schema: openapiv3::OpenAPI) -> Self {
        Self {
            schema,
            rules: ValidationRules::default(),
        }
    }

    /// Create a new contract validator with custom rules
    pub fn with_rules(schema: openapiv3::OpenAPI, rules: ValidationRules) -> Self {
        Self { schema, rules }
    }

    /// Validate a request against the OpenAPI schema
    pub fn validate_request<T>(&self, request: &T) -> Result<ValidationResult>
    where
        T: Serialize + Validate,
    {
        // First, run custom validation rules
        let custom_validation = request.validate();
        if let Err(errors) = custom_validation {
            return Ok(ValidationResult {
                is_valid: false,
                errors: self.convert_validation_errors(errors),
                warnings: Vec::new(),
                score: 0,
            });
        }

        // Then, validate against OpenAPI schema
        self.validate_against_schema(request)
    }

    /// Validate a response against the OpenAPI schema
    pub fn validate_response<T>(&self, response: &T) -> Result<ValidationResult>
    where
        T: Serialize,
    {
        self.validate_against_schema(response)
    }

    /// Validate against OpenAPI schema
    fn validate_against_schema<T>(&self, data: &T) -> Result<ValidationResult>
    where
        T: Serialize,
    {
        // Convert to JSON for schema validation
        let json_value = serde_json::to_value(data)
            .map_err(|e| Error::ValidationError(format!("Serialization error: {}", e)))?;

        // Perform schema validation
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Basic validation checks
        self.validate_required_fields(&json_value, &mut errors);
        self.validate_field_types(&json_value, &mut errors);
        self.validate_field_constraints(&json_value, &mut errors, &mut warnings);

        let score = self.calculate_validation_score(&errors, &warnings);
        let is_valid = errors.is_empty() || !self.rules.strict;

        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
            score,
        })
    }

    /// Validate required fields
    #[allow(clippy::ptr_arg)]
    fn validate_required_fields(
        &self,
        data: &serde_json::Value,
        errors: &mut Vec<ValidationErrorDetail>,
    ) {
        // This would be implemented with actual OpenAPI schema validation
        // For now, we'll do basic checks
        if let Some(obj) = data.as_object() {
            // Check for required fields based on common patterns
            if !obj.contains_key("title") {
                errors.push(ValidationErrorDetail {
                    code: "MISSING_REQUIRED_FIELD".to_string(),
                    message: "Title is required".to_string(),
                    field: Some("title".to_string()),
                    suggestion: Some("Provide a title for the metadata".to_string()),
                });
            }

            if !obj.contains_key("url") {
                errors.push(ValidationErrorDetail {
                    code: "MISSING_REQUIRED_FIELD".to_string(),
                    message: "URL is required".to_string(),
                    field: Some("url".to_string()),
                    suggestion: Some("Provide a canonical URL for the page".to_string()),
                });
            }
        }
    }

    /// Validate field types
    #[allow(clippy::ptr_arg)]
    fn validate_field_types(
        &self,
        data: &serde_json::Value,
        errors: &mut Vec<ValidationErrorDetail>,
    ) {
        if let Some(obj) = data.as_object() {
            // Validate title type and length
            if let Some(title) = obj.get("title") {
                if let Some(title_str) = title.as_str() {
                    if title_str.len() > 60 {
                        errors.push(ValidationErrorDetail {
                            code: "FIELD_TOO_LONG".to_string(),
                            message: "Title is too long".to_string(),
                            field: Some("title".to_string()),
                            suggestion: Some(
                                "Keep title under 60 characters for better SEO".to_string(),
                            ),
                        });
                    }
                } else {
                    errors.push(ValidationErrorDetail {
                        code: "INVALID_TYPE".to_string(),
                        message: "Title must be a string".to_string(),
                        field: Some("title".to_string()),
                        suggestion: Some("Provide title as a string value".to_string()),
                    });
                }
            }

            // Validate description length
            if let Some(description) = obj.get("description") {
                if let Some(desc_str) = description.as_str() {
                    if desc_str.len() > 160 {
                        errors.push(ValidationErrorDetail {
                            code: "FIELD_TOO_LONG".to_string(),
                            message: "Description is too long".to_string(),
                            field: Some("description".to_string()),
                            suggestion: Some(
                                "Keep description under 160 characters for better SEO".to_string(),
                            ),
                        });
                    }
                }
            }

            // Validate URL format
            if let Some(url) = obj.get("url") {
                if let Some(url_str) = url.as_str() {
                    if !self.is_valid_url(url_str) {
                        errors.push(ValidationErrorDetail {
                            code: "INVALID_URL".to_string(),
                            message: "Invalid URL format".to_string(),
                            field: Some("url".to_string()),
                            suggestion: Some(
                                "Provide a valid URL starting with http:// or https://".to_string(),
                            ),
                        });
                    }
                }
            }
        }
    }

    /// Validate field constraints
    #[allow(clippy::ptr_arg)]
    fn validate_field_constraints(
        &self,
        data: &serde_json::Value,
        errors: &mut Vec<ValidationErrorDetail>,
        warnings: &mut Vec<ValidationWarningDetail>,
    ) {
        if let Some(obj) = data.as_object() {
            // Check for missing recommended fields
            if !obj.contains_key("description") {
                warnings.push(ValidationWarningDetail {
                    code: "MISSING_RECOMMENDED_FIELD".to_string(),
                    message: "Description is recommended for SEO".to_string(),
                    field: Some("description".to_string()),
                    suggestion: Some(
                        "Add a description to improve search engine visibility".to_string(),
                    ),
                });
            }

            if !obj.contains_key("image") {
                warnings.push(ValidationWarningDetail {
                    code: "MISSING_RECOMMENDED_FIELD".to_string(),
                    message: "Image is recommended for social sharing".to_string(),
                    field: Some("image".to_string()),
                    suggestion: Some(
                        "Add an image URL for better social media sharing".to_string(),
                    ),
                });
            }

            // Validate Open Graph configuration
            if let Some(og) = obj.get("openGraph") {
                self.validate_open_graph(og, errors, warnings);
            }

            // Validate Twitter configuration
            if let Some(twitter) = obj.get("twitter") {
                self.validate_twitter_config(twitter, errors, warnings);
            }
        }
    }

    /// Validate Open Graph configuration
    #[allow(clippy::ptr_arg)]
    fn validate_open_graph(
        &self,
        og: &serde_json::Value,
        _errors: &mut Vec<ValidationErrorDetail>,
        warnings: &mut Vec<ValidationWarningDetail>,
    ) {
        if let Some(og_obj) = og.as_object() {
            // Check for required OG fields
            if !og_obj.contains_key("type") {
                warnings.push(ValidationWarningDetail {
                    code: "MISSING_OG_TYPE".to_string(),
                    message: "Open Graph type is recommended".to_string(),
                    field: Some("openGraph.type".to_string()),
                    suggestion: Some(
                        "Specify the Open Graph object type (e.g., 'website', 'article')"
                            .to_string(),
                    ),
                });
            }

            if !og_obj.contains_key("images") {
                warnings.push(ValidationWarningDetail {
                    code: "MISSING_OG_IMAGES".to_string(),
                    message: "Open Graph images are recommended".to_string(),
                    field: Some("openGraph.images".to_string()),
                    suggestion: Some("Add Open Graph images for better social sharing".to_string()),
                });
            }
        }
    }

    /// Validate Twitter configuration
    #[allow(clippy::ptr_arg)]
    fn validate_twitter_config(
        &self,
        twitter: &serde_json::Value,
        _errors: &mut Vec<ValidationErrorDetail>,
        warnings: &mut Vec<ValidationWarningDetail>,
    ) {
        if let Some(twitter_obj) = twitter.as_object() {
            if !twitter_obj.contains_key("card") {
                warnings.push(ValidationWarningDetail {
                    code: "MISSING_TWITTER_CARD".to_string(),
                    message: "Twitter card type is recommended".to_string(),
                    field: Some("twitter.card".to_string()),
                    suggestion: Some(
                        "Specify Twitter card type (e.g., 'summary', 'summary_large_image')"
                            .to_string(),
                    ),
                });
            }
        }
    }

    /// Calculate validation score
    fn calculate_validation_score(
        &self,
        errors: &[ValidationErrorDetail],
        warnings: &[ValidationWarningDetail],
    ) -> u8 {
        let error_penalty = errors.len() * 20; // Each error costs 20 points
        let warning_penalty = warnings.len() * 5; // Each warning costs 5 points

        let total_penalty = error_penalty + warning_penalty;
        if total_penalty >= 100 {
            0
        } else {
            100 - total_penalty as u8
        }
    }

    /// Convert validator errors to our format
    fn convert_validation_errors(&self, errors: ValidationErrors) -> Vec<ValidationErrorDetail> {
        let mut result = Vec::new();

        for (field, field_errors) in errors.field_errors() {
            for error in field_errors {
                result.push(ValidationErrorDetail {
                    code: error.code.to_string(),
                    message: error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Validation error for field '{}'", field)),
                    field: Some(field.to_string()),
                    suggestion: self.get_suggestion_for_code(&error.code),
                });
            }
        }

        result
    }

    /// Get suggestion for validation error code
    fn get_suggestion_for_code(&self, code: &str) -> Option<String> {
        match code {
            "length" => Some("Check the length of the field".to_string()),
            "range" => Some("Check the value is within the allowed range".to_string()),
            "url" => Some("Provide a valid URL".to_string()),
            "email" => Some("Provide a valid email address".to_string()),
            _ => None,
        }
    }

    /// Check if URL is valid
    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationErrorDetail>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarningDetail>,
    /// Overall validation score (0-100)
    pub score: u8,
}

/// Validation error detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrorDetail {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Field that caused the error
    pub field: Option<String>,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Validation warning detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarningDetail {
    /// Warning code
    pub code: String,
    /// Warning message
    pub message: String,
    /// Field that caused the warning
    pub field: Option<String>,
    /// Suggested improvement
    pub suggestion: Option<String>,
}

/// Contract validation middleware for Axum
pub struct ContractMiddleware {
    validator: ContractValidator,
}

impl ContractMiddleware {
    /// Create new contract middleware
    pub fn new(validator: ContractValidator) -> Self {
        Self { validator }
    }

    /// Validate request middleware
    pub async fn validate_request<T>(&self, request: &T) -> Result<ValidationResult>
    where
        T: Serialize + Validate,
    {
        self.validator.validate_request(request)
    }

    /// Validate response middleware
    pub async fn validate_response<T>(&self, response: &T) -> Result<ValidationResult>
    where
        T: Serialize,
    {
        self.validator.validate_response(response)
    }
}

/// Load OpenAPI schema from file
pub fn load_openapi_schema(path: &str) -> Result<openapiv3::OpenAPI> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::ValidationError(format!("Failed to read schema file: {}", e)))?;

    let schema: openapiv3::OpenAPI = serde_yaml::from_str(&content)
        .map_err(|e| Error::ValidationError(format!("Failed to parse OpenAPI schema: {}", e)))?;

    Ok(schema)
}

/// Create default contract validator
pub fn create_default_validator() -> Result<ContractValidator> {
    let schema_path = "docs/api/openapi.yaml";
    let schema = load_openapi_schema(schema_path)?;
    Ok(ContractValidator::new(schema))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_validator_creation() {
        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);
        assert!(validator.rules.strict);
    }

    #[test]
    fn test_validation_rules_default() {
        let rules = ValidationRules::default();
        assert!(rules.strict);
        assert!(!rules.allow_additional_properties);
        assert_eq!(rules.max_request_size, 1024 * 1024);
    }

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        };

        assert!(result.is_valid);
        assert_eq!(result.score, 100);
    }

    #[test]
    fn test_validation_error_detail() {
        let error = ValidationErrorDetail {
            code: "MISSING_REQUIRED_FIELD".to_string(),
            message: "Title is required".to_string(),
            field: Some("title".to_string()),
            suggestion: Some("Provide a title".to_string()),
        };

        assert_eq!(error.code, "MISSING_REQUIRED_FIELD");
        assert_eq!(error.field, Some("title".to_string()));
    }

    #[test]
    fn test_validation_warning_detail() {
        let warning = ValidationWarningDetail {
            code: "MISSING_RECOMMENDED_FIELD".to_string(),
            message: "Description is recommended".to_string(),
            field: Some("description".to_string()),
            suggestion: Some("Add a description".to_string()),
        };

        assert_eq!(warning.code, "MISSING_RECOMMENDED_FIELD");
        assert_eq!(warning.field, Some("description".to_string()));
    }

    #[test]
    fn test_validation_score_calculation() {
        let errors = vec![ValidationErrorDetail {
            code: "ERROR1".to_string(),
            message: "Error 1".to_string(),
            field: None,
            suggestion: None,
        }];
        let warnings = vec![ValidationWarningDetail {
            code: "WARNING1".to_string(),
            message: "Warning 1".to_string(),
            field: None,
            suggestion: None,
        }];

        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &warnings);

        // 1 error (20 points) + 1 warning (5 points) = 25 points penalty
        // Score should be 100 - 25 = 75
        assert_eq!(score, 75);
    }

    #[test]
    fn test_validation_score_max_penalty() {
        let errors = vec![
            ValidationErrorDetail {
                code: "ERROR1".to_string(),
                message: "Error 1".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR2".to_string(),
                message: "Error 2".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR3".to_string(),
                message: "Error 3".to_string(),
                field: None,
                suggestion: None,
            },
        ];

        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &[]);

        // 3 errors * 20 points = 60 points penalty
        // Score should be 100 - 60 = 40
        assert_eq!(score, 40);
    }

    #[test]
    fn test_validation_score_over_100_penalty() {
        let errors = vec![
            ValidationErrorDetail {
                code: "ERROR1".to_string(),
                message: "Error 1".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR2".to_string(),
                message: "Error 2".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR3".to_string(),
                message: "Error 3".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR4".to_string(),
                message: "Error 4".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR5".to_string(),
                message: "Error 5".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR6".to_string(),
                message: "Error 6".to_string(),
                field: None,
                suggestion: None,
            },
        ];

        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &[]);

        // 6 errors * 20 points = 120 points penalty
        // Score should be 0 (capped at 0)
        assert_eq!(score, 0);
    }

    #[test]
    fn test_is_valid_url() {
        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);

        assert!(validator.is_valid_url("https://example.com"));
        assert!(validator.is_valid_url("http://example.com"));
        assert!(!validator.is_valid_url("ftp://example.com"));
        assert!(!validator.is_valid_url("invalid-url"));
    }

    #[test]
    fn test_contract_middleware_creation() {
        let schema = openapiv3::OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let _middleware = ContractMiddleware::new(validator);

        // Test that middleware was created successfully
        // If we get here, creation succeeded
    }
}

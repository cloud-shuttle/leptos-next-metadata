//! Validation rule implementations
//!
//! This module provides pluggable validation rules for API contract validation.

use super::types::ValidationError;
use serde_json::Value;
use std::collections::HashMap;

/// Trait for custom validation rules
pub trait ValidationRule: Send + Sync {
    fn name(&self) -> &str;

    fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError>;

    fn validate_response(
        &self,
        _method: &str,
        _path: &str,
        _status_code: u16,
        _headers: &HashMap<String, String>,
        _body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        // Default: no response validation
        Ok(())
    }
}

/// Validates basic schema compliance
#[derive(Clone)]
pub struct SchemaComplianceRule;

impl ValidationRule for SchemaComplianceRule {
    fn name(&self) -> &str {
        "schema-compliance"
    }

    fn validate_request(
        &self,
        method: &str,
        _path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        // Validate Content-Type header for requests with body
        if body.is_some() && method != "GET" && !headers.contains_key("content-type") {
            return Err(ValidationError::MissingHeader {
                header: "content-type".to_string(),
            });
        }

        Ok(())
    }
}

/// Validates required fields are present
#[derive(Clone)]
pub struct RequiredFieldsRule;

impl ValidationRule for RequiredFieldsRule {
    fn name(&self) -> &str {
        "required-fields"
    }

    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            // Check for common required fields in metadata
            if !obj.contains_key("title") && !obj.contains_key("description") {
                return Err(ValidationError::MissingRequiredField {
                    field: "title or description".to_string(),
                });
            }
        }

        Ok(())
    }
}

/// Validates data types match schema constraints
#[derive(Clone)]
pub struct TypeConstraintsRule;

impl ValidationRule for TypeConstraintsRule {
    fn name(&self) -> &str {
        "type-constraints"
    }

    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            // Validate string fields
            for (key, value) in obj {
                if key.ends_with("_url") {
                    if let Value::String(url) = value {
                        if !self.is_valid_url(url) {
                            return Err(ValidationError::InvalidFormat {
                                field: key.clone(),
                                expected: "valid URL".to_string(),
                                actual: url.clone(),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl TypeConstraintsRule {
    fn is_valid_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
}

/// Validates string formats (email, url, date, etc.)
#[derive(Clone)]
pub struct FormatValidationRule;

impl ValidationRule for FormatValidationRule {
    fn name(&self) -> &str {
        "format-validation"
    }

    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            for (key, value) in obj {
                if let Value::String(s) = value {
                    if key.contains("email") && !self.is_valid_email(s) {
                        return Err(ValidationError::InvalidFormat {
                            field: key.clone(),
                            expected: "valid email".to_string(),
                            actual: s.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl FormatValidationRule {
    fn is_valid_email(&self, email: &str) -> bool {
        // Simple email validation
        email.contains('@') && email.contains('.') && email.len() > 5
    }
}

/// Validates OpenGraph specific constraints
#[derive(Clone)]
pub struct OpenGraphValidationRule;

impl ValidationRule for OpenGraphValidationRule {
    fn name(&self) -> &str {
        "open-graph-validation"
    }

    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            if let Some(Value::Object(og)) = obj.get("open_graph") {
                // Validate OpenGraph image URL
                if let Some(Value::Object(image)) = og.get("image") {
                    if let Some(Value::String(url)) = image.get("url") {
                        if !self.is_valid_url(url) {
                            return Err(ValidationError::InvalidFormat {
                                field: "open_graph.image.url".to_string(),
                                expected: "valid URL".to_string(),
                                actual: url.clone(),
                            });
                        }
                    }
                }

                // Validate OpenGraph title length
                if let Some(Value::String(title)) = og.get("title") {
                    if title.len() > 95 {
                        return Err(ValidationError::FieldTooLong {
                            field: "open_graph.title".to_string(),
                            max_length: 95,
                            actual_length: title.len(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl OpenGraphValidationRule {
    fn is_valid_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
}

/// Validates Twitter Card specific constraints
#[derive(Clone)]
pub struct TwitterValidationRule;

impl ValidationRule for TwitterValidationRule {
    fn name(&self) -> &str {
        "twitter-validation"
    }

    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            if let Some(Value::Object(twitter)) = obj.get("twitter") {
                // Validate Twitter card type
                if let Some(Value::String(card)) = twitter.get("card") {
                    let valid_cards = ["summary", "summary_large_image", "app", "player"];
                    if !valid_cards.contains(&card.as_str()) {
                        return Err(ValidationError::InvalidValue {
                            field: "twitter.card".to_string(),
                            expected: format!("one of: {}", valid_cards.join(", ")),
                            actual: card.clone(),
                        });
                    }
                }

                // Validate Twitter image URL
                if let Some(Value::String(image)) = twitter.get("image") {
                    if !self.is_valid_url(image) {
                        return Err(ValidationError::InvalidFormat {
                            field: "twitter.image".to_string(),
                            expected: "valid URL".to_string(),
                            actual: image.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl TwitterValidationRule {
    fn is_valid_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
}

/// Enum for all validation rules to make them cloneable
#[derive(Clone)]
pub enum ValidationRuleEnum {
    SchemaCompliance(SchemaComplianceRule),
    RequiredFields(RequiredFieldsRule),
    TypeConstraints(TypeConstraintsRule),
    FormatValidation(FormatValidationRule),
    OpenGraphValidation(OpenGraphValidationRule),
    TwitterValidation(TwitterValidationRule),
}

impl ValidationRule for ValidationRuleEnum {
    fn name(&self) -> &str {
        match self {
            ValidationRuleEnum::SchemaCompliance(rule) => rule.name(),
            ValidationRuleEnum::RequiredFields(rule) => rule.name(),
            ValidationRuleEnum::TypeConstraints(rule) => rule.name(),
            ValidationRuleEnum::FormatValidation(rule) => rule.name(),
            ValidationRuleEnum::OpenGraphValidation(rule) => rule.name(),
            ValidationRuleEnum::TwitterValidation(rule) => rule.name(),
        }
    }

    fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        match self {
            ValidationRuleEnum::SchemaCompliance(rule) => {
                rule.validate_request(method, path, headers, body)
            }
            ValidationRuleEnum::RequiredFields(rule) => {
                rule.validate_request(method, path, headers, body)
            }
            ValidationRuleEnum::TypeConstraints(rule) => {
                rule.validate_request(method, path, headers, body)
            }
            ValidationRuleEnum::FormatValidation(rule) => {
                rule.validate_request(method, path, headers, body)
            }
            ValidationRuleEnum::OpenGraphValidation(rule) => {
                rule.validate_request(method, path, headers, body)
            }
            ValidationRuleEnum::TwitterValidation(rule) => {
                rule.validate_request(method, path, headers, body)
            }
        }
    }
}

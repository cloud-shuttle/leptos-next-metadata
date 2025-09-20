//! Validation rules for metadata
//!
//! This module defines the validation rules and constraints
//! for various metadata fields and types.

use super::types::*;

/// Validation rules for metadata fields
pub struct ValidationRules;

impl ValidationRules {
    /// Get validation rules for a specific field
    pub fn get_field_rules(field: &str) -> Vec<ValidationRule> {
        match field {
            "title" => vec![ValidationRule {
                field: "title".to_string(),
                min_length: Some(10),
                max_length: Some(60),
                required: true,
                pattern: None,
            }],
            "description" => vec![ValidationRule {
                field: "description".to_string(),
                min_length: Some(50),
                max_length: Some(160),
                required: true,
                pattern: None,
            }],
            "url" => vec![ValidationRule {
                field: "url".to_string(),
                min_length: None,
                max_length: None,
                required: true,
                pattern: Some(r"^https?://".to_string()),
            }],
            _ => vec![],
        }
    }

    /// Check if a value meets the validation rules
    pub fn validate_value(value: &str, rules: &[ValidationRule]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for rule in rules {
            if rule.required && value.is_empty() {
                errors.push(ValidationError {
                    code: ValidationErrorCode::MissingRequired,
                    message: format!("Field '{}' is required", rule.field),
                    field: Some(rule.field.clone()),
                    suggestion: Some("Provide a value for this field".to_string()),
                });
            }

            if let Some(min_len) = rule.min_length {
                if value.len() < min_len {
                    errors.push(ValidationError {
                        code: ValidationErrorCode::FieldTooShort,
                        message: format!(
                            "Field '{}' is too short (minimum {} characters)",
                            rule.field, min_len
                        ),
                        field: Some(rule.field.clone()),
                        suggestion: Some(format!(
                            "Make the field at least {} characters long",
                            min_len
                        )),
                    });
                }
            }

            if let Some(max_len) = rule.max_length {
                if value.len() > max_len {
                    errors.push(ValidationError {
                        code: ValidationErrorCode::FieldTooLong,
                        message: format!(
                            "Field '{}' is too long (maximum {} characters)",
                            rule.field, max_len
                        ),
                        field: Some(rule.field.clone()),
                        suggestion: Some(format!(
                            "Shorten the field to at most {} characters",
                            max_len
                        )),
                    });
                }
            }

            if let Some(pattern) = &rule.pattern {
                if !regex::Regex::new(pattern).unwrap().is_match(value) {
                    errors.push(ValidationError {
                        code: ValidationErrorCode::InvalidFormat,
                        message: format!("Field '{}' has invalid format", rule.field),
                        field: Some(rule.field.clone()),
                        suggestion: Some("Check the format of this field".to_string()),
                    });
                }
            }
        }

        errors
    }
}

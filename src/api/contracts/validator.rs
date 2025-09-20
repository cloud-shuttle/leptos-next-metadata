//! OpenAPI contract validation engine
//!
//! This module provides the core validation logic for API contracts,
//! including real OpenAPI schema validation and custom validation rules.

use super::rules::{ValidationRule, ValidationRuleEnum};
use super::types::*;
use crate::Result;
use openapiv3::OpenAPI;
use serde_json::Value;
use std::collections::HashMap;

/// OpenAPI contract validation engine
#[derive(Clone)]
pub struct ContractValidator {
    /// OpenAPI specification
    spec: OpenAPI,
    /// Custom validation rules
    rules: Vec<ValidationRuleEnum>,
    /// Validation configuration
    config: ValidationRules,
}

impl ContractValidator {
    /// Create validator from OpenAPI specification
    pub fn new(spec: OpenAPI) -> Result<Self> {
        let mut validator = Self {
            spec,
            rules: Vec::new(),
            config: ValidationRules::default(),
        };

        // Add default validation rules
        validator.add_rule(ValidationRuleEnum::SchemaCompliance(
            super::rules::SchemaComplianceRule,
        ));
        validator.add_rule(ValidationRuleEnum::RequiredFields(
            super::rules::RequiredFieldsRule,
        ));
        validator.add_rule(ValidationRuleEnum::TypeConstraints(
            super::rules::TypeConstraintsRule,
        ));
        validator.add_rule(ValidationRuleEnum::FormatValidation(
            super::rules::FormatValidationRule,
        ));
        validator.add_rule(ValidationRuleEnum::OpenGraphValidation(
            super::rules::OpenGraphValidationRule,
        ));
        validator.add_rule(ValidationRuleEnum::TwitterValidation(
            super::rules::TwitterValidationRule,
        ));

        Ok(validator)
    }

    /// Create validator with custom configuration
    pub fn with_config(spec: OpenAPI, config: ValidationRules) -> Result<Self> {
        let mut validator = Self {
            spec,
            rules: Vec::new(),
            config,
        };

        // Add default validation rules
        validator.add_rule(ValidationRuleEnum::SchemaCompliance(
            super::rules::SchemaComplianceRule,
        ));
        validator.add_rule(ValidationRuleEnum::RequiredFields(
            super::rules::RequiredFieldsRule,
        ));
        validator.add_rule(ValidationRuleEnum::TypeConstraints(
            super::rules::TypeConstraintsRule,
        ));
        validator.add_rule(ValidationRuleEnum::FormatValidation(
            super::rules::FormatValidationRule,
        ));
        validator.add_rule(ValidationRuleEnum::OpenGraphValidation(
            super::rules::OpenGraphValidationRule,
        ));
        validator.add_rule(ValidationRuleEnum::TwitterValidation(
            super::rules::TwitterValidationRule,
        ));

        Ok(validator)
    }

    /// Add custom validation rule
    pub fn add_rule(&mut self, rule: ValidationRuleEnum) {
        self.rules.push(rule);
    }

    /// Validate request against OpenAPI spec
    pub fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> ValidationResult {
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        };

        // Check if path exists in OpenAPI spec
        if !self.path_exists(method, path) {
            result.add_error(ValidationErrorDetail {
                code: "PATH_NOT_FOUND".to_string(),
                message: format!("Path '{}' not found in OpenAPI spec", path),
                field: Some("path".to_string()),
                suggestion: Some(
                    "Check the OpenAPI specification for available endpoints".to_string(),
                ),
            });
            result.is_valid = false;
            result.score = 0;
        }

        // Apply custom validation rules
        for rule in &self.rules {
            if let Err(error) = rule.validate_request(method, path, headers, body) {
                result.add_error(ValidationErrorDetail {
                    code: "CUSTOM_VALIDATION".to_string(),
                    message: error.to_string(),
                    field: None,
                    suggestion: Some(format!("Check the {} validation rule", rule.name())),
                });
                result.is_valid = false;
                result.score = result.score.saturating_sub(10);
            }
        }

        result
    }

    /// Validate response against OpenAPI spec
    pub fn validate_response(
        &self,
        method: &str,
        path: &str,
        status_code: u16,
        _headers: &HashMap<String, String>,
        _body: Option<&Value>,
    ) -> ValidationResult {
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        };

        // Check if path exists in OpenAPI spec
        if !self.path_exists(method, path) {
            return result; // Skip response validation for unknown endpoints
        }

        // Basic response validation
        if (400..500).contains(&status_code) {
            result.add_warning(ValidationWarningDetail {
                code: "CLIENT_ERROR".to_string(),
                message: format!("Client error response: {}", status_code),
                field: Some("status_code".to_string()),
                suggestion: Some("Check request parameters and format".to_string()),
            });
        }

        if status_code >= 500 {
            result.add_error(ValidationErrorDetail {
                code: "SERVER_ERROR".to_string(),
                message: format!("Server error response: {}", status_code),
                field: Some("status_code".to_string()),
                suggestion: Some("Check server implementation".to_string()),
            });
            result.is_valid = false;
            result.score = result.score.saturating_sub(30);
        }

        result
    }

    /// Check if a path exists in the OpenAPI spec
    fn path_exists(&self, _method: &str, path: &str) -> bool {
        // Simple path existence check
        for (spec_path, _path_item) in &self.spec.paths.paths {
            if self.path_matches(spec_path, path) {
                return true;
            }
        }
        false
    }

    /// Check if a path matches the OpenAPI path pattern
    fn path_matches(&self, spec_path: &str, request_path: &str) -> bool {
        // Simple path matching - in production, you'd want more sophisticated matching
        // that handles path parameters like /users/{id}
        if spec_path == request_path {
            return true;
        }

        // Handle path parameters
        let spec_segments: Vec<&str> = spec_path.split('/').collect();
        let request_segments: Vec<&str> = request_path.split('/').collect();

        if spec_segments.len() != request_segments.len() {
            return false;
        }

        for (spec_seg, req_seg) in spec_segments.iter().zip(request_segments.iter()) {
            if spec_seg.starts_with('{') && spec_seg.ends_with('}') {
                // This is a path parameter, match any value
                continue;
            }
            if spec_seg != req_seg {
                return false;
            }
        }

        true
    }

    /// Get the OpenAPI specification
    pub fn get_spec(&self) -> &OpenAPI {
        &self.spec
    }

    /// Get validation configuration
    pub fn get_config(&self) -> &ValidationRules {
        &self.config
    }

    /// Get list of validation rules
    pub fn get_rules(&self) -> Vec<&str> {
        self.rules.iter().map(|rule| rule.name()).collect()
    }
}

impl ValidationResult {
    /// Add an error to the validation result
    pub fn add_error(&mut self, error: ValidationErrorDetail) {
        self.errors.push(error);
        self.is_valid = false;
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: ValidationWarningDetail) {
        self.warnings.push(warning);
    }

    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get validation score
    pub fn get_score(&self) -> u8 {
        self.score
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

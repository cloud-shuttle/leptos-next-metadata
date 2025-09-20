//! API contract types and structures
//!
//! This module contains the type definitions for API contract validation,
//! including validation results, errors, warnings, and configuration.

use serde::{Deserialize, Serialize};

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

/// Validation error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Missing required header: {header}")]
    MissingHeader { header: String },

    #[error("Missing required field: {field}")]
    MissingRequiredField { field: String },

    #[error("Invalid format for field '{field}': expected {expected}, got {actual}")]
    InvalidFormat {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Invalid value for field '{field}': expected {expected}, got {actual}")]
    InvalidValue {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Field '{field}' is too long: max {max_length}, got {actual_length}")]
    FieldTooLong {
        field: String,
        max_length: usize,
        actual_length: usize,
    },

    #[error("Field '{field}' is too short: min {min_length}, got {actual_length}")]
    FieldTooShort {
        field: String,
        min_length: usize,
        actual_length: usize,
    },

    #[error("Path not found: {path}")]
    PathNotFound { path: String },

    #[error("Unresolved reference: {reference}")]
    UnresolvedReference { reference: String },

    #[error("Schema validation error: {message}")]
    SchemaValidation { message: String },

    #[error("Type mismatch for field '{field}': expected {expected}, got {actual}")]
    TypeMismatch {
        field: String,
        expected: String,
        actual: String,
    },
}

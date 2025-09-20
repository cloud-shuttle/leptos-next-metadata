//! Validation types and structures
//!
//! This module contains the type definitions for metadata validation,
//! including validation results, errors, warnings, and status codes.

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

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        }
    }

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

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation rule for a specific field
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Field name
    pub field: String,

    /// Minimum length requirement
    pub min_length: Option<usize>,

    /// Maximum length requirement
    pub max_length: Option<usize>,

    /// Whether the field is required
    pub required: bool,

    /// Regex pattern for validation
    pub pattern: Option<String>,
}

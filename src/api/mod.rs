//! API module for contract-first development
//!
//! This module provides API contract validation, OpenAPI schema support,
//! and contract-first development tools for the metadata API.

pub mod contracts;

// Re-export main types for convenience
pub use contracts::{
    create_default_validator, load_openapi_schema, ContractMiddleware, ContractValidator,
    ValidationErrorDetail, ValidationResult, ValidationRules, ValidationWarningDetail,
};

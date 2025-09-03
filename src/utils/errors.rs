//! Error handling utilities

use crate::{Error, Result};

/// Result type alias for utility operations
pub type UtilResult<T> = std::result::Result<T, UtilError>;

/// Utility-specific errors
#[derive(Debug, thiserror::Error)]
pub enum UtilError {
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("URL error: {0}")]
    UrlError(String),
}

impl From<UtilError> for Error {
    fn from(err: UtilError) -> Self {
        Error::ValidationError(err.to_string())
    }
}

/// Convert any error to our Error type
pub fn to_metadata_error<E: std::fmt::Display>(err: E, context: &str) -> Error {
    Error::ValidationError(format!("{}: {}", context, err))
}

/// Chain errors with context
pub fn with_context<T, E: std::fmt::Display>(
    result: std::result::Result<T, E>,
    context: &str,
) -> Result<T> {
    result.map_err(|e| to_metadata_error(e, context))
}
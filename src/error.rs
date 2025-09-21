//! Unified error handling for leptos-next-metadata
//!
//! Provides consistent error types and handling across native and WASM environments

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unified error type for leptos-next-metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataError {
    /// Error kind
    pub kind: ErrorKind,
    /// Error message
    pub message: String,
    /// Error context
    pub context: Option<String>,
    /// Error source (if applicable)
    pub source: Option<String>,
    /// Error timestamp
    pub timestamp: Option<String>,
    /// Additional error metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Error kinds for different types of failures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorKind {
    /// Validation errors
    Validation,
    /// Network/HTTP errors
    Network,
    /// File system errors
    FileSystem,
    /// Serialization/deserialization errors
    Serialization,
    /// Configuration errors
    Configuration,
    /// Security errors
    Security,
    /// Performance errors
    Performance,
    /// Browser/DOM errors (WASM only)
    Browser,
    /// Storage errors
    Storage,
    /// Image processing errors
    ImageProcessing,
    /// Template rendering errors
    Template,
    /// Unknown/unexpected errors
    Unknown,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    /// Low severity - non-critical issues
    Low,
    /// Medium severity - may affect functionality
    Medium,
    /// High severity - significant functionality impact
    High,
    /// Critical severity - application breaking
    Critical,
}

/// Error context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Component or module where error occurred
    pub component: Option<String>,
    /// Operation being performed
    pub operation: Option<String>,
    /// User agent or environment info
    pub environment: Option<String>,
    /// Request ID or session ID
    pub request_id: Option<String>,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
}

/// Error reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReportingConfig {
    /// Whether to enable error reporting
    pub enabled: bool,
    /// Maximum number of errors to report per session
    pub max_errors_per_session: usize,
    /// Error reporting endpoint
    pub endpoint: Option<String>,
    /// Whether to include stack traces
    pub include_stack_traces: bool,
    /// Whether to include user context
    pub include_user_context: bool,
    /// Error sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
}

impl MetadataError {
    /// Create a new error
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            context: None,
            source: None,
            timestamp: Some(Self::current_timestamp()),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a new error with context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Create a new error with source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Add metadata to the error
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get error severity based on kind
    pub fn severity(&self) -> ErrorSeverity {
        match self.kind {
            ErrorKind::Validation => ErrorSeverity::Medium,
            ErrorKind::Network => ErrorSeverity::High,
            ErrorKind::FileSystem => ErrorSeverity::High,
            ErrorKind::Serialization => ErrorSeverity::Medium,
            ErrorKind::Configuration => ErrorSeverity::High,
            ErrorKind::Security => ErrorSeverity::Critical,
            ErrorKind::Performance => ErrorSeverity::Low,
            ErrorKind::Browser => ErrorSeverity::Medium,
            ErrorKind::Storage => ErrorSeverity::Medium,
            ErrorKind::ImageProcessing => ErrorSeverity::Medium,
            ErrorKind::Template => ErrorSeverity::Medium,
            ErrorKind::Unknown => ErrorSeverity::High,
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::Validation
                | ErrorKind::Performance
                | ErrorKind::Browser
                | ErrorKind::Storage
        )
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self.kind {
            ErrorKind::Validation => "Please check your input and try again.".to_string(),
            ErrorKind::Network => {
                "Network connection issue. Please check your internet connection.".to_string()
            }
            ErrorKind::FileSystem => "File system error. Please try again.".to_string(),
            ErrorKind::Serialization => "Data processing error. Please try again.".to_string(),
            ErrorKind::Configuration => "Configuration error. Please contact support.".to_string(),
            ErrorKind::Security => "Security error detected. Please refresh the page.".to_string(),
            ErrorKind::Performance => {
                "Performance issue detected. The page may be slow.".to_string()
            }
            ErrorKind::Browser => {
                "Browser compatibility issue. Please try a different browser.".to_string()
            }
            ErrorKind::Storage => "Storage error. Your data may not be saved.".to_string(),
            ErrorKind::ImageProcessing => {
                "Image processing error. Please try a different image.".to_string()
            }
            ErrorKind::Template => "Template rendering error. Please try again.".to_string(),
            ErrorKind::Unknown => "An unexpected error occurred. Please try again.".to_string(),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Create from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get current timestamp
    fn current_timestamp() -> String {
        #[cfg(target_arch = "wasm32")]
        {
            // Use JavaScript Date for WASM
            if let Some(_window) = web_sys::window() {
                let date = js_sys::Date::new_0();
                date.to_iso_string()
                    .as_string()
                    .unwrap_or_else(|| "unknown".to_string())
            } else {
                "unknown".to_string()
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use std::time for native
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        }
    }
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)?;
        if let Some(context) = &self.context {
            write!(f, " (Context: {})", context)?;
        }
        if let Some(source) = &self.source {
            write!(f, " (Source: {})", source)?;
        }
        Ok(())
    }
}

impl std::error::Error for MetadataError {}

/// Result type alias for MetadataError
pub type MetadataResult<T> = Result<T, MetadataError>;

/// Error handler trait for different environments
pub trait ErrorHandler {
    /// Handle an error
    fn handle_error(&self, error: &MetadataError) -> Result<(), MetadataError>;

    /// Report an error
    fn report_error(&self, error: &MetadataError) -> Result<(), MetadataError>;

    /// Log an error
    fn log_error(&self, error: &MetadataError) -> Result<(), MetadataError>;
}

/// Console error handler for WASM
#[cfg(target_arch = "wasm32")]
pub struct ConsoleErrorHandler {
    config: ErrorReportingConfig,
}

#[cfg(target_arch = "wasm32")]
impl ConsoleErrorHandler {
    pub fn new(config: ErrorReportingConfig) -> Self {
        Self { config }
    }
}

#[cfg(target_arch = "wasm32")]
impl ErrorHandler for ConsoleErrorHandler {
    fn handle_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // Log to console
        web_sys::console::error_1(&format!("[ERROR] {}", error).into());

        // Report if enabled
        if self.config.enabled {
            self.report_error(error)?;
        }

        Ok(())
    }

    fn report_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // In a real implementation, this would send to an error reporting service
        web_sys::console::warn_1(&format!("[REPORT] {}", error).into());
        Ok(())
    }

    fn log_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        web_sys::console::log_1(&format!("[LOG] {}", error).into());
        Ok(())
    }
}

/// File error handler for native
#[cfg(not(target_arch = "wasm32"))]
pub struct FileErrorHandler {
    config: ErrorReportingConfig,
    log_file: Option<std::path::PathBuf>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileErrorHandler {
    pub fn new(config: ErrorReportingConfig, log_file: Option<std::path::PathBuf>) -> Self {
        Self { config, log_file }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ErrorHandler for FileErrorHandler {
    fn handle_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // Log to stderr
        eprintln!("[ERROR] {}", error);

        // Log to file if configured
        if let Some(log_file) = &self.log_file {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
            {
                use std::io::Write;
                let _ = writeln!(
                    file,
                    "[{}] [ERROR] {}",
                    chrono::Utc::now().to_rfc3339(),
                    error
                );
            }
        }

        // Report if enabled
        if self.config.enabled {
            self.report_error(error)?;
        }

        Ok(())
    }

    fn report_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // In a real implementation, this would send to an error reporting service
        eprintln!("[REPORT] {}", error);
        Ok(())
    }

    fn log_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        println!("[LOG] {}", error);
        Ok(())
    }
}

/// Error context builder
pub struct ErrorContextBuilder {
    context: ErrorContext,
}

impl ErrorContextBuilder {
    pub fn new() -> Self {
        Self {
            context: ErrorContext {
                component: None,
                operation: None,
                environment: None,
                request_id: None,
                stack_trace: None,
            },
        }
    }

    pub fn component(mut self, component: impl Into<String>) -> Self {
        self.context.component = Some(component.into());
        self
    }

    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.context.operation = Some(operation.into());
        self
    }

    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.context.environment = Some(environment.into());
        self
    }

    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.context.request_id = Some(request_id.into());
        self
    }

    pub fn stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.context.stack_trace = Some(stack_trace.into());
        self
    }

    pub fn build(self) -> ErrorContext {
        self.context
    }
}

/// Error utilities
pub struct ErrorUtils;

impl ErrorUtils {
    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::Validation, message)
    }

    /// Create a network error
    pub fn network_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::Network, message)
    }

    /// Create a security error
    pub fn security_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::Security, message)
    }

    /// Create a browser error (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn browser_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::Browser, message)
    }

    /// Create a storage error
    pub fn storage_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::Storage, message)
    }

    /// Create an image processing error
    pub fn image_processing_error(message: impl Into<String>) -> MetadataError {
        MetadataError::new(ErrorKind::ImageProcessing, message)
    }

    /// Wrap a standard error
    pub fn wrap_error(
        error: impl std::error::Error + Send + Sync + 'static,
        kind: ErrorKind,
    ) -> MetadataError {
        MetadataError::new(kind, error.to_string())
    }

    /// Get error statistics
    pub fn get_error_stats(errors: &[MetadataError]) -> ErrorStats {
        let mut stats = ErrorStats::default();

        for error in errors {
            stats.total_errors += 1;

            match error.kind {
                ErrorKind::Validation => stats.validation_errors += 1,
                ErrorKind::Network => stats.network_errors += 1,
                ErrorKind::Security => stats.security_errors += 1,
                ErrorKind::Browser => stats.browser_errors += 1,
                ErrorKind::Storage => stats.storage_errors += 1,
                ErrorKind::ImageProcessing => stats.image_processing_errors += 1,
                _ => stats.other_errors += 1,
            }

            match error.severity() {
                ErrorSeverity::Low => stats.low_severity += 1,
                ErrorSeverity::Medium => stats.medium_severity += 1,
                ErrorSeverity::High => stats.high_severity += 1,
                ErrorSeverity::Critical => stats.critical_severity += 1,
            }
        }

        stats
    }
}

/// Error statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ErrorStats {
    pub total_errors: usize,
    pub validation_errors: usize,
    pub network_errors: usize,
    pub security_errors: usize,
    pub browser_errors: usize,
    pub storage_errors: usize,
    pub image_processing_errors: usize,
    pub other_errors: usize,
    pub low_severity: usize,
    pub medium_severity: usize,
    pub high_severity: usize,
    pub critical_severity: usize,
}

impl ErrorStats {
    /// Get error rate by kind
    pub fn error_rate_by_kind(&self) -> std::collections::HashMap<String, f64> {
        let mut rates = std::collections::HashMap::new();
        if self.total_errors > 0 {
            rates.insert(
                "validation".to_string(),
                self.validation_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "network".to_string(),
                self.network_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "security".to_string(),
                self.security_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "browser".to_string(),
                self.browser_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "storage".to_string(),
                self.storage_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "image_processing".to_string(),
                self.image_processing_errors as f64 / self.total_errors as f64,
            );
            rates.insert(
                "other".to_string(),
                self.other_errors as f64 / self.total_errors as f64,
            );
        }
        rates
    }

    /// Get severity distribution
    pub fn severity_distribution(&self) -> std::collections::HashMap<String, f64> {
        let mut distribution = std::collections::HashMap::new();
        if self.total_errors > 0 {
            distribution.insert(
                "low".to_string(),
                self.low_severity as f64 / self.total_errors as f64,
            );
            distribution.insert(
                "medium".to_string(),
                self.medium_severity as f64 / self.total_errors as f64,
            );
            distribution.insert(
                "high".to_string(),
                self.high_severity as f64 / self.total_errors as f64,
            );
            distribution.insert(
                "critical".to_string(),
                self.critical_severity as f64 / self.total_errors as f64,
            );
        }
        distribution
    }
}

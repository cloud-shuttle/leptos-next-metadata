//! WASM-specific error handling
//!
//! Provides error handling utilities specifically designed for WASM environments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

use crate::error::{
    ErrorContext, ErrorContextBuilder, ErrorHandler, ErrorKind, ErrorReportingConfig,
    ErrorSeverity, ErrorStats, ErrorUtils, MetadataError,
};

/// WASM-specific error handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmErrorHandler {
    /// Error reporting configuration
    config: ErrorReportingConfig,
    /// Error history for this session
    error_history: Vec<MetadataError>,
    /// Maximum errors to keep in history
    max_history_size: usize,
    /// Error reporting endpoint
    reporting_endpoint: Option<String>,
}

/// WASM error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmErrorContext {
    /// Browser information
    pub user_agent: Option<String>,
    /// Current URL
    pub current_url: Option<String>,
    /// Referrer
    pub referrer: Option<String>,
    /// Screen resolution
    pub screen_resolution: Option<(u32, u32)>,
    /// Timezone
    pub timezone: Option<String>,
    /// Language
    pub language: Option<String>,
    /// Platform
    pub platform: Option<String>,
    /// Online status
    pub is_online: bool,
    /// Cookie enabled
    pub cookies_enabled: bool,
    /// Local storage available
    pub local_storage_available: bool,
    /// Session storage available
    pub session_storage_available: bool,
}

/// Error reporting service for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmErrorReporter {
    /// Reporting endpoint
    endpoint: String,
    /// API key for authentication
    api_key: Option<String>,
    /// Batch size for error reporting
    batch_size: usize,
    /// Error queue
    error_queue: Vec<MetadataError>,
    /// Maximum queue size
    max_queue_size: usize,
}

impl WasmErrorHandler {
    /// Create a new WASM error handler
    pub fn new(config: ErrorReportingConfig) -> Self {
        Self {
            config,
            error_history: Vec::new(),
            max_history_size: 100,
            reporting_endpoint: None,
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        config: ErrorReportingConfig,
        max_history_size: usize,
        reporting_endpoint: Option<String>,
    ) -> Self {
        Self {
            config,
            error_history: Vec::new(),
            max_history_size,
            reporting_endpoint,
        }
    }

    /// Get WASM-specific error context
    pub fn get_wasm_context(&self) -> Result<WasmErrorContext, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window available"))?;
        let navigator = window.navigator();
        let location = window.location();
        let screen = window.screen().ok();

        Ok(WasmErrorContext {
            user_agent: navigator.user_agent().ok(),
            current_url: location.href().ok(),
            referrer: None, // referrer() method not available in web-sys
            screen_resolution: screen.map(|s| {
                (
                    s.width().unwrap_or(0) as u32,
                    s.height().unwrap_or(0) as u32,
                )
            }),
            timezone: None, // Would need additional JS to get timezone
            language: navigator.language(),
            platform: navigator.platform().ok(),
            is_online: navigator.on_line(),
            cookies_enabled: false, // cookie_enabled() method not available in web-sys
            local_storage_available: self.check_local_storage_availability(),
            session_storage_available: self.check_session_storage_availability(),
        })
    }

    /// Check if local storage is available
    fn check_local_storage_availability(&self) -> bool {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(_)) = window.local_storage() {
                return true;
            }
        }
        false
    }

    /// Check if session storage is available
    fn check_session_storage_availability(&self) -> bool {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(_)) = window.session_storage() {
                return true;
            }
        }
        false
    }

    /// Add error to history
    pub fn add_to_history(&mut self, error: MetadataError) {
        self.error_history.push(error);

        // Trim history if it exceeds max size
        if self.error_history.len() > self.max_history_size {
            self.error_history.remove(0);
        }
    }

    /// Get error history
    pub fn get_error_history(&self) -> &[MetadataError] {
        &self.error_history
    }

    /// Clear error history
    pub fn clear_history(&mut self) {
        self.error_history.clear();
    }

    /// Get error statistics
    pub fn get_error_stats(&self) -> ErrorStats {
        ErrorUtils::get_error_stats(&self.error_history)
    }

    /// Get recent errors
    pub fn get_recent_errors(&self, count: usize) -> Vec<MetadataError> {
        self.error_history
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Get errors by severity
    pub fn get_errors_by_severity(&self, severity: ErrorSeverity) -> Vec<MetadataError> {
        self.error_history
            .iter()
            .filter(|error| error.severity() == severity)
            .cloned()
            .collect()
    }

    /// Get errors by kind
    pub fn get_errors_by_kind(&self, kind: ErrorKind) -> Vec<MetadataError> {
        self.error_history
            .iter()
            .filter(|error| error.kind == kind)
            .cloned()
            .collect()
    }

    /// Export error history as JSON
    pub fn export_history(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.error_history)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    /// Import error history from JSON
    pub fn import_history(&mut self, json: &str) -> Result<(), JsValue> {
        let errors: Vec<MetadataError> = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("JSON deserialization error: {}", e)))?;

        self.error_history = errors;
        Ok(())
    }
}

impl ErrorHandler for WasmErrorHandler {
    fn handle_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // Log to console
        web_sys::console::error_1(&format!("[WASM ERROR] {}", error).into());

        // Log to browser console with additional context
        if let Ok(context) = self.get_wasm_context() {
            web_sys::console::log_1(&format!("[WASM CONTEXT] {:?}", context).into());
        }

        // Report if enabled
        if self.config.enabled {
            self.report_error(error)?;
        }

        Ok(())
    }

    fn report_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        // In a real implementation, this would send to an error reporting service
        web_sys::console::warn_1(&format!("[WASM REPORT] {}", error).into());

        // If we have a reporting endpoint, we could send the error there
        if let Some(endpoint) = &self.reporting_endpoint {
            web_sys::console::log_1(&format!("[WASM ENDPOINT] Would send to: {}", endpoint).into());
        }

        Ok(())
    }

    fn log_error(&self, error: &MetadataError) -> Result<(), MetadataError> {
        web_sys::console::log_1(&format!("[WASM LOG] {}", error).into());
        Ok(())
    }
}

impl WasmErrorReporter {
    /// Create a new WASM error reporter
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            api_key: None,
            batch_size: 10,
            error_queue: Vec::new(),
            max_queue_size: 100,
        }
    }

    /// Create with API key
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Create with custom batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Queue an error for reporting
    pub fn queue_error(&mut self, error: MetadataError) -> Result<(), JsValue> {
        if self.error_queue.len() >= self.max_queue_size {
            // Remove oldest error
            self.error_queue.remove(0);
        }

        self.error_queue.push(error);

        // Auto-report if batch size is reached
        if self.error_queue.len() >= self.batch_size {
            self.flush_queue()?;
        }

        Ok(())
    }

    /// Flush the error queue
    pub fn flush_queue(&mut self) -> Result<(), JsValue> {
        if self.error_queue.is_empty() {
            return Ok(());
        }

        // In a real implementation, this would send the errors to the reporting service
        web_sys::console::log_1(
            &format!(
                "[WASM REPORTER] Flushing {} errors to {}",
                self.error_queue.len(),
                self.endpoint
            )
            .into(),
        );

        // Clear the queue after reporting
        self.error_queue.clear();

        Ok(())
    }

    /// Get queue status
    pub fn get_queue_status(&self) -> QueueStatus {
        QueueStatus {
            queue_size: self.error_queue.len(),
            max_queue_size: self.max_queue_size,
            batch_size: self.batch_size,
            endpoint: self.endpoint.clone(),
            has_api_key: self.api_key.is_some(),
        }
    }
}

/// Queue status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub queue_size: usize,
    pub max_queue_size: usize,
    pub batch_size: usize,
    pub endpoint: String,
    pub has_api_key: bool,
}

/// WASM error utilities
pub struct WasmErrorUtils;

impl WasmErrorUtils {
    /// Create a browser-specific error
    pub fn browser_error(message: impl Into<String>) -> MetadataError {
        ErrorUtils::browser_error(message)
    }

    /// Create a storage-specific error
    pub fn storage_error(message: impl Into<String>) -> MetadataError {
        ErrorUtils::storage_error(message)
    }

    /// Create a validation error with WASM context
    pub fn validation_error_with_context(
        message: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> MetadataError {
        ErrorUtils::validation_error(message).with_context(format!(
            "Field: {}, Value: {}",
            field.into(),
            value.into()
        ))
    }

    /// Create a network error with WASM context
    pub fn network_error_with_context(
        message: impl Into<String>,
        url: impl Into<String>,
        status: Option<u16>,
    ) -> MetadataError {
        let mut error = ErrorUtils::network_error(message);
        error = error.with_context(format!("URL: {}", url.into()));
        if let Some(status) = status {
            error = error.with_metadata("status_code", status.to_string());
        }
        error
    }

    /// Get browser error context
    pub fn get_browser_context() -> Result<WasmErrorContext, JsValue> {
        let handler = WasmErrorHandler::new(ErrorReportingConfig {
            enabled: false,
            max_errors_per_session: 0,
            endpoint: None,
            include_stack_traces: false,
            include_user_context: false,
            sampling_rate: 0.0,
        });
        handler.get_wasm_context()
    }

    /// Check if error is recoverable in WASM context
    pub fn is_wasm_recoverable(error: &MetadataError) -> bool {
        match error.kind {
            ErrorKind::Browser => true,     // Browser errors are often recoverable
            ErrorKind::Storage => true,     // Storage errors can be handled gracefully
            ErrorKind::Validation => true,  // Validation errors are always recoverable
            ErrorKind::Performance => true, // Performance issues are recoverable
            _ => error.is_recoverable(),
        }
    }

    /// Get user-friendly error message for WASM
    pub fn get_wasm_user_message(error: &MetadataError) -> String {
        match error.kind {
            ErrorKind::Browser => "Browser compatibility issue. Please try refreshing the page or using a different browser.".to_string(),
            ErrorKind::Storage => "Storage issue. Your data may not be saved. Please try again.".to_string(),
            ErrorKind::Validation => "Please check your input and try again.".to_string(),
            ErrorKind::Network => "Network connection issue. Please check your internet connection and try again.".to_string(),
            _ => error.user_message(),
        }
    }
}

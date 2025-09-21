//! Analytics Event Handlers
//!
//! Provides various handlers for processing analytics events including
//! console logging, local storage, and remote analytics services.

use crate::analytics::{AnalyticsEvent, AnalyticsEventHandler};
use crate::error::{ErrorKind, MetadataError};
use serde::{Deserialize, Serialize};

/// Console analytics handler for development and debugging
pub struct ConsoleAnalyticsHandler {
    /// Handler name
    name: String,
    /// Log level
    log_level: LogLevel,
}

/// Log levels for console output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl ConsoleAnalyticsHandler {
    /// Create a new console analytics handler
    pub fn new(name: String, log_level: LogLevel) -> Self {
        Self { name, log_level }
    }

    /// Create a default console handler
    pub fn default() -> Self {
        Self::new("console".to_string(), LogLevel::Info)
    }
}

impl AnalyticsEventHandler for ConsoleAnalyticsHandler {
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError> {
        let log_message = format!(
            "[Analytics] {} - {} at {}",
            self.name, event.event_type, event.timestamp
        );

        match self.log_level {
            LogLevel::Debug => println!("DEBUG: {}", log_message),
            LogLevel::Info => println!("INFO: {}", log_message),
            LogLevel::Warn => println!("WARN: {}", log_message),
            LogLevel::Error => eprintln!("ERROR: {}", log_message),
        }

        // Log additional properties if available
        if !event.properties.is_empty() {
            println!("  Properties: {:?}", event.properties);
        }

        if let Some(duration) = event.duration_ms {
            println!("  Duration: {}ms", duration);
        }

        if let Some(error) = &event.error {
            println!("  Error: {} - {}", error.kind, error.message);
        }

        Ok(())
    }

    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        println!(
            "[Analytics] {} - Processing batch of {} events",
            self.name,
            events.len()
        );

        for event in events {
            self.handle_event(event)?;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Local storage analytics handler for browser environments
#[cfg(target_arch = "wasm32")]
pub struct LocalStorageAnalyticsHandler {
    /// Handler name
    name: String,
    /// Storage key prefix
    storage_prefix: String,
    /// Maximum events to store
    max_events: usize,
}

#[cfg(target_arch = "wasm32")]
impl LocalStorageAnalyticsHandler {
    /// Create a new local storage analytics handler
    pub fn new(name: String, storage_prefix: String, max_events: usize) -> Self {
        Self {
            name,
            storage_prefix,
            max_events,
        }
    }

    /// Create a default local storage handler
    pub fn default() -> Self {
        Self::new(
            "local_storage".to_string(),
            "leptos_analytics".to_string(),
            1000,
        )
    }

    /// Get events from local storage
    fn get_stored_events(&self) -> Result<Vec<AnalyticsEvent>, MetadataError> {
        use web_sys::window;

        let window = window().ok_or_else(|| {
            MetadataError::new(ErrorKind::Browser, "Window not available".to_string())
        })?;

        let storage = window
            .local_storage()
            .map_err(|_| {
                MetadataError::new(
                    ErrorKind::Browser,
                    "Local storage not available".to_string(),
                )
            })?
            .ok_or_else(|| {
                MetadataError::new(
                    ErrorKind::Browser,
                    "Local storage not available".to_string(),
                )
            })?;

        let key = format!("{}_events", self.storage_prefix);
        let stored_data = storage.get_item(&key).map_err(|_| {
            MetadataError::new(
                ErrorKind::Browser,
                "Failed to read from local storage".to_string(),
            )
        })?;

        if let Some(data) = stored_data {
            let events: Vec<AnalyticsEvent> = serde_json::from_str(&data)
                .map_err(|e| MetadataError::new(ErrorKind::Serialization, e.to_string()))?;
            Ok(events)
        } else {
            Ok(Vec::new())
        }
    }

    /// Store events in local storage
    fn store_events(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        use web_sys::window;

        let window = window().ok_or_else(|| {
            MetadataError::new(ErrorKind::Browser, "Window not available".to_string())
        })?;

        let storage = window.local_storage().map_err(|_| {
            MetadataError::new(
                ErrorKind::Browser,
                "Local storage not available".to_string(),
            )
        })?;

        if let Some(storage) = storage {
            let key = format!("{}_events", self.storage_prefix);
            let data = serde_json::to_string(events)
                .map_err(|e| MetadataError::new(ErrorKind::Serialization, e.to_string()))?;

            storage.set_item(&key, &data).map_err(|_| {
                MetadataError::new(
                    ErrorKind::Browser,
                    "Failed to write to local storage".to_string(),
                )
            })?;
        }

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl AnalyticsEventHandler for LocalStorageAnalyticsHandler {
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError> {
        let mut stored_events = self.get_stored_events().unwrap_or_default();
        stored_events.push(event.clone());

        // Limit the number of stored events
        if stored_events.len() > self.max_events {
            let excess = stored_events.len() - self.max_events;
            stored_events.drain(0..excess);
        }

        self.store_events(&stored_events)?;
        Ok(())
    }

    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        let mut stored_events = self.get_stored_events().unwrap_or_default();
        stored_events.extend(events.iter().cloned());

        // Limit the number of stored events
        if stored_events.len() > self.max_events {
            let excess = stored_events.len() - self.max_events;
            stored_events.drain(0..excess);
        }

        self.store_events(&stored_events)?;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Remote analytics handler for sending events to external services
pub struct RemoteAnalyticsHandler {
    /// Handler name
    name: String,
    /// Endpoint URL
    endpoint_url: String,
    /// API key
    api_key: Option<String>,
    /// Request timeout in seconds
    timeout_seconds: u64,
    /// Batch size for remote requests
    batch_size: usize,
}

impl RemoteAnalyticsHandler {
    /// Create a new remote analytics handler
    pub fn new(
        name: String,
        endpoint_url: String,
        api_key: Option<String>,
        timeout_seconds: u64,
        batch_size: usize,
    ) -> Self {
        Self {
            name,
            endpoint_url,
            api_key,
            timeout_seconds,
            batch_size,
        }
    }

    /// Create a handler for Google Analytics 4
    pub fn google_analytics_4(measurement_id: String, api_secret: Option<String>) -> Self {
        let endpoint = format!(
            "https://www.google-analytics.com/mp/collect?measurement_id={}",
            measurement_id
        );
        Self::new(
            "google_analytics_4".to_string(),
            endpoint,
            api_secret,
            10,
            20,
        )
    }

    /// Create a handler for custom analytics endpoint
    pub fn custom_endpoint(name: String, endpoint_url: String, api_key: Option<String>) -> Self {
        Self::new(name, endpoint_url, api_key, 10, 10)
    }
}

impl AnalyticsEventHandler for RemoteAnalyticsHandler {
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError> {
        // For single events, we'll batch them with a batch of 1
        self.handle_batch(&[event.clone()])
    }

    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        // In a real implementation, this would make HTTP requests
        // For now, we'll simulate the behavior

        if events.is_empty() {
            return Ok(());
        }

        // Simulate network request
        println!(
            "[Analytics] {} - Sending {} events to {}",
            self.name,
            events.len(),
            self.endpoint_url
        );

        // In a real implementation, you would:
        // 1. Serialize events to JSON
        // 2. Create HTTP request with proper headers
        // 3. Send request with timeout
        // 4. Handle response and errors

        // For now, just log the events
        for event in events {
            println!("  Event: {} at {}", event.event_type, event.timestamp);
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// File-based analytics handler for native environments
#[cfg(not(target_arch = "wasm32"))]
pub struct FileAnalyticsHandler {
    /// Handler name
    name: String,
    /// File path
    file_path: String,
    /// Maximum file size in bytes
    max_file_size: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileAnalyticsHandler {
    /// Create a new file analytics handler
    pub fn new(name: String, file_path: String, max_file_size: usize) -> Self {
        Self {
            name,
            file_path,
            max_file_size,
        }
    }

    /// Create a default file handler
    pub fn default() -> Self {
        Self::new(
            "file".to_string(),
            "analytics_events.jsonl".to_string(),
            10 * 1024 * 1024, // 10MB
        )
    }

    /// Append events to file
    fn append_events(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| MetadataError::new(ErrorKind::FileSystem, e.to_string()))?;

        for event in events {
            let json_line = serde_json::to_string(event)
                .map_err(|e| MetadataError::new(ErrorKind::Serialization, e.to_string()))?;
            writeln!(file, "{}", json_line)
                .map_err(|e| MetadataError::new(ErrorKind::FileSystem, e.to_string()))?;
        }

        Ok(())
    }

    /// Check if file needs rotation
    fn needs_rotation(&self) -> Result<bool, MetadataError> {
        use std::fs::metadata;

        match metadata(&self.file_path) {
            Ok(metadata) => Ok(metadata.len() as usize > self.max_file_size),
            Err(_) => Ok(false), // File doesn't exist, no rotation needed
        }
    }

    /// Rotate the log file
    fn rotate_file(&self) -> Result<(), MetadataError> {
        use std::fs;

        if self.needs_rotation()? {
            let backup_path = format!("{}.backup", self.file_path);
            fs::rename(&self.file_path, &backup_path)
                .map_err(|e| MetadataError::new(ErrorKind::FileSystem, e.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AnalyticsEventHandler for FileAnalyticsHandler {
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError> {
        self.handle_batch(&[event.clone()])
    }

    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        // Check if file rotation is needed
        self.rotate_file()?;

        // Append events to file
        self.append_events(events)?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Memory-based analytics handler for temporary storage
pub struct MemoryAnalyticsHandler {
    /// Handler name
    name: String,
    /// Stored events
    events: std::sync::Mutex<Vec<AnalyticsEvent>>,
    /// Maximum events to store
    max_events: usize,
}

impl MemoryAnalyticsHandler {
    /// Create a new memory analytics handler
    pub fn new(name: String, max_events: usize) -> Self {
        Self {
            name,
            events: std::sync::Mutex::new(Vec::new()),
            max_events,
        }
    }

    /// Create a default memory handler
    pub fn default() -> Self {
        Self::new("memory".to_string(), 1000)
    }

    /// Get all stored events
    pub fn get_events(&self) -> Result<Vec<AnalyticsEvent>, MetadataError> {
        let events = self.events.lock().map_err(|_| {
            MetadataError::new(ErrorKind::Unknown, "Failed to acquire lock".to_string())
        })?;
        Ok(events.clone())
    }

    /// Clear all stored events
    pub fn clear_events(&self) -> Result<(), MetadataError> {
        let mut events = self.events.lock().map_err(|_| {
            MetadataError::new(ErrorKind::Unknown, "Failed to acquire lock".to_string())
        })?;
        events.clear();
        Ok(())
    }

    /// Get event count
    pub fn event_count(&self) -> Result<usize, MetadataError> {
        let events = self.events.lock().map_err(|_| {
            MetadataError::new(ErrorKind::Unknown, "Failed to acquire lock".to_string())
        })?;
        Ok(events.len())
    }
}

impl AnalyticsEventHandler for MemoryAnalyticsHandler {
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError> {
        self.handle_batch(&[event.clone()])
    }

    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError> {
        let mut stored_events = self.events.lock().map_err(|_| {
            MetadataError::new(ErrorKind::Unknown, "Failed to acquire lock".to_string())
        })?;

        stored_events.extend(events.iter().cloned());

        // Limit the number of stored events
        if stored_events.len() > self.max_events {
            let excess = stored_events.len() - self.max_events;
            stored_events.drain(0..excess);
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Analytics handler factory for creating handlers
pub struct AnalyticsHandlerFactory;

impl AnalyticsHandlerFactory {
    /// Create a console handler
    pub fn console(
        name: Option<String>,
        log_level: Option<LogLevel>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(ConsoleAnalyticsHandler::new(
            name.unwrap_or_else(|| "console".to_string()),
            log_level.unwrap_or(LogLevel::Info),
        ))
    }

    /// Create a memory handler
    pub fn memory(
        name: Option<String>,
        max_events: Option<usize>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(MemoryAnalyticsHandler::new(
            name.unwrap_or_else(|| "memory".to_string()),
            max_events.unwrap_or(1000),
        ))
    }

    /// Create a local storage handler (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn local_storage(
        name: Option<String>,
        storage_prefix: Option<String>,
        max_events: Option<usize>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(LocalStorageAnalyticsHandler::new(
            name.unwrap_or_else(|| "local_storage".to_string()),
            storage_prefix.unwrap_or_else(|| "leptos_analytics".to_string()),
            max_events.unwrap_or(1000),
        ))
    }

    /// Create a file handler (native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(
        name: Option<String>,
        file_path: Option<String>,
        max_file_size: Option<usize>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(FileAnalyticsHandler::new(
            name.unwrap_or_else(|| "file".to_string()),
            file_path.unwrap_or_else(|| "analytics_events.jsonl".to_string()),
            max_file_size.unwrap_or(10 * 1024 * 1024),
        ))
    }

    /// Create a remote handler
    pub fn remote(
        name: String,
        endpoint_url: String,
        api_key: Option<String>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(RemoteAnalyticsHandler::new(
            name,
            endpoint_url,
            api_key,
            10,
            10,
        ))
    }

    /// Create a Google Analytics 4 handler
    pub fn google_analytics_4(
        measurement_id: String,
        api_secret: Option<String>,
    ) -> Box<dyn AnalyticsEventHandler> {
        Box::new(RemoteAnalyticsHandler::google_analytics_4(
            measurement_id,
            api_secret,
        ))
    }
}

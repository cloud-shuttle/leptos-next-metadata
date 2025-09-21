//! WASM-specific Analytics Integration
//!
//! Provides browser-specific analytics functionality including
//! performance monitoring, user interaction tracking, and
//! integration with browser APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, Performance, PerformanceEntry, PerformanceObserver, PerformanceObserverInit,
};

use crate::analytics::{
    AnalyticsConfig, AnalyticsEvent, AnalyticsEventType, AnalyticsManager, AnalyticsSession,
    PerformanceMetrics, PrivacySettings,
};
use crate::error::{ErrorKind, MetadataError};

/// WASM-specific analytics context
#[derive(Debug, Clone)]
pub struct WasmAnalyticsContext {
    /// Analytics manager
    manager: AnalyticsManager,
    /// Performance observer for monitoring
    performance_observer: Option<PerformanceObserver>,
    /// User agent
    user_agent: Option<String>,
    /// Page URL
    page_url: Option<String>,
    /// Session start time
    session_start_time: Option<f64>,
}

/// Browser performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserPerformanceInfo {
    /// Navigation timing
    pub navigation_timing: Option<NavigationTiming>,
    /// Resource timing
    pub resource_timing: Vec<ResourceTiming>,
    /// Memory information
    pub memory_info: Option<MemoryInfo>,
    /// Connection information
    pub connection_info: Option<ConnectionInfo>,
}

/// Navigation timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationTiming {
    /// Page load time
    pub load_time: f64,
    /// DOM content loaded time
    pub dom_content_loaded_time: f64,
    /// First paint time
    pub first_paint_time: Option<f64>,
    /// First contentful paint time
    pub first_contentful_paint_time: Option<f64>,
    /// Largest contentful paint time
    pub largest_contentful_paint_time: Option<f64>,
    /// First input delay
    pub first_input_delay: Option<f64>,
    /// Cumulative layout shift
    pub cumulative_layout_shift: Option<f64>,
}

/// Resource timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTiming {
    /// Resource name
    pub name: String,
    /// Resource type
    pub resource_type: String,
    /// Load time
    pub load_time: f64,
    /// Transfer size
    pub transfer_size: u32,
    /// Compressed size
    pub compressed_size: Option<u32>,
}

/// Memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Used JS heap size
    pub used_js_heap_size: u32,
    /// Total JS heap size
    pub total_js_heap_size: u32,
    /// JS heap size limit
    pub js_heap_size_limit: u32,
}

/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Connection type
    pub connection_type: String,
    /// Effective type
    pub effective_type: String,
    /// Downlink speed
    pub downlink: f64,
    /// Round trip time
    pub rtt: u32,
    /// Save data mode
    pub save_data: bool,
}

impl WasmAnalyticsContext {
    /// Create a new WASM analytics context
    pub fn new(config: AnalyticsConfig) -> Result<Self, MetadataError> {
        let mut context = Self {
            manager: AnalyticsManager::new(config),
            performance_observer: None,
            user_agent: None,
            page_url: None,
            session_start_time: None,
        };

        // Initialize browser-specific features
        context.initialize_browser_features()?;
        Ok(context)
    }

    /// Create a default WASM analytics context
    pub fn default() -> Result<Self, MetadataError> {
        let config = AnalyticsConfig {
            enabled: true,
            batch_size: 10,
            flush_interval_seconds: 30,
            max_local_events: 1000,
            track_performance: true,
            track_errors: true,
            track_interactions: true,
            custom_event_types: vec![],
            privacy: PrivacySettings {
                anonymize_ip: true,
                hash_identifiers: true,
                collect_user_agent: true,
                collect_page_urls: true,
                retention_days: 90,
            },
        };

        Self::new(config)
    }

    /// Initialize browser-specific features
    fn initialize_browser_features(&mut self) -> Result<(), MetadataError> {
        // Get user agent
        if let Some(window) = window() {
            if let Some(navigator) = window.navigator() {
                self.user_agent = Some(navigator.user_agent().unwrap_or_default());
            }

            // Get page URL
            if let Some(location) = window.location() {
                self.page_url = location.href().ok();
            }

            // Initialize performance monitoring
            if self.manager.get_config().track_performance {
                self.initialize_performance_monitoring()?;
            }

            // Initialize session
            self.start_session()?;
        }

        Ok(())
    }

    /// Start analytics session
    fn start_session(&mut self) -> Result<(), MetadataError> {
        let session_id = self.generate_session_id();
        self.manager
            .start_session(session_id, self.user_agent.clone())?;

        // Record session start time
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                self.session_start_time = Some(performance.now());
            }
        }

        Ok(())
    }

    /// Generate a unique session ID
    fn generate_session_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // In a real implementation, you might want to use a more sophisticated ID generation
        format!("session_{}", timestamp)
    }

    /// Initialize performance monitoring
    fn initialize_performance_monitoring(&mut self) -> Result<(), MetadataError> {
        let window = window().ok_or_else(|| {
            MetadataError::new(ErrorKind::Browser, "Window not available".to_string())
        })?;

        let performance = window.performance().ok_or_else(|| {
            MetadataError::new(
                ErrorKind::Browser,
                "Performance API not available".to_string(),
            )
        })?;

        // Set up performance observer for navigation timing
        if let Ok(observer) = PerformanceObserver::new(&Closure::wrap(Box::new({
            let manager = self.manager.clone();
            move |entries: &js_sys::Array| {
                for i in 0..entries.length() {
                    if let Some(entry) = entries.get(i).dyn_into::<PerformanceEntry>().ok() {
                        let _ = Self::handle_performance_entry(&manager, &entry);
                    }
                }
            }
        })
            as Box<dyn FnMut(&js_sys::Array)>))
        {
            let mut init = PerformanceObserverInit::new();
            init.entry_types(&js_sys::Array::of1(&JsValue::from_str("navigation")));
            init.entry_types(&js_sys::Array::of1(&JsValue::from_str("resource")));
            init.entry_types(&js_sys::Array::of1(&JsValue::from_str("measure")));

            if let Ok(_) = observer.observe_with_options(&init) {
                self.performance_observer = Some(observer);
            }
        }

        Ok(())
    }

    /// Handle performance entry
    fn handle_performance_entry(
        manager: &AnalyticsManager,
        entry: &PerformanceEntry,
    ) -> Result<(), MetadataError> {
        let entry_type = entry.entry_type();
        let name = entry.name();
        let start_time = entry.start_time();
        let duration = entry.duration();

        let mut properties = HashMap::new();
        properties.insert(
            "entry_type".to_string(),
            serde_json::Value::String(entry_type),
        );
        properties.insert("name".to_string(), serde_json::Value::String(name));
        properties.insert(
            "start_time".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(start_time).unwrap_or_default()),
        );

        // Track performance event
        let mut manager_mut = manager.clone();
        manager_mut.track_performance(
            &format!("{}:{}", entry_type, name),
            duration as u64,
            true, // Assume success for now
            None, // Memory usage not available from basic PerformanceEntry
        )?;

        Ok(())
    }

    /// Track page view
    pub fn track_page_view(
        &mut self,
        page_id: String,
        page_title: Option<String>,
    ) -> Result<(), MetadataError> {
        let mut properties = HashMap::new();
        properties.insert(
            "page_id".to_string(),
            serde_json::Value::String(page_id.clone()),
        );

        if let Some(title) = page_title {
            properties.insert("page_title".to_string(), serde_json::Value::String(title));
        }

        if let Some(url) = &self.page_url {
            properties.insert(
                "page_url".to_string(),
                serde_json::Value::String(url.clone()),
            );
        }

        self.manager
            .track_event(AnalyticsEventType::UserInteraction, properties, None)?;

        Ok(())
    }

    /// Track user interaction
    pub fn track_user_interaction(
        &mut self,
        interaction_type: &str,
        element_id: Option<String>,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        let mut event_properties = properties;
        event_properties.insert(
            "interaction_type".to_string(),
            serde_json::Value::String(interaction_type.to_string()),
        );

        if let Some(id) = element_id {
            event_properties.insert("element_id".to_string(), serde_json::Value::String(id));
        }

        self.manager
            .track_event(AnalyticsEventType::UserInteraction, event_properties, None)?;

        Ok(())
    }

    /// Track metadata generation
    pub fn track_metadata_generation(
        &mut self,
        metadata_type: &str,
        generation_time_ms: u64,
        success: bool,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        let mut event_properties = properties;
        event_properties.insert(
            "metadata_type".to_string(),
            serde_json::Value::String(metadata_type.to_string()),
        );
        event_properties.insert("success".to_string(), serde_json::Value::Bool(success));

        self.manager.track_event(
            AnalyticsEventType::MetadataGenerated,
            event_properties,
            Some(generation_time_ms),
        )?;

        // Also track as performance event
        self.manager.track_performance(
            &format!("metadata_generation:{}", metadata_type),
            generation_time_ms,
            success,
            None,
        )?;

        Ok(())
    }

    /// Track OG image generation
    pub fn track_og_image_generation(
        &mut self,
        image_type: &str,
        generation_time_ms: u64,
        success: bool,
        image_size: Option<(u32, u32)>,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        let mut event_properties = properties;
        event_properties.insert(
            "image_type".to_string(),
            serde_json::Value::String(image_type.to_string()),
        );
        event_properties.insert("success".to_string(), serde_json::Value::Bool(success));

        if let Some((width, height)) = image_size {
            event_properties.insert(
                "image_width".to_string(),
                serde_json::Value::Number(serde_json::Number::from(width)),
            );
            event_properties.insert(
                "image_height".to_string(),
                serde_json::Value::Number(serde_json::Number::from(height)),
            );
        }

        self.manager.track_event(
            AnalyticsEventType::OgImageGenerated,
            event_properties,
            Some(generation_time_ms),
        )?;

        // Also track as performance event
        self.manager.track_performance(
            &format!("og_image_generation:{}", image_type),
            generation_time_ms,
            success,
            None,
        )?;

        Ok(())
    }

    /// Track theme application
    pub fn track_theme_application(
        &mut self,
        theme_id: &str,
        theme_name: &str,
        application_time_ms: u64,
        success: bool,
    ) -> Result<(), MetadataError> {
        let mut properties = HashMap::new();
        properties.insert(
            "theme_id".to_string(),
            serde_json::Value::String(theme_id.to_string()),
        );
        properties.insert(
            "theme_name".to_string(),
            serde_json::Value::String(theme_name.to_string()),
        );
        properties.insert("success".to_string(), serde_json::Value::Bool(success));

        self.manager.track_event(
            AnalyticsEventType::ThemeApplied,
            properties,
            Some(application_time_ms),
        )?;

        Ok(())
    }

    /// Get browser performance information
    pub fn get_browser_performance_info(&self) -> Result<BrowserPerformanceInfo, MetadataError> {
        let window = window().ok_or_else(|| {
            MetadataError::new(ErrorKind::Browser, "Window not available".to_string())
        })?;

        let performance = window.performance().ok_or_else(|| {
            MetadataError::new(
                ErrorKind::Browser,
                "Performance API not available".to_string(),
            )
        })?;

        // Get navigation timing
        let navigation_timing = self.get_navigation_timing(&performance)?;

        // Get resource timing
        let resource_timing = self.get_resource_timing(&performance)?;

        // Get memory information
        let memory_info = self.get_memory_info(&window)?;

        // Get connection information
        let connection_info = self.get_connection_info(&window)?;

        Ok(BrowserPerformanceInfo {
            navigation_timing,
            resource_timing,
            memory_info,
            connection_info,
        })
    }

    /// Get navigation timing information
    fn get_navigation_timing(
        &self,
        performance: &Performance,
    ) -> Result<Option<NavigationTiming>, MetadataError> {
        // Get navigation timing entries
        let navigation_entries = performance.get_entries_by_type("navigation");

        if navigation_entries.length() > 0 {
            if let Some(nav_entry) = navigation_entries
                .get(0)
                .dyn_into::<web_sys::PerformanceNavigationTiming>()
                .ok()
            {
                let load_time = nav_entry.load_event_end() - nav_entry.load_event_start();
                let dom_content_loaded_time = nav_entry.dom_content_loaded_event_end()
                    - nav_entry.dom_content_loaded_event_start();

                // Try to get paint timing
                let paint_entries = performance.get_entries_by_name("first-paint");
                let first_paint_time = if paint_entries.length() > 0 {
                    paint_entries
                        .get(0)
                        .dyn_into::<PerformanceEntry>()
                        .ok()
                        .map(|e| e.start_time())
                } else {
                    None
                };

                let fcp_entries = performance.get_entries_by_name("first-contentful-paint");
                let first_contentful_paint_time = if fcp_entries.length() > 0 {
                    fcp_entries
                        .get(0)
                        .dyn_into::<PerformanceEntry>()
                        .ok()
                        .map(|e| e.start_time())
                } else {
                    None
                };

                return Ok(Some(NavigationTiming {
                    load_time,
                    dom_content_loaded_time,
                    first_paint_time,
                    first_contentful_paint_time,
                    largest_contentful_paint_time: None, // Would need LCP observer
                    first_input_delay: None,             // Would need FID observer
                    cumulative_layout_shift: None,       // Would need CLS observer
                }));
            }
        }

        Ok(None)
    }

    /// Get resource timing information
    fn get_resource_timing(
        &self,
        performance: &Performance,
    ) -> Result<Vec<ResourceTiming>, MetadataError> {
        let resource_entries = performance.get_entries_by_type("resource");
        let mut resource_timing = Vec::new();

        for i in 0..resource_entries.length() {
            if let Some(entry) = resource_entries
                .get(i)
                .dyn_into::<web_sys::PerformanceResourceTiming>()
                .ok()
            {
                let name = entry.name();
                let initiator_type = entry.initiator_type();
                let load_time = entry.load_event_end() - entry.load_event_start();
                let transfer_size = entry.transfer_size();

                resource_timing.push(ResourceTiming {
                    name,
                    resource_type: initiator_type,
                    load_time,
                    transfer_size,
                    compressed_size: None, // Not available in basic ResourceTiming
                });
            }
        }

        Ok(resource_timing)
    }

    /// Get memory information
    fn get_memory_info(
        &self,
        window: &web_sys::Window,
    ) -> Result<Option<MemoryInfo>, MetadataError> {
        // Try to get memory information from performance.memory (Chrome-specific)
        if let Some(performance) = window.performance() {
            // This is a Chrome-specific API, so we need to handle it carefully
            let memory = js_sys::Reflect::get(&performance, &JsValue::from_str("memory"));

            if let Ok(memory_obj) = memory {
                if !memory_obj.is_undefined() {
                    let used_js_heap_size =
                        js_sys::Reflect::get(&memory_obj, &JsValue::from_str("usedJSHeapSize"))
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0) as u32;

                    let total_js_heap_size =
                        js_sys::Reflect::get(&memory_obj, &JsValue::from_str("totalJSHeapSize"))
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0) as u32;

                    let js_heap_size_limit =
                        js_sys::Reflect::get(&memory_obj, &JsValue::from_str("jsHeapSizeLimit"))
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0) as u32;

                    return Ok(Some(MemoryInfo {
                        used_js_heap_size,
                        total_js_heap_size,
                        js_heap_size_limit,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Get connection information
    fn get_connection_info(
        &self,
        window: &web_sys::Window,
    ) -> Result<Option<ConnectionInfo>, MetadataError> {
        // Try to get connection information from navigator.connection
        if let Some(navigator) = window.navigator() {
            let connection = js_sys::Reflect::get(&navigator, &JsValue::from_str("connection"));

            if let Ok(conn_obj) = connection {
                if !conn_obj.is_undefined() {
                    let connection_type =
                        js_sys::Reflect::get(&conn_obj, &JsValue::from_str("type"))
                            .ok()
                            .and_then(|v| v.as_string())
                            .unwrap_or_else(|| "unknown".to_string());

                    let effective_type =
                        js_sys::Reflect::get(&conn_obj, &JsValue::from_str("effectiveType"))
                            .ok()
                            .and_then(|v| v.as_string())
                            .unwrap_or_else(|| "unknown".to_string());

                    let downlink = js_sys::Reflect::get(&conn_obj, &JsValue::from_str("downlink"))
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);

                    let rtt = js_sys::Reflect::get(&conn_obj, &JsValue::from_str("rtt"))
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as u32;

                    let save_data = js_sys::Reflect::get(&conn_obj, &JsValue::from_str("saveData"))
                        .ok()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    return Ok(Some(ConnectionInfo {
                        connection_type,
                        effective_type,
                        downlink,
                        rtt,
                        save_data,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Get analytics manager
    pub fn get_manager(&self) -> &AnalyticsManager {
        &self.manager
    }

    /// Get mutable analytics manager
    pub fn get_manager_mut(&mut self) -> &mut AnalyticsManager {
        &mut self.manager
    }

    /// End analytics session
    pub fn end_session(&mut self) -> Result<(), MetadataError> {
        self.manager.end_session()?;

        // Clean up performance observer
        if let Some(observer) = self.performance_observer.take() {
            let _ = observer.disconnect();
        }

        Ok(())
    }
}

impl Drop for WasmAnalyticsContext {
    fn drop(&mut self) {
        let _ = self.end_session();
    }
}

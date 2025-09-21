//! WASM-specific implementations for client-side metadata management
//!
//! This module provides browser-compatible implementations of metadata functionality
//! that can run in WebAssembly environments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

use crate::wasm::canvas_og::CanvasOgGenerator;

pub mod browser_api;
pub mod canvas_og;
pub mod context;
pub mod error_handler;
pub mod feature_detection;
pub mod performance;
pub mod security;
pub mod storage;

/// WASM-specific metadata context for client-side management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMetadataContext {
    /// Current metadata state
    pub metadata: HashMap<String, String>,
    /// Browser capabilities
    pub capabilities: WasmCapabilities,
    /// Storage backend
    pub storage: WasmStorage,
}

/// Browser capabilities detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmCapabilities {
    /// Web Storage support
    pub web_storage: bool,
    /// Canvas support for image generation
    pub canvas: bool,
    /// WebGL support
    pub webgl: bool,
    /// Fetch API support
    pub fetch: bool,
    /// Web Workers support
    pub web_workers: bool,
}

/// Storage backend for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasmStorage {
    /// Local Storage
    Local,
    /// Session Storage
    Session,
    /// Memory-only storage
    Memory,
}

impl Default for WasmMetadataContext {
    fn default() -> Self {
        Self {
            metadata: HashMap::new(),
            capabilities: WasmCapabilities::detect(),
            storage: WasmStorage::Local,
        }
    }
}

impl WasmMetadataContext {
    /// Create a new WASM metadata context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with specific storage backend
    pub fn with_storage(storage: WasmStorage) -> Self {
        Self {
            metadata: HashMap::new(),
            capabilities: WasmCapabilities::detect(),
            storage,
        }
    }

    /// Check if canvas OG generation is available
    pub fn can_generate_og_images(&self) -> bool {
        self.capabilities.canvas
    }

    /// Get performance optimization recommendations
    pub fn get_performance_recommendations(
        &self,
    ) -> crate::wasm::performance::BundleRecommendations {
        crate::wasm::performance::BundleOptimizer::get_recommendations()
    }

    /// Check optimization status
    pub fn check_optimization_status(&self) -> crate::wasm::performance::OptimizationStatus {
        crate::wasm::performance::BundleOptimizer::check_optimizations()
    }

    /// Get estimated bundle savings
    pub fn get_estimated_savings(&self) -> crate::wasm::performance::BundleSavings {
        crate::wasm::performance::BundleOptimizer::get_estimated_savings()
    }

    /// Get performance tips
    pub fn get_performance_tips(&self) -> Vec<crate::wasm::performance::PerformanceTip> {
        crate::wasm::performance::RuntimeOptimizer::get_performance_tips()
    }

    /// Get memory optimization recommendations
    pub fn get_memory_optimization(&self) -> crate::wasm::performance::MemoryOptimization {
        crate::wasm::performance::RuntimeOptimizer::optimize_memory()
    }

    /// Get security validator
    pub fn get_security_validator(
        &self,
    ) -> Result<crate::wasm::security::SecurityValidator, JsValue> {
        crate::wasm::security::SecurityValidator::new()
    }

    /// Perform security audit
    pub fn perform_security_audit(&self) -> Result<crate::wasm::security::SecurityAudit, JsValue> {
        let validator = crate::wasm::security::SecurityValidator::new()?;
        validator.perform_audit()
    }

    /// Get security recommendations
    pub fn get_security_recommendations(&self) -> Vec<String> {
        crate::wasm::security::SecurityUtils::get_wasm_security_recommendations()
    }

    /// Check if environment is secure
    pub fn is_secure_environment(&self) -> bool {
        crate::wasm::security::SecurityUtils::is_secure_environment()
    }

    /// Get security headers
    pub fn get_security_headers(&self) -> HashMap<String, String> {
        crate::wasm::security::SecurityUtils::get_security_headers()
    }

    /// Get WASM error handler
    pub fn get_error_handler(
        &self,
    ) -> Result<crate::wasm::error_handler::WasmErrorHandler, JsValue> {
        Ok(crate::wasm::error_handler::WasmErrorHandler::new(
            crate::error::ErrorReportingConfig {
                enabled: true,
                max_errors_per_session: 100,
                endpoint: None,
                include_stack_traces: true,
                include_user_context: true,
                sampling_rate: 1.0,
            },
        ))
    }

    /// Get WASM error context
    pub fn get_wasm_error_context(
        &self,
    ) -> Result<crate::wasm::error_handler::WasmErrorContext, JsValue> {
        crate::wasm::error_handler::WasmErrorUtils::get_browser_context()
    }

    /// Create browser error
    pub fn create_browser_error(&self, message: &str) -> crate::error::MetadataError {
        crate::wasm::error_handler::WasmErrorUtils::browser_error(message)
    }

    /// Create storage error
    pub fn create_storage_error(&self, message: &str) -> crate::error::MetadataError {
        crate::wasm::error_handler::WasmErrorUtils::storage_error(message)
    }

    /// Check if error is recoverable in WASM
    pub fn is_wasm_recoverable(&self, error: &crate::error::MetadataError) -> bool {
        crate::wasm::error_handler::WasmErrorUtils::is_wasm_recoverable(error)
    }

    /// Get WASM user-friendly error message
    pub fn get_wasm_user_message(&self, error: &crate::error::MetadataError) -> String {
        crate::wasm::error_handler::WasmErrorUtils::get_wasm_user_message(error)
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: &str, value: &str) -> Result<(), JsValue> {
        self.metadata.insert(key.to_string(), value.to_string());
        self.update_dom_metadata(key, value)?;
        Ok(())
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Update DOM metadata elements
    fn update_dom_metadata(&self, key: &str, value: &str) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;

        match key {
            "title" => {
                document.set_title(value);
            }
            "description" => {
                self.update_meta_tag(&document, "description", value)?;
            }
            "keywords" => {
                self.update_meta_tag(&document, "keywords", value)?;
            }
            "author" => {
                self.update_meta_tag(&document, "author", value)?;
            }
            "viewport" => {
                self.update_meta_tag(&document, "viewport", value)?;
            }
            "theme-color" => {
                self.update_meta_tag(&document, "theme-color", value)?;
            }
            "robots" => {
                self.update_meta_tag(&document, "robots", value)?;
            }
            "canonical" => {
                self.update_link_tag(&document, "canonical", value)?;
            }
            _ => {
                // Generic meta tag
                self.update_meta_tag(&document, key, value)?;
            }
        }

        Ok(())
    }

    /// Update or create a meta tag
    fn update_meta_tag(
        &self,
        document: &Document,
        name: &str,
        content: &str,
    ) -> Result<(), JsValue> {
        let selector = format!("meta[name=\"{}\"]", name);
        let element = document.query_selector(&selector)?;

        if let Some(element) = element {
            // Update existing meta tag
            element.set_attribute("content", content)?;
        } else {
            // Create new meta tag
            let meta = document.create_element("meta")?;
            meta.set_attribute("name", name)?;
            meta.set_attribute("content", content)?;
            document
                .head()
                .ok_or("No head element")?
                .append_child(&meta)?;
        }

        Ok(())
    }

    /// Update or create a link tag
    fn update_link_tag(&self, document: &Document, rel: &str, href: &str) -> Result<(), JsValue> {
        let selector = format!("link[rel=\"{}\"]", rel);
        let element = document.query_selector(&selector)?;

        if let Some(element) = element {
            // Update existing link tag
            element.set_attribute("href", href)?;
        } else {
            // Create new link tag
            let link = document.create_element("link")?;
            link.set_attribute("rel", rel)?;
            link.set_attribute("href", href)?;
            document
                .head()
                .ok_or("No head element")?
                .append_child(&link)?;
        }

        Ok(())
    }
}

impl WasmCapabilities {
    /// Detect browser capabilities
    pub fn detect() -> Self {
        let window = web_sys::window().unwrap_or_else(|| {
            // Fallback for testing
            return JsValue::NULL.into();
        });

        Self {
            web_storage: Self::detect_web_storage(&window),
            canvas: Self::detect_canvas(&window),
            webgl: Self::detect_webgl(&window),
            fetch: Self::detect_fetch(&window),
            web_workers: Self::detect_web_workers(&window),
        }
    }

    fn detect_web_storage(window: &Window) -> bool {
        window.local_storage().is_ok() && window.session_storage().is_ok()
    }

    fn detect_canvas(window: &Window) -> bool {
        if let Some(document) = window.document() {
            if let Ok(canvas) = document.create_element("canvas") {
                return canvas.dyn_into::<web_sys::HtmlCanvasElement>().is_ok();
            }
        }
        false
    }

    fn detect_webgl(window: &Window) -> bool {
        if let Some(document) = window.document() {
            if let Ok(canvas) = document.create_element("canvas") {
                if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                    return canvas.get_context("webgl").is_ok()
                        || canvas.get_context("experimental-webgl").is_ok();
                }
            }
        }
        false
    }

    fn detect_fetch(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("fetch")).unwrap_or(false)
    }

    fn detect_web_workers(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("Worker")).unwrap_or(false)
    }
}

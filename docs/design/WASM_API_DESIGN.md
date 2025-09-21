# WASM API Design Document

**Date**: January 2025  
**Status**: 📐 DESIGN  
**Version**: 1.0  
**Target**: leptos-next-metadata v1.4.0

## Executive Summary

This document defines the WebAssembly (WASM) API design for the leptos-next-metadata library. The API provides a unified interface for metadata management across server and client environments, with WASM-specific optimizations and browser integration.

## API Design Principles

### **1. Unified Interface**
- Consistent API across server and WASM environments
- Feature detection for environment-specific capabilities
- Graceful degradation for unsupported features

### **2. Browser Integration**
- Native JavaScript interop with `wasm-bindgen`
- DOM manipulation for metadata updates
- Web Storage integration for persistence

### **3. Performance Optimization**
- Minimal JavaScript glue code
- Efficient memory management
- Lazy loading of advanced features

### **4. Developer Experience**
- TypeScript definitions for JavaScript users
- Clear error messages and validation
- Comprehensive examples and documentation

## Core API Design

### **1. Metadata Management**

#### **WasmMetadataContext**

```rust
// src/wasm/metadata_context.rs
use wasm_bindgen::prelude::*;
use web_sys::Storage;

#[wasm_bindgen]
pub struct WasmMetadataContext {
    metadata: Metadata,
    storage: Option<Storage>,
    parent: Option<Box<WasmMetadataContext>>,
    security_context: SecurityContext,
}

#[wasm_bindgen]
impl WasmMetadataContext {
    /// Create a new WASM metadata context
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten());
            
        Self {
            metadata: Metadata::default(),
            storage,
            parent: None,
            security_context: SecurityContext::new(),
        }
    }
    
    /// Create a new context with custom configuration
    #[wasm_bindgen]
    pub fn with_config(config: &WasmConfig) -> Result<WasmMetadataContext, JsValue> {
        // Implementation with custom configuration
    }
    
    /// Set metadata for the current context
    #[wasm_bindgen]
    pub fn set_metadata(&mut self, metadata: &Metadata) -> Result<(), JsValue> {
        let mut sanitized_metadata = metadata.clone();
        self.security_context.sanitize_metadata(&mut sanitized_metadata);
        
        self.metadata = sanitized_metadata;
        self.persist_metadata()?;
        self.update_document_metadata()?;
        
        Ok(())
    }
    
    /// Get current metadata
    #[wasm_bindgen]
    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    
    /// Merge metadata with existing metadata
    #[wasm_bindgen]
    pub fn merge_metadata(&mut self, metadata: &Metadata) -> Result<(), JsValue> {
        let merged = self.metadata.merge(metadata)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.set_metadata(&merged)?;
        Ok(())
    }
    
    /// Push metadata to stack (for nested contexts)
    #[wasm_bindgen]
    pub fn push_metadata(&mut self, metadata: &Metadata) -> Result<(), JsValue> {
        let mut sanitized_metadata = metadata.clone();
        self.security_context.sanitize_metadata(&mut sanitized_metadata);
        
        // Create new context with current as parent
        let new_context = WasmMetadataContext {
            metadata: sanitized_metadata,
            storage: self.storage.clone(),
            parent: Some(Box::new(self.clone())),
            security_context: self.security_context.clone(),
        };
        
        *self = new_context;
        self.persist_metadata()?;
        self.update_document_metadata()?;
        
        Ok(())
    }
    
    /// Pop metadata from stack
    #[wasm_bindgen]
    pub fn pop_metadata(&mut self) -> Result<Option<Metadata>, JsValue> {
        if let Some(parent) = self.parent.take() {
            let popped_metadata = self.metadata.clone();
            *self = *parent;
            self.persist_metadata()?;
            self.update_document_metadata()?;
            Ok(Some(popped_metadata))
        } else {
            Ok(None)
        }
    }
    
    /// Clear all metadata
    #[wasm_bindgen]
    pub fn clear_metadata(&mut self) -> Result<(), JsValue> {
        self.metadata = Metadata::default();
        self.persist_metadata()?;
        self.update_document_metadata()?;
        Ok(())
    }
    
    /// Get metadata as JSON string
    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.metadata)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Load metadata from JSON string
    #[wasm_bindgen]
    pub fn from_json(&mut self, json: &str) -> Result<(), JsValue> {
        let metadata: Metadata = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.set_metadata(&metadata)?;
        Ok(())
    }
    
    /// Persist metadata to storage
    fn persist_metadata(&self) -> Result<(), JsValue> {
        if let Some(storage) = &self.storage {
            let json = self.to_json()?;
            storage.set_item("leptos_metadata", &json)?;
        }
        Ok(())
    }
    
    /// Update document metadata in DOM
    fn update_document_metadata(&self) -> Result<(), JsValue> {
        update_document_metadata(&self.metadata)?;
        Ok(())
    }
}

// Clone implementation for WASM
impl Clone for WasmMetadataContext {
    fn clone(&self) -> Self {
        Self {
            metadata: self.metadata.clone(),
            storage: self.storage.clone(),
            parent: self.parent.clone(),
            security_context: self.security_context.clone(),
        }
    }
}
```

#### **WasmConfig**

```rust
// src/wasm/config.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmConfig {
    enable_persistence: bool,
    enable_dom_updates: bool,
    max_cache_size: usize,
    sanitize_input: bool,
    allowed_origins: Vec<String>,
}

#[wasm_bindgen]
impl WasmConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            enable_persistence: true,
            enable_dom_updates: true,
            max_cache_size: 10 * 1024 * 1024, // 10MB
            sanitize_input: true,
            allowed_origins: vec![],
        }
    }
    
    #[wasm_bindgen]
    pub fn with_persistence(mut self, enable: bool) -> Self {
        self.enable_persistence = enable;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_dom_updates(mut self, enable: bool) -> Self {
        self.enable_dom_updates = enable;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_max_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_sanitization(mut self, enable: bool) -> Self {
        self.sanitize_input = enable;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }
}
```

### **2. Browser API Integration**

#### **Document Metadata Updates**

```rust
// src/wasm/browser_apis.rs
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlMetaElement, HtmlTitleElement};

/// Update document metadata in the DOM
pub fn update_document_metadata(metadata: &Metadata) -> Result<(), JsValue> {
    let window = window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    
    // Update title
    if let Some(title) = &metadata.title {
        update_title(&document, &title.to_string())?;
    }
    
    // Update meta tags
    update_meta_tag(&document, "description", &metadata.description)?;
    update_meta_tag(&document, "keywords", &metadata.keywords)?;
    update_meta_tag(&document, "author", &metadata.authors)?;
    
    // Update Open Graph tags
    if let Some(og) = &metadata.open_graph {
        update_meta_property(&document, "og:title", &og.title)?;
        update_meta_property(&document, "og:description", &og.description)?;
        update_meta_property(&document, "og:type", &og.r#type)?;
        update_meta_property(&document, "og:url", &og.url)?;
        update_meta_property(&document, "og:image", &og.image)?;
        update_meta_property(&document, "og:site_name", &og.site_name)?;
    }
    
    // Update Twitter Card tags
    if let Some(twitter) = &metadata.twitter {
        update_meta_name(&document, "twitter:card", &twitter.card)?;
        update_meta_name(&document, "twitter:site", &twitter.site)?;
        update_meta_name(&document, "twitter:creator", &twitter.creator)?;
        update_meta_name(&document, "twitter:title", &twitter.title)?;
        update_meta_name(&document, "twitter:description", &twitter.description)?;
        update_meta_name(&document, "twitter:image", &twitter.image)?;
    }
    
    // Update robots meta
    if let Some(robots) = &metadata.robots {
        update_meta_name(&document, "robots", &robots.to_string())?;
    }
    
    // Update canonical URL
    if let Some(canonical) = &metadata.canonical {
        update_canonical_link(&document, canonical)?;
    }
    
    // Update theme color
    if let Some(theme_color) = &metadata.theme_color {
        update_meta_name(&document, "theme-color", &theme_color.to_string())?;
    }
    
    Ok(())
}

fn update_title(document: &Document, title: &str) -> Result<(), JsValue> {
    document.set_title(title);
    Ok(())
}

fn update_meta_tag(document: &Document, name: &str, content: &Option<String>) -> Result<(), JsValue> {
    if let Some(content) = content {
        let meta = get_or_create_meta_tag(document, name, "name")?;
        meta.set_attribute("content", content)?;
    }
    Ok(())
}

fn update_meta_property(document: &Document, property: &str, content: &Option<String>) -> Result<(), JsValue> {
    if let Some(content) = content {
        let meta = get_or_create_meta_tag(document, property, "property")?;
        meta.set_attribute("content", content)?;
    }
    Ok(())
}

fn update_meta_name(document: &Document, name: &str, content: &Option<String>) -> Result<(), JsValue> {
    if let Some(content) = content {
        let meta = get_or_create_meta_tag(document, name, "name")?;
        meta.set_attribute("content", content)?;
    }
    Ok(())
}

fn get_or_create_meta_tag(document: &Document, name: &str, attr_type: &str) -> Result<HtmlMetaElement, JsValue> {
    let selector = format!("meta[{}='{}']", attr_type, name);
    let existing = document.query_selector(&selector)?;
    
    if let Some(meta) = existing {
        Ok(meta.dyn_into::<HtmlMetaElement>()?)
    } else {
        let meta = document.create_element("meta")?;
        meta.set_attribute(attr_type, name)?;
        document.head()?.append_child(&meta)?;
        Ok(meta.dyn_into::<HtmlMetaElement>()?)
    }
}

fn update_canonical_link(document: &Document, url: &str) -> Result<(), JsValue> {
    let existing = document.query_selector("link[rel='canonical']")?;
    
    if let Some(link) = existing {
        link.set_attribute("href", url)?;
    } else {
        let link = document.create_element("link")?;
        link.set_attribute("rel", "canonical")?;
        link.set_attribute("href", url)?;
        document.head()?.append_child(&link)?;
    }
    
    Ok(())
}
```

### **3. Canvas-Based OG Image Generation**

#### **CanvasOgImageGenerator**

```rust
// src/wasm/canvas_og_image.rs
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
pub struct CanvasOgImageGenerator {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CanvasOgImageGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<CanvasOgImageGenerator, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
        
        canvas.set_width(width);
        canvas.set_height(height);
        
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        Ok(CanvasOgImageGenerator { canvas, ctx, width, height })
    }
    
    /// Generate OG image with text content
    #[wasm_bindgen]
    pub fn generate_text_image(&self, params: &TextOgImageParams) -> Result<Vec<u8>, JsValue> {
        // Clear canvas
        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        
        // Set background
        self.ctx.set_fill_style(&params.background_color.into());
        self.ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        
        // Add title
        if let Some(title) = &params.title {
            self.ctx.set_font(&format!("{}px {}", params.title_font_size, params.font_family));
            self.ctx.set_fill_style(&params.title_color.into());
            self.ctx.set_text_align("left");
            self.ctx.set_text_baseline("top");
            self.ctx.fill_text(title, params.padding, params.padding)?;
        }
        
        // Add description
        if let Some(description) = &params.description {
            let y_pos = params.padding + params.title_font_size as f64 + 20.0;
            self.ctx.set_font(&format!("{}px {}", params.description_font_size, params.font_family));
            self.ctx.set_fill_style(&params.description_color.into());
            self.ctx.fill_text(description, params.padding, y_pos)?;
        }
        
        // Add site name
        if let Some(site_name) = &params.site_name {
            let y_pos = self.height as f64 - params.padding - params.site_font_size as f64;
            self.ctx.set_font(&format!("{}px {}", params.site_font_size, params.font_family));
            self.ctx.set_fill_style(&params.site_color.into());
            self.ctx.fill_text(site_name, params.padding, y_pos)?;
        }
        
        // Convert to image data
        let image_data = self.ctx.get_image_data(0.0, 0.0, self.width as f64, self.height as f64)?;
        Ok(image_data.data().to_vec())
    }
    
    /// Generate OG image with background image
    #[wasm_bindgen]
    pub fn generate_image_background(&self, params: &ImageOgImageParams) -> Result<Vec<u8>, JsValue> {
        // Implementation for image-based backgrounds
        // This would require loading images and compositing
        Ok(vec![])
    }
    
    /// Get canvas as data URL
    #[wasm_bindgen]
    pub fn to_data_url(&self, format: &str, quality: f64) -> Result<String, JsValue> {
        self.canvas.to_data_url_with_type_and_quality(format, quality)
    }
    
    /// Get canvas as blob
    #[wasm_bindgen]
    pub fn to_blob(&self, format: &str, quality: f64) -> Result<js_sys::Promise, JsValue> {
        self.canvas.to_blob_with_type_and_quality(format, quality)
    }
}

#[wasm_bindgen]
pub struct TextOgImageParams {
    title: Option<String>,
    description: Option<String>,
    site_name: Option<String>,
    background_color: String,
    title_color: String,
    description_color: String,
    site_color: String,
    font_family: String,
    title_font_size: u32,
    description_font_size: u32,
    site_font_size: u32,
    padding: f64,
}

#[wasm_bindgen]
impl TextOgImageParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            site_name: None,
            background_color: "#ffffff".to_string(),
            title_color: "#000000".to_string(),
            description_color: "#666666".to_string(),
            site_color: "#999999".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            title_font_size: 48,
            description_font_size: 24,
            site_font_size: 16,
            padding: 40.0,
        }
    }
    
    #[wasm_bindgen]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    
    #[wasm_bindgen]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    #[wasm_bindgen]
    pub fn with_site_name(mut self, site_name: String) -> Self {
        self.site_name = Some(site_name);
        self
    }
    
    #[wasm_bindgen]
    pub fn with_background_color(mut self, color: String) -> Self {
        self.background_color = color;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_title_color(mut self, color: String) -> Self {
        self.title_color = color;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_description_color(mut self, color: String) -> Self {
        self.description_color = color;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_site_color(mut self, color: String) -> Self {
        self.site_color = color;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_font_family(mut self, font: String) -> Self {
        self.font_family = font;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_title_font_size(mut self, size: u32) -> Self {
        self.title_font_size = size;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_description_font_size(mut self, size: u32) -> Self {
        self.description_font_size = size;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_site_font_size(mut self, size: u32) -> Self {
        self.site_font_size = size;
        self
    }
    
    #[wasm_bindgen]
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
}
```

### **4. Web Storage Cache**

#### **WasmCache**

```rust
// src/wasm/cache.rs
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct WasmCache {
    memory_cache: HashMap<String, CacheEntry>,
    storage: Option<Storage>,
    max_memory_entries: usize,
    max_cache_size: usize,
}

#[wasm_bindgen]
impl WasmCache {
    #[wasm_bindgen(constructor)]
    pub fn new(max_entries: usize, max_size: usize) -> Self {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten());
            
        Self {
            memory_cache: HashMap::new(),
            storage,
            max_memory_entries: max_entries,
            max_cache_size: max_size,
        }
    }
    
    /// Get cached data
    #[wasm_bindgen]
    pub async fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        // Check memory cache first
        if let Some(entry) = self.memory_cache.get(key) {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            } else {
                self.memory_cache.remove(key);
            }
        }
        
        // Check persistent storage
        if let Some(storage) = &self.storage {
            if let Ok(Some(data)) = storage.get_item(key) {
                if let Ok(decoded) = base64::decode(&data) {
                    let entry = CacheEntry::new(decoded);
                    self.memory_cache.insert(key.to_string(), entry);
                    return Some(decoded);
                }
            }
        }
        
        None
    }
    
    /// Set cached data
    #[wasm_bindgen]
    pub async fn set(&mut self, key: &str, data: &[u8]) -> Result<(), JsValue> {
        let entry = CacheEntry::new(data.to_vec());
        
        // Update memory cache
        self.memory_cache.insert(key.to_string(), entry);
        
        // Persist to storage if enabled
        if let Some(storage) = &self.storage {
            let encoded = base64::encode(data);
            storage.set_item(key, &encoded)?;
        }
        
        // Cleanup if needed
        if self.memory_cache.len() > self.max_memory_entries {
            self.cleanup_memory_cache();
        }
        
        Ok(())
    }
    
    /// Remove cached data
    #[wasm_bindgen]
    pub async fn remove(&mut self, key: &str) -> Result<(), JsValue> {
        self.memory_cache.remove(key);
        
        if let Some(storage) = &self.storage {
            storage.remove_item(key)?;
        }
        
        Ok(())
    }
    
    /// Clear all cached data
    #[wasm_bindgen]
    pub async fn clear(&mut self) -> Result<(), JsValue> {
        self.memory_cache.clear();
        
        if let Some(storage) = &self.storage {
            storage.clear()?;
        }
        
        Ok(())
    }
    
    /// Get cache statistics
    #[wasm_bindgen]
    pub fn get_stats(&self) -> CacheStats {
        let mut total_size = 0;
        let mut expired_entries = 0;
        
        for entry in self.memory_cache.values() {
            total_size += entry.data.len();
            if entry.is_expired() {
                expired_entries += 1;
            }
        }
        
        CacheStats {
            entries: self.memory_cache.len(),
            max_entries: self.max_memory_entries,
            total_size,
            max_size: self.max_cache_size,
            expired_entries,
        }
    }
    
    fn cleanup_memory_cache(&mut self) {
        // Remove expired entries first
        self.memory_cache.retain(|_, entry| !entry.is_expired());
        
        // If still over limit, remove oldest entries
        if self.memory_cache.len() > self.max_memory_entries {
            let mut entries: Vec<_> = self.memory_cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.created_at);
            
            let to_remove = entries.len() - self.max_memory_entries;
            for (key, _) in entries.iter().take(to_remove) {
                self.memory_cache.remove(*key);
            }
        }
    }
}

#[wasm_bindgen]
pub struct CacheStats {
    entries: usize,
    max_entries: usize,
    total_size: usize,
    max_size: usize,
    expired_entries: usize,
}

#[wasm_bindgen]
impl CacheStats {
    #[wasm_bindgen(getter)]
    pub fn entries(&self) -> usize {
        self.entries
    }
    
    #[wasm_bindgen(getter)]
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }
    
    #[wasm_bindgen(getter)]
    pub fn total_size(&self) -> usize {
        self.total_size
    }
    
    #[wasm_bindgen(getter)]
    pub fn max_size(&self) -> usize {
        self.max_size
    }
    
    #[wasm_bindgen(getter)]
    pub fn expired_entries(&self) -> usize {
        self.expired_entries
    }
}

struct CacheEntry {
    data: Vec<u8>,
    created_at: f64,
    ttl: f64,
}

impl CacheEntry {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            created_at: js_sys::Date::now(),
            ttl: 3600000.0, // 1 hour in milliseconds
        }
    }
    
    fn is_expired(&self) -> bool {
        js_sys::Date::now() - self.created_at > self.ttl
    }
}
```

## Feature Detection API

### **Environment Detection**

```rust
// src/wasm/feature_detection.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FeatureDetector;

#[wasm_bindgen]
impl FeatureDetector {
    /// Check if a feature is available in the current environment
    #[wasm_bindgen]
    pub fn is_feature_available(feature: &str) -> bool {
        match feature {
            "og-images" => false, // Server-only
            "file-conventions" => false, // Server-only
            "http-client" => false, // Server-only
            "advanced-caching" => false, // Server-only
            "canvas-og-images" => true, // WASM-only
            "web-storage" => true, // WASM-only
            "browser-apis" => true, // WASM-only
            "core-metadata" => true, // Universal
            "json-ld" => true, // Universal
            "macros" => true, // Universal
            _ => false,
        }
    }
    
    /// Get all available features in the current environment
    #[wasm_bindgen]
    pub fn get_available_features() -> Vec<String> {
        vec![
            "core-metadata".to_string(),
            "json-ld".to_string(),
            "macros".to_string(),
            "canvas-og-images".to_string(),
            "web-storage".to_string(),
            "browser-apis".to_string(),
        ]
    }
    
    /// Get environment information
    #[wasm_bindgen]
    pub fn get_environment_info() -> EnvironmentInfo {
        EnvironmentInfo {
            is_wasm: true,
            is_server: false,
            is_client: true,
            user_agent: get_user_agent(),
            supports_web_storage: supports_web_storage(),
            supports_canvas: supports_canvas(),
        }
    }
}

#[wasm_bindgen]
pub struct EnvironmentInfo {
    is_wasm: bool,
    is_server: bool,
    is_client: bool,
    user_agent: String,
    supports_web_storage: bool,
    supports_canvas: bool,
}

#[wasm_bindgen]
impl EnvironmentInfo {
    #[wasm_bindgen(getter)]
    pub fn is_wasm(&self) -> bool {
        self.is_wasm
    }
    
    #[wasm_bindgen(getter)]
    pub fn is_server(&self) -> bool {
        self.is_server
    }
    
    #[wasm_bindgen(getter)]
    pub fn is_client(&self) -> bool {
        self.is_client
    }
    
    #[wasm_bindgen(getter)]
    pub fn user_agent(&self) -> String {
        self.user_agent.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn supports_web_storage(&self) -> bool {
        self.supports_web_storage
    }
    
    #[wasm_bindgen(getter)]
    pub fn supports_canvas(&self) -> bool {
        self.supports_canvas
    }
}

fn get_user_agent() -> String {
    window()
        .and_then(|w| w.navigator().user_agent().ok())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn supports_web_storage() -> bool {
    window()
        .and_then(|w| w.local_storage().ok())
        .is_some()
}

fn supports_canvas() -> bool {
    window()
        .and_then(|w| w.document().ok())
        .and_then(|d| d.create_element("canvas").ok())
        .is_some()
}
```

## Error Handling

### **WASM-Specific Error Types**

```rust
// src/wasm/errors.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMetadataError {
    message: String,
    error_type: String,
}

#[wasm_bindgen]
impl WasmMetadataError {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, error_type: String) -> Self {
        Self { message, error_type }
    }
    
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn error_type(&self) -> String {
        self.error_type.clone()
    }
    
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.error_type, self.message)
    }
}

pub fn handle_wasm_error(error: JsValue) -> WasmMetadataError {
    let message = error.as_string().unwrap_or_else(|| "Unknown WASM error".to_string());
    WasmMetadataError::new(message, "WasmError".to_string())
}

pub fn handle_validation_error(error: &str) -> WasmMetadataError {
    WasmMetadataError::new(error.to_string(), "ValidationError".to_string())
}

pub fn handle_feature_error(feature: &str) -> WasmMetadataError {
    WasmMetadataError::new(
        format!("Feature '{}' is not available in WASM environment", feature),
        "FeatureNotAvailable".to_string(),
    )
}
```

## TypeScript Definitions

### **Generated TypeScript Definitions**

```typescript
// pkg/leptos_next_metadata.d.ts
export class WasmMetadataContext {
  constructor();
  static withConfig(config: WasmConfig): WasmMetadataContext;
  setMetadata(metadata: Metadata): void;
  getMetadata(): Metadata;
  mergeMetadata(metadata: Metadata): void;
  pushMetadata(metadata: Metadata): void;
  popMetadata(): Metadata | null;
  clearMetadata(): void;
  toJson(): string;
  fromJson(json: string): void;
}

export class WasmConfig {
  constructor();
  withPersistence(enable: boolean): WasmConfig;
  withDomUpdates(enable: boolean): WasmConfig;
  withMaxCacheSize(size: number): WasmConfig;
  withSanitization(enable: boolean): WasmConfig;
  withAllowedOrigins(origins: string[]): WasmConfig;
}

export class CanvasOgImageGenerator {
  constructor(width: number, height: number);
  generateTextImage(params: TextOgImageParams): Uint8Array;
  generateImageBackground(params: ImageOgImageParams): Uint8Array;
  toDataUrl(format: string, quality: number): string;
  toBlob(format: string, quality: number): Promise<Blob>;
}

export class TextOgImageParams {
  constructor();
  withTitle(title: string): TextOgImageParams;
  withDescription(description: string): TextOgImageParams;
  withSiteName(siteName: string): TextOgImageParams;
  withBackgroundColor(color: string): TextOgImageParams;
  withTitleColor(color: string): TextOgImageParams;
  withDescriptionColor(color: string): TextOgImageParams;
  withSiteColor(color: string): TextOgImageParams;
  withFontFamily(font: string): TextOgImageParams;
  withTitleFontSize(size: number): TextOgImageParams;
  withDescriptionFontSize(size: number): TextOgImageParams;
  withSiteFontSize(size: number): TextOgImageParams;
  withPadding(padding: number): TextOgImageParams;
}

export class WasmCache {
  constructor(maxEntries: number, maxSize: number);
  get(key: string): Promise<Uint8Array | null>;
  set(key: string, data: Uint8Array): Promise<void>;
  remove(key: string): Promise<void>;
  clear(): Promise<void>;
  getStats(): CacheStats;
}

export class CacheStats {
  readonly entries: number;
  readonly maxEntries: number;
  readonly totalSize: number;
  readonly maxSize: number;
  readonly expiredEntries: number;
}

export class FeatureDetector {
  static isFeatureAvailable(feature: string): boolean;
  static getAvailableFeatures(): string[];
  static getEnvironmentInfo(): EnvironmentInfo;
}

export class EnvironmentInfo {
  readonly isWasm: boolean;
  readonly isServer: boolean;
  readonly isClient: boolean;
  readonly userAgent: string;
  readonly supportsWebStorage: boolean;
  readonly supportsCanvas: boolean;
}

export class WasmMetadataError {
  constructor(message: string, errorType: string);
  readonly message: string;
  readonly errorType: string;
  toString(): string;
}

export interface Metadata {
  title?: Title;
  description?: string;
  keywords?: Keywords;
  authors?: Authors;
  robots?: Robots;
  openGraph?: OpenGraph;
  twitter?: Twitter;
  jsonLd?: JsonLd;
  canonical?: CanonicalUrl;
  alternateLinks?: AlternateLink[];
  viewport?: Viewport;
  themeColor?: ThemeColor;
  colorScheme?: ColorScheme;
  referrer?: ReferrerPolicy;
  formatDetection?: FormatDetection;
  additional?: Record<string, AdditionalValue>;
}

export interface Title {
  static?: string;
  template?: string;
  default?: string;
}

export interface Keywords {
  keywords: string[];
}

export interface Authors {
  authors: Author[];
}

export interface Author {
  name: string;
  url?: string;
}

export interface Robots {
  index: boolean;
  follow: boolean;
  nocache: boolean;
  noarchive: boolean;
  nosnippet: boolean;
  noimageindex: boolean;
  notranslate: boolean;
}

export interface OpenGraph {
  title?: string;
  description?: string;
  type?: string;
  url?: string;
  image?: string;
  siteName?: string;
}

export interface Twitter {
  card?: string;
  site?: string;
  creator?: string;
  title?: string;
  description?: string;
  image?: string;
}

export interface JsonLd {
  type: string;
  data: Record<string, any>;
}

export interface CanonicalUrl {
  url: string;
}

export interface AlternateLink {
  href: string;
  hreflang?: string;
  media?: string;
  type?: string;
}

export interface Viewport {
  width?: string;
  height?: string;
  initialScale?: number;
  minimumScale?: number;
  maximumScale?: number;
  userScalable?: boolean;
}

export interface ThemeColor {
  color: string;
  media?: string;
}

export interface ColorScheme {
  scheme: string;
}

export interface ReferrerPolicy {
  policy: string;
}

export interface FormatDetection {
  telephone: boolean;
  date: boolean;
  address: boolean;
  email: boolean;
  url: boolean;
}

export type AdditionalValue = string | number | boolean | string[] | Record<string, any>;
```

## Usage Examples

### **Basic Usage**

```javascript
// Basic metadata management
import { WasmMetadataContext, Metadata } from 'leptos-next-metadata';

const context = new WasmMetadataContext();

const metadata = {
  title: { static: "My Page" },
  description: "This is my awesome page",
  openGraph: {
    title: "My Page - Open Graph",
    type: "website",
    image: "/og-image.png"
  }
};

context.setMetadata(metadata);
```

### **Advanced Usage with Configuration**

```javascript
// Advanced configuration
import { WasmMetadataContext, WasmConfig } from 'leptos-next-metadata';

const config = new WasmConfig()
  .withPersistence(true)
  .withDomUpdates(true)
  .withMaxCacheSize(5 * 1024 * 1024) // 5MB
  .withSanitization(true);

const context = WasmMetadataContext.withConfig(config);

// Nested metadata contexts
context.pushMetadata({
  title: { static: "Blog Post" },
  description: "A great blog post"
});

// Later...
context.popMetadata(); // Returns to previous metadata
```

### **OG Image Generation**

```javascript
// Canvas-based OG image generation
import { CanvasOgImageGenerator, TextOgImageParams } from 'leptos-next-metadata';

const generator = new CanvasOgImageGenerator(1200, 630);

const params = new TextOgImageParams()
  .withTitle("My Awesome Blog Post")
  .withDescription("This is a great blog post about Rust and WASM")
  .withSiteName("My Blog")
  .withBackgroundColor("#1a1a1a")
  .withTitleColor("#ffffff")
  .withDescriptionColor("#cccccc")
  .withFontFamily("Inter, sans-serif");

const imageData = generator.generateTextImage(params);
const dataUrl = generator.toDataUrl("image/png", 0.9);
```

### **Caching**

```javascript
// Web storage caching
import { WasmCache } from 'leptos-next-metadata';

const cache = new WasmCache(100, 10 * 1024 * 1024); // 100 entries, 10MB

// Cache some data
await cache.set("my-key", new Uint8Array([1, 2, 3, 4]));

// Retrieve cached data
const data = await cache.get("my-key");

// Get cache statistics
const stats = cache.getStats();
console.log(`Cache has ${stats.entries} entries`);
```

### **Feature Detection**

```javascript
// Feature detection
import { FeatureDetector } from 'leptos-next-metadata';

// Check if a feature is available
if (FeatureDetector.isFeatureAvailable("canvas-og-images")) {
  // Use canvas-based OG image generation
  const generator = new CanvasOgImageGenerator(1200, 630);
}

// Get all available features
const features = FeatureDetector.getAvailableFeatures();
console.log("Available features:", features);

// Get environment information
const env = FeatureDetector.getEnvironmentInfo();
console.log("Environment:", env);
```

## Conclusion

This WASM API design provides a comprehensive, browser-integrated interface for metadata management in WebAssembly environments. The API offers:

1. **Unified Interface**: Consistent API across server and WASM environments
2. **Browser Integration**: Native DOM manipulation and Web Storage support
3. **Performance Optimization**: Efficient memory management and lazy loading
4. **Developer Experience**: TypeScript definitions and clear error handling
5. **Feature Detection**: Runtime feature availability checking
6. **Canvas Integration**: Client-side OG image generation capabilities

The design supports the phased implementation approach and provides a solid foundation for client-side metadata management in Leptos applications.

---

**Design completed by**: AI Agent  
**Status**: 📐 Ready for implementation  
**Next step**: Begin Phase 1 implementation

//! WASM metadata context implementation
//!
//! Provides client-side metadata management with DOM integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

use crate::metadata::{
    Author, Authors, CanonicalUrl, Description, Keywords, Metadata, Robots, ThemeColor, Title,
    Viewport,
};

/// WASM-specific metadata context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmContext {
    /// Current metadata state
    pub metadata: Metadata,
    /// Storage backend
    pub storage_backend: StorageBackend,
}

/// Storage backend options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    /// Local Storage
    LocalStorage,
    /// Session Storage
    SessionStorage,
    /// Memory-only storage
    Memory,
}

impl Default for WasmContext {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            storage_backend: StorageBackend::LocalStorage,
        }
    }
}

impl WasmContext {
    /// Create a new WASM context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with specific storage backend
    pub fn with_storage(storage: StorageBackend) -> Self {
        Self {
            metadata: Metadata::default(),
            storage_backend: storage,
        }
    }

    /// Set page title
    pub fn set_title(&mut self, title: Title) -> Result<(), JsValue> {
        self.metadata.title = Some(title.clone());
        self.update_dom_title(&title)?;
        self.save_to_storage("title", &title)?;
        Ok(())
    }

    /// Set page description
    pub fn set_description(&mut self, description: Description) -> Result<(), JsValue> {
        self.metadata.description = Some(description.clone());
        self.update_dom_meta("description", &description.to_string())?;
        self.save_to_storage("description", &description)?;
        Ok(())
    }

    /// Set page keywords
    pub fn set_keywords(&mut self, keywords: Keywords) -> Result<(), JsValue> {
        self.metadata.keywords = Some(keywords.clone());
        self.update_dom_meta("keywords", &keywords.to_string())?;
        self.save_to_storage("keywords", &keywords)?;
        Ok(())
    }

    /// Set page author
    pub fn set_author(&mut self, author: Author) -> Result<(), JsValue> {
        self.metadata.authors = Some(Authors::Single(author.clone()));
        self.update_dom_meta("author", &author.to_string())?;
        self.save_to_storage("author", &author)?;
        Ok(())
    }

    /// Set viewport
    pub fn set_viewport(&mut self, viewport: Viewport) -> Result<(), JsValue> {
        self.metadata.viewport = Some(viewport.clone());
        self.update_dom_meta("viewport", &viewport.to_string())?;
        self.save_to_storage("viewport", &viewport)?;
        Ok(())
    }

    /// Set theme color
    pub fn set_theme_color(&mut self, theme_color: ThemeColor) -> Result<(), JsValue> {
        self.metadata.theme_color = Some(theme_color.clone());
        self.update_dom_meta("theme-color", &theme_color.to_string())?;
        self.save_to_storage("theme_color", &theme_color)?;
        Ok(())
    }

    /// Set robots
    pub fn set_robots(&mut self, robots: Robots) -> Result<(), JsValue> {
        self.metadata.robots = Some(robots.clone());
        self.update_dom_meta("robots", &robots.to_string())?;
        self.save_to_storage("robots", &robots)?;
        Ok(())
    }

    /// Set canonical URL
    pub fn set_canonical_url(&mut self, canonical: CanonicalUrl) -> Result<(), JsValue> {
        self.metadata.canonical = Some(canonical.clone());
        self.update_dom_link("canonical", &canonical.to_string())?;
        self.save_to_storage("canonical_url", &canonical)?;
        Ok(())
    }

    /// Update DOM title
    fn update_dom_title(&self, title: &Title) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                document.set_title(&title.to_string());
            }
        }
        Ok(())
    }

    /// Update DOM meta tag
    fn update_dom_meta(&self, name: &str, content: &str) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
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
            }
        }
        Ok(())
    }

    /// Update DOM link tag
    fn update_dom_link(&self, rel: &str, href: &str) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
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
            }
        }
        Ok(())
    }

    /// Save to storage backend
    fn save_to_storage<T: Serialize>(&self, key: &str, value: &T) -> Result<(), JsValue> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

        match self.storage_backend {
            StorageBackend::LocalStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        storage.set_item(key, &serialized)?;
                    }
                }
            }
            StorageBackend::SessionStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.session_storage() {
                        storage.set_item(key, &serialized)?;
                    }
                }
            }
            StorageBackend::Memory => {
                // Memory storage would be handled by the context itself
                // This is a placeholder for future implementation
            }
        }
        Ok(())
    }

    /// Load from storage backend
    pub fn load_from_storage<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, JsValue> {
        let stored = match self.storage_backend {
            StorageBackend::LocalStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        storage.get_item(key).ok().flatten()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            StorageBackend::SessionStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.session_storage() {
                        storage.get_item(key).ok().flatten()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            StorageBackend::Memory => {
                // Memory storage would be handled by the context itself
                None
            }
        };

        if let Some(stored) = stored {
            let deserialized: T = serde_json::from_str(&stored)
                .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Clear storage
    pub fn clear_storage(&self) -> Result<(), JsValue> {
        match self.storage_backend {
            StorageBackend::LocalStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        storage.clear()?;
                    }
                }
            }
            StorageBackend::SessionStorage => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.session_storage() {
                        storage.clear()?;
                    }
                }
            }
            StorageBackend::Memory => {
                // Memory storage would be handled by the context itself
            }
        }
        Ok(())
    }

    /// Get current metadata
    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Update metadata from storage
    pub fn load_metadata_from_storage(&mut self) -> Result<(), JsValue> {
        if let Some(title) = self.load_from_storage::<Title>("title")? {
            self.metadata.title = Some(title);
        }
        if let Some(description) = self.load_from_storage::<Description>("description")? {
            self.metadata.description = Some(description);
        }
        if let Some(keywords) = self.load_from_storage::<Keywords>("keywords")? {
            self.metadata.keywords = Some(keywords);
        }
        if let Some(author) = self.load_from_storage::<Author>("author")? {
            self.metadata.authors = Some(Authors::Single(author));
        }
        if let Some(viewport) = self.load_from_storage::<Viewport>("viewport")? {
            self.metadata.viewport = Some(viewport);
        }
        if let Some(theme_color) = self.load_from_storage::<ThemeColor>("theme_color")? {
            self.metadata.theme_color = Some(theme_color);
        }
        if let Some(robots) = self.load_from_storage::<Robots>("robots")? {
            self.metadata.robots = Some(robots);
        }
        if let Some(canonical) = self.load_from_storage::<CanonicalUrl>("canonical_url")? {
            self.metadata.canonical = Some(canonical);
        }
        Ok(())
    }
}

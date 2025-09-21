//! Web Storage implementation for WASM
//!
//! Provides persistent storage for metadata in browser environments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Storage;

/// Storage backend for WASM metadata
#[derive(Debug, Clone)]
pub enum WasmStorage {
    /// Local Storage (persistent across sessions)
    Local(Storage),
    /// Session Storage (cleared when tab closes)
    Session(Storage),
    /// Memory-only storage (lost on page reload)
    Memory(HashMap<String, String>),
}

impl WasmStorage {
    /// Create local storage backend
    pub fn local() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let storage = window
            .local_storage()?
            .ok_or("Local storage not available")?;
        Ok(Self::Local(storage))
    }

    /// Create session storage backend
    pub fn session() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let storage = window
            .session_storage()?
            .ok_or("Session storage not available")?;
        Ok(Self::Session(storage))
    }

    /// Create memory storage backend
    pub fn memory() -> Self {
        Self::Memory(HashMap::new())
    }

    /// Store a value
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), JsValue> {
        match self {
            Self::Local(storage) => {
                storage.set_item(key, value)?;
            }
            Self::Session(storage) => {
                storage.set_item(key, value)?;
            }
            Self::Memory(map) => {
                map.insert(key.to_string(), value.to_string());
            }
        }
        Ok(())
    }

    /// Retrieve a value
    pub fn get(&self, key: &str) -> Result<Option<String>, JsValue> {
        match self {
            Self::Local(storage) => Ok(storage.get_item(key)?),
            Self::Session(storage) => Ok(storage.get_item(key)?),
            Self::Memory(map) => Ok(map.get(key).cloned()),
        }
    }

    /// Remove a value
    pub fn remove(&mut self, key: &str) -> Result<(), JsValue> {
        match self {
            Self::Local(storage) => {
                storage.remove_item(key)?;
            }
            Self::Session(storage) => {
                storage.remove_item(key)?;
            }
            Self::Memory(map) => {
                map.remove(key);
            }
        }
        Ok(())
    }

    /// Clear all values
    pub fn clear(&mut self) -> Result<(), JsValue> {
        match self {
            Self::Local(storage) => {
                storage.clear()?;
            }
            Self::Session(storage) => {
                storage.clear()?;
            }
            Self::Memory(map) => {
                map.clear();
            }
        }
        Ok(())
    }

    /// Get storage length
    pub fn len(&self) -> Result<usize, JsValue> {
        match self {
            Self::Local(storage) => Ok(storage.length()? as usize),
            Self::Session(storage) => Ok(storage.length()? as usize),
            Self::Memory(map) => Ok(map.len()),
        }
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> Result<bool, JsValue> {
        Ok(self.len()? == 0)
    }

    /// Get all keys
    pub fn keys(&self) -> Result<Vec<String>, JsValue> {
        match self {
            Self::Local(storage) => {
                let mut keys = Vec::new();
                for i in 0..storage.length()? {
                    if let Some(key) = storage.key(i)? {
                        keys.push(key);
                    }
                }
                Ok(keys)
            }
            Self::Session(storage) => {
                let mut keys = Vec::new();
                for i in 0..storage.length()? {
                    if let Some(key) = storage.key(i)? {
                        keys.push(key);
                    }
                }
                Ok(keys)
            }
            Self::Memory(map) => Ok(map.keys().cloned().collect()),
        }
    }

    /// Store serialized data
    pub fn store_serialized<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), JsValue> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;
        self.set(key, &serialized)
    }

    /// Retrieve and deserialize data
    pub fn retrieve_deserialized<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, JsValue> {
        if let Some(serialized) = self.get(key)? {
            let deserialized: T = serde_json::from_str(&serialized)
                .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &str) -> Result<bool, JsValue> {
        Ok(self.get(key)?.is_some())
    }

    /// Get storage type
    pub fn storage_type(&self) -> &'static str {
        match self {
            Self::Local(_) => "local",
            Self::Session(_) => "session",
            Self::Memory(_) => "memory",
        }
    }

    /// Get storage quota (approximate)
    pub fn get_quota(&self) -> Result<Option<usize>, JsValue> {
        match self {
            Self::Local(_) | Self::Session(_) => {
                // Browser storage quota is not directly accessible
                // This is a placeholder for future implementation
                Ok(None)
            }
            Self::Memory(_) => {
                // Memory storage has no quota
                Ok(None)
            }
        }
    }

    /// Get storage usage (approximate)
    pub fn get_usage(&self) -> Result<usize, JsValue> {
        let mut usage = 0;
        for key in self.keys()? {
            if let Some(value) = self.get(&key)? {
                usage += key.len() + value.len();
            }
        }
        Ok(usage)
    }
}

/// Storage manager for metadata
#[derive(Debug)]
pub struct MetadataStorage {
    storage: WasmStorage,
    prefix: String,
}

impl MetadataStorage {
    /// Create a new metadata storage manager
    pub fn new(storage: WasmStorage) -> Self {
        Self {
            storage,
            prefix: "metadata_".to_string(),
        }
    }

    /// Create with custom prefix
    pub fn with_prefix(storage: WasmStorage, prefix: String) -> Self {
        Self { storage, prefix }
    }

    /// Get prefixed key
    fn prefixed_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }

    /// Store metadata
    pub fn store_metadata<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), JsValue> {
        let prefixed_key = self.prefixed_key(key);
        self.storage.store_serialized(&prefixed_key, value)
    }

    /// Retrieve metadata
    pub fn retrieve_metadata<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, JsValue> {
        let prefixed_key = self.prefixed_key(key);
        self.storage.retrieve_deserialized(&prefixed_key)
    }

    /// Remove metadata
    pub fn remove_metadata(&mut self, key: &str) -> Result<(), JsValue> {
        let prefixed_key = self.prefixed_key(key);
        self.storage.remove(&prefixed_key)
    }

    /// Clear all metadata
    pub fn clear_metadata(&mut self) -> Result<(), JsValue> {
        let keys = self.storage.keys()?;
        for key in keys {
            if key.starts_with(&self.prefix) {
                self.storage.remove(&key)?;
            }
        }
        Ok(())
    }

    /// List all metadata keys
    pub fn list_metadata_keys(&self) -> Result<Vec<String>, JsValue> {
        let keys = self.storage.keys()?;
        let metadata_keys: Vec<String> = keys
            .into_iter()
            .filter(|key| key.starts_with(&self.prefix))
            .map(|key| key.trim_start_matches(&self.prefix).to_string())
            .collect();
        Ok(metadata_keys)
    }

    /// Get storage backend
    pub fn storage_type(&self) -> &'static str {
        self.storage.storage_type()
    }

    /// Get storage usage
    pub fn get_usage(&self) -> Result<usize, JsValue> {
        self.storage.get_usage()
    }
}

/// Storage factory for creating storage backends
pub struct StorageFactory;

impl StorageFactory {
    /// Create local storage
    pub fn local() -> Result<WasmStorage, JsValue> {
        WasmStorage::local()
    }

    /// Create session storage
    pub fn session() -> Result<WasmStorage, JsValue> {
        WasmStorage::session()
    }

    /// Create memory storage
    pub fn memory() -> WasmStorage {
        WasmStorage::memory()
    }

    /// Create best available storage
    pub fn best_available() -> WasmStorage {
        // Try local storage first, then session, then memory
        if let Ok(storage) = Self::local() {
            storage
        } else if let Ok(storage) = Self::session() {
            storage
        } else {
            Self::memory()
        }
    }

    /// Create metadata storage with best available backend
    pub fn metadata_storage() -> Result<MetadataStorage, JsValue> {
        let storage = Self::best_available();
        Ok(MetadataStorage::new(storage))
    }

    /// Create metadata storage with specific backend
    pub fn metadata_storage_with_backend(backend: &str) -> Result<MetadataStorage, JsValue> {
        let storage = match backend {
            "local" => Self::local()?,
            "session" => Self::session()?,
            "memory" => Self::memory(),
            _ => return Err(JsValue::from_str("Invalid storage backend")),
        };
        Ok(MetadataStorage::new(storage))
    }
}

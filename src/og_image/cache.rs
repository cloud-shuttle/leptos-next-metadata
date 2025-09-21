//! Caching system for OG image generation
//!
//! This module provides a flexible caching system to improve performance
//! by avoiding regenerating identical images.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

// Conditional compilation for WASM compatibility
#[cfg(target_arch = "wasm32")]
use parking_lot::RwLock as AsyncRwLock;

#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::RwLock as AsyncRwLock;

use crate::og_image::types::*;
use crate::Result;

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// The generated image data
    pub data: Vec<u8>,
    /// When this entry was created
    pub created_at: Instant,
    /// How many times this entry has been accessed
    pub access_count: u64,
    /// Last time this entry was accessed
    pub last_accessed: Instant,
}

impl CacheEntry {
    pub fn new(data: Vec<u8>) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            access_count: 1,
            last_accessed: now,
        }
    }

    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub fn time_since_last_access(&self) -> Duration {
        self.last_accessed.elapsed()
    }
}

/// Cache key for OG image parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheKey {
    /// Template name
    pub template: String,
    /// Serialized template data
    pub data_hash: u64,
    /// Image dimensions
    pub size: (u32, u32),
    /// Image format
    pub format: String,
}

impl CacheKey {
    pub fn new(params: &OgImageParams) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        // Hash the template data by serializing to string
        for (key, value) in &params.data {
            key.hash(&mut hasher);
            // Serialize the value to string for hashing since liquid::Value doesn't implement Hash
            let value_str = serde_json::to_string(value).unwrap_or_default();
            value_str.hash(&mut hasher);
        }

        Self {
            template: params.template.clone(),
            data_hash: hasher.finish(),
            size: params.size.unwrap_or((1200, 630)),
            format: format!("{:?}", params.format),
        }
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.template.hash(state);
        self.data_hash.hash(state);
        self.size.hash(state);
        self.format.hash(state);
    }
}

/// Cache provider trait for different caching backends
#[async_trait::async_trait]
pub trait CacheProvider: Send + Sync {
    /// Get a cached image by key
    async fn get(&self, key: &CacheKey) -> Result<Option<Vec<u8>>>;

    /// Store an image in the cache
    async fn set(&self, key: &CacheKey, data: &[u8]) -> Result<()>;

    /// Remove an entry from the cache
    async fn remove(&self, key: &CacheKey) -> Result<()>;

    /// Clear all entries from the cache
    async fn clear(&self) -> Result<()>;

    /// Get cache statistics
    async fn stats(&self) -> Result<CacheStats>;
}

/// In-memory cache implementation
pub struct MemoryCache {
    /// The actual cache storage
    cache: AsyncRwLock<HashMap<CacheKey, CacheEntry>>,
    /// Maximum number of entries
    max_entries: usize,
    /// Maximum age for entries
    max_age: Duration,
    /// Maximum time since last access
    max_idle_time: Duration,
}

impl MemoryCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: AsyncRwLock::new(HashMap::new()),
            max_entries,
            max_age: Duration::from_secs(3600),       // 1 hour
            max_idle_time: Duration::from_secs(1800), // 30 minutes
        }
    }

    pub fn with_ttl(max_entries: usize, max_age: Duration, max_idle_time: Duration) -> Self {
        Self {
            cache: AsyncRwLock::new(HashMap::new()),
            max_entries,
            max_age,
            max_idle_time,
        }
    }

    /// Clean up expired entries
    async fn cleanup(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        let now = Instant::now();

        cache.retain(|_, entry| {
            let age = now.duration_since(entry.created_at);
            let idle_time = now.duration_since(entry.last_accessed);

            age < self.max_age && idle_time < self.max_idle_time
        });

        Ok(())
    }

    /// Evict least recently used entries if cache is full
    async fn evict_lru(&self) -> Result<()> {
        let mut cache = self.cache.write().await;

        if cache.len() < self.max_entries {
            return Ok(());
        }

        // Find the entry with the oldest last_accessed time
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for (key, entry) in cache.iter() {
            if entry.last_accessed < oldest_time {
                oldest_time = entry.last_accessed;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            cache.remove(&key);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl CacheProvider for MemoryCache {
    async fn get(&self, key: &CacheKey) -> Result<Option<Vec<u8>>> {
        let mut cache = self.cache.write().await;

        if let Some(entry) = cache.get_mut(key) {
            // Check if entry is still valid
            let age = entry.age();
            let idle_time = entry.time_since_last_access();

            if age < self.max_age && idle_time < self.max_idle_time {
                entry.access();
                return Ok(Some(entry.data.clone()));
            } else {
                // Entry is expired, remove it
                cache.remove(key);
            }
        }

        Ok(None)
    }

    async fn set(&self, key: &CacheKey, data: &[u8]) -> Result<()> {
        // Clean up expired entries first
        self.cleanup().await?;

        // Evict LRU entries if needed
        self.evict_lru().await?;

        let mut cache = self.cache.write().await;
        cache.insert(key.clone(), CacheEntry::new(data.to_vec()));

        Ok(())
    }

    async fn remove(&self, key: &CacheKey) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    async fn stats(&self) -> Result<CacheStats> {
        let cache = self.cache.read().await;

        let mut total_accesses = 0;
        let mut total_age = Duration::ZERO;
        let mut oldest_entry = Instant::now();

        for entry in cache.values() {
            total_accesses += entry.access_count;
            total_age += entry.age();
            if entry.created_at < oldest_entry {
                oldest_entry = entry.created_at;
            }
        }

        Ok(CacheStats {
            entries: cache.len(),
            max_entries: self.max_entries,
            total_accesses,
            average_age: if cache.is_empty() {
                Duration::ZERO
            } else {
                total_age / cache.len() as u32
            },
            oldest_entry_age: oldest_entry.elapsed(),
            hit_rate: 0.0, // This would need to be tracked separately
        })
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries in cache
    pub entries: usize,
    /// Maximum number of entries allowed
    pub max_entries: usize,
    /// Total number of cache accesses
    pub total_accesses: u64,
    /// Average age of entries
    pub average_age: Duration,
    /// Age of the oldest entry
    pub oldest_entry_age: Duration,
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
}

/// No-op cache implementation for testing or when caching is disabled
pub struct NoOpCache;

#[async_trait::async_trait]
impl CacheProvider for NoOpCache {
    async fn get(&self, _key: &CacheKey) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    async fn set(&self, _key: &CacheKey, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    async fn remove(&self, _key: &CacheKey) -> Result<()> {
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        Ok(())
    }

    async fn stats(&self) -> Result<CacheStats> {
        Ok(CacheStats {
            entries: 0,
            max_entries: 0,
            total_accesses: 0,
            average_age: Duration::ZERO,
            oldest_entry_age: Duration::ZERO,
            hit_rate: 0.0,
        })
    }
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries
    pub max_entries: usize,
    /// Maximum age for entries
    pub max_age: Duration,
    /// Maximum idle time before eviction
    pub max_idle_time: Duration,
    /// Whether to enable caching
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 100,
            max_age: Duration::from_secs(3600),       // 1 hour
            max_idle_time: Duration::from_secs(1800), // 30 minutes
            enabled: true,
        }
    }
}

impl CacheConfig {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            ..Default::default()
        }
    }

    pub fn with_ttl(max_entries: usize, max_age: Duration, max_idle_time: Duration) -> Self {
        Self {
            max_entries,
            max_age,
            max_idle_time,
            enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

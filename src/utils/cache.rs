//! Cache utilities

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Generate cache key from multiple components
pub fn generate_key(components: &[&str]) -> String {
    let combined = components.join("|");
    let mut hasher = DefaultHasher::new();
    combined.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Generate cache key for metadata
pub fn metadata_cache_key(
    path: &str,
    params: Option<&HashMap<String, String>>,
    user_agent: Option<&str>,
) -> String {
    let mut components = vec![path];
    
    let params_str;
    if let Some(params) = params {
        params_str = serde_json::to_string(params).unwrap_or_default();
        components.push(&params_str);
    }
    
    if let Some(ua) = user_agent {
        components.push(ua);
    }
    
    generate_key(&components)
}

/// Generate cache key for OG images
pub fn og_image_cache_key(template: &str, params: &HashMap<String, String>) -> String {
    let params_str = serde_json::to_string(params).unwrap_or_default();
    generate_key(&[template, &params_str])
}

/// TTL-aware cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub created_at: std::time::Instant,
    pub ttl: std::time::Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T, ttl: std::time::Duration) -> Self {
        Self {
            value,
            created_at: std::time::Instant::now(),
            ttl,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}
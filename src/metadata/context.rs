//! Leptos context integration for metadata management
//! 
//! This module provides context management for sharing metadata across the component tree.

use crate::{Metadata, MetadataConfig};
use std::sync::Arc;
use parking_lot::RwLock;

/// Metadata context that holds the current metadata state
#[derive(Clone)]
pub struct MetadataContext {
    /// Current metadata configuration
    pub config: Arc<MetadataConfig>,
    
    /// Current metadata stack for inheritance
    pub metadata_stack: Arc<RwLock<Vec<Metadata>>>,
    
    /// Parent metadata context (for nested contexts)
    pub parent: Option<Arc<MetadataContext>>,
}

impl MetadataContext {
    /// Create a new metadata context with default configuration
    pub fn new() -> Self {
        Self {
            config: Arc::new(MetadataConfig::default()),
            metadata_stack: Arc::new(RwLock::new(Vec::new())),
            parent: None,
        }
    }
    
    /// Create a new metadata context with custom configuration
    pub fn with_config(config: MetadataConfig) -> Self {
        Self {
            config: Arc::new(config),
            metadata_stack: Arc::new(RwLock::new(Vec::new())),
            parent: None,
        }
    }
    
    /// Create a child context that inherits from a parent
    pub fn with_parent(parent: Arc<MetadataContext>) -> Self {
        Self {
            config: parent.config.clone(),
            metadata_stack: Arc::new(RwLock::new(Vec::new())),
            parent: Some(parent),
        }
    }
    
    /// Add metadata to the current context
    pub fn push_metadata(&self, metadata: Metadata) {
        let mut stack = self.metadata_stack.write();
        stack.push(metadata);
    }
    
    /// Remove the most recent metadata from the current context
    pub fn pop_metadata(&self) -> Option<Metadata> {
        let mut stack = self.metadata_stack.write();
        stack.pop()
    }
    
    /// Get the merged metadata from this context and all parent contexts
    pub fn get_merged_metadata(&self) -> Metadata {
        let mut result = Metadata::default();
        
        // Start with parent metadata if it exists
        if let Some(ref parent) = self.parent {
            result = crate::metadata::merge::merge_metadata(result, parent.get_merged_metadata());
        }
        
        // Apply metadata from this context's stack
        let stack = self.metadata_stack.read();
        for metadata in stack.iter() {
            result = crate::metadata::merge::merge_metadata(result, metadata.clone());
        }
        
        result
    }
    
    /// Update the configuration for this context
    pub fn update_config(&mut self, config: MetadataConfig) {
        self.config = Arc::new(config);
    }
}

impl Default for MetadataContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified metadata provider function (without component integration for now)
pub fn provide_metadata_context() -> MetadataContext {
    MetadataContext::new()
}

/// Simple provider component - placeholder for future Leptos integration
pub struct MetadataProvider {
    context: MetadataContext,
}

impl MetadataProvider {
    pub fn new() -> Self {
        Self {
            context: MetadataContext::new(),
        }
    }
    
    pub fn with_config(config: MetadataConfig) -> Self {
        Self {
            context: MetadataContext::with_config(config),
        }
    }
    
    pub fn get_context(&self) -> &MetadataContext {
        &self.context
    }
}
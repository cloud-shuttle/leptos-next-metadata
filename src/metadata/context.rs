//! Metadata context management for leptos-next-metadata
//! 
//! This module provides the context system that allows metadata to be
//! shared and inherited across component hierarchies.

use crate::{Metadata, MetadataConfig, Result};
use leptos::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Metadata context that holds the current metadata state
#[derive(Clone)]
pub struct MetadataContext {
    /// Current metadata configuration
    pub config: Arc<MetadataConfig>,
    
    /// Current metadata stack
    pub metadata_stack: Arc<RwLock<Vec<Metadata>>>,
    
    /// Parent metadata context (for inheritance)
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
    
    /// Create a child context that inherits from this one
    pub fn create_child(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            metadata_stack: Arc::new(RwLock::new(Vec::new())),
            parent: Some(Arc::new(self.clone())),
        }
    }
    
    /// Push metadata to the current context
    pub fn push_metadata(&self, metadata: Metadata) {
        let mut stack = self.metadata_stack.write();
        stack.push(metadata);
    }
    
    /// Pop metadata from the current context
    pub fn pop_metadata(&self) -> Option<Metadata> {
        let mut stack = self.metadata_stack.write();
        stack.pop()
    }
    
    /// Get the current metadata by merging all levels
    pub fn get_current_metadata(&self) -> Result<Metadata> {
        let stack = self.metadata_stack.read();
        let mut current = Metadata::default();
        
        // Merge all metadata in the stack
        for metadata in stack.iter() {
            current = current.merge(metadata)?;
        }
        
        // Inherit from parent if available
        if let Some(ref parent) = self.parent {
            let parent_meta = parent.get_current_metadata()?;
            current = current.merge(&parent_meta)?;
        }
        
        Ok(current)
    }
    
    /// Get the metadata configuration
    pub fn get_config(&self) -> &MetadataConfig {
        &self.config
    }
    
    /// Update the metadata configuration
    pub fn update_config(&self, config: MetadataConfig) {
        let mut current_config = Arc::get_mut(&mut self.config.clone())
            .expect("Failed to get mutable reference to config");
        *current_config = config;
    }
}

/// Provider component that wraps the application and provides metadata context
#[component]
pub fn MetadataProvider(
    #[prop(into)] children: Children,
    #[prop(optional)] config: Option<MetadataConfig>,
) -> impl IntoView {
    let context = if let Some(config) = config {
        MetadataContext::with_config(config)
    } else {
        MetadataContext::new()
    };
    
    provide_context(Arc::new(context));
    
    children()
}

/// Hook to get the current metadata context
pub fn use_metadata_context() -> Result<Arc<MetadataContext>> {
    use_context::<Arc<MetadataContext>>()
        .ok_or(crate::Error::ContextNotProvided)
}

/// Hook to get the current metadata
pub fn use_metadata() -> Result<Metadata> {
    let context = use_metadata_context()?;
    context.get_current_metadata()
}

/// Hook to get the metadata configuration
pub fn use_metadata_config() -> Result<Arc<MetadataConfig>> {
    let context = use_metadata_context()?;
    Ok(Arc::clone(&context.config))
}

/// Function to provide metadata context at the root level
/// 
/// This should be called in your root App component to set up
/// the metadata system for the entire application.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use leptos_next_metadata::prelude::*;
/// 
/// #[component]
/// pub fn App() -> impl IntoView {
///     // Provide metadata context for the entire app
///     provide_metadata_context();
///     
///     view! {
///         <MetadataProvider>
///             <Router>
///                 <Routes>
///                     // Your routes
///                 </Routes>
///             </Router>
///         </MetadataProvider>
///     }
/// }
/// ```
pub fn provide_metadata_context() {
    let context = MetadataContext::new();
    provide_context(Arc::new(context));
}

/// Function to provide metadata context with custom configuration
pub fn provide_metadata_context_with_config(config: MetadataConfig) {
    let context = MetadataContext::with_config(config);
    provide_context(Arc::new(context));
}

/// Hook to set metadata for the current component
/// 
/// This hook allows you to set metadata that will be merged with
/// parent metadata and inherited by child components.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use leptos_next_metadata::prelude::*;
/// 
/// #[component]
/// fn MyPage() -> impl IntoView {
///     let set_metadata = use_set_metadata();
///     
///     // Set metadata for this component
///     set_metadata(Metadata::with_title("My Page"));
///     
///     view! { <h1>"My Page"</h1> }
/// }
/// ```
pub fn use_set_metadata() -> Result<impl Fn(Metadata)> {
    let context = use_metadata_context()?;
    
    Ok(move |metadata: Metadata| {
        context.push_metadata(metadata);
    })
}

/// Hook to get and set metadata for the current component
/// 
/// This hook provides both the current metadata and a function to update it.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use leptos_next_metadata::prelude::*;
/// 
/// #[component]
/// fn MyPage() -> impl IntoView {
///     let (metadata, set_metadata) = use_metadata_state();
///     
///     // Update metadata based on some condition
///     if let Ok(meta) = metadata {
///         if meta.title.is_none() {
///             set_metadata(Metadata::with_title("Default Title"));
///         }
///     }
///     
///     view! { <h1>"My Page"</h1> }
/// }
/// ```
pub fn use_metadata_state() -> (Result<Metadata>, impl Fn(Metadata)) {
    let context = use_metadata_context().expect("Metadata context not provided");
    let metadata = context.get_current_metadata();
    let set_metadata = move |new_metadata: Metadata| {
        context.push_metadata(new_metadata);
    };
    
    (metadata, set_metadata)
}

/// Hook to create a metadata scope for a component
/// 
/// This hook creates a new metadata scope that will be automatically
/// cleaned up when the component unmounts.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use leptos_next_metadata::prelude::*;
/// 
/// #[component]
/// fn MyPage() -> impl IntoView {
///     let _scope = use_metadata_scope();
///     
///     // Set metadata for this scope
///     let set_metadata = use_set_metadata().unwrap();
///     set_metadata(Metadata::with_title("My Page"));
///     
///     view! { <h1>"My Page"</h1> }
/// }
/// ```
pub fn use_metadata_scope() -> Result<MetadataScope> {
    let context = use_metadata_context()?;
    let scope = MetadataScope::new(Arc::clone(&context));
    
    // Clean up when component unmounts
    on_cleanup(move || {
        // The scope will be dropped here, cleaning up metadata
    });
    
    Ok(scope)
}

/// Metadata scope that automatically manages metadata lifecycle
pub struct MetadataScope {
    context: Arc<MetadataContext>,
    metadata_count: usize,
}

impl MetadataScope {
    /// Create a new metadata scope
    fn new(context: Arc<MetadataContext>) -> Self {
        Self {
            context,
            metadata_count: 0,
        }
    }
    
    /// Add metadata to this scope
    pub fn add_metadata(&mut self, metadata: Metadata) -> Result<()> {
        self.context.push_metadata(metadata);
        self.metadata_count += 1;
        Ok(())
    }
    
    /// Get the current metadata for this scope
    pub fn get_metadata(&self) -> Result<Metadata> {
        self.context.get_current_metadata()
    }
}

impl Drop for MetadataScope {
    fn drop(&mut self) {
        // Remove all metadata that was added by this scope
        for _ in 0..self.metadata_count {
            self.context.pop_metadata();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Metadata;
    
    #[test]
    fn test_metadata_context_creation() {
        let context = MetadataContext::new();
        assert!(context.parent.is_none());
        assert!(context.metadata_stack.read().is_empty());
    }
    
    #[test]
    fn test_metadata_context_with_config() {
        let config = MetadataConfig::default();
        let context = MetadataContext::with_config(config);
        assert!(context.parent.is_none());
    }
    
    #[test]
    fn test_metadata_context_child() {
        let parent = MetadataContext::new();
        let child = parent.create_child();
        assert!(child.parent.is_some());
    }
    
    #[test]
    fn test_metadata_push_pop() {
        let context = MetadataContext::new();
        let metadata = Metadata::with_title("Test");
        
        context.push_metadata(metadata.clone());
        assert_eq!(context.metadata_stack.read().len(), 1);
        
        let popped = context.pop_metadata();
        assert!(popped.is_some());
        assert!(context.metadata_stack.read().is_empty());
    }
    
    #[test]
    fn test_metadata_inheritance() {
        let parent = MetadataContext::new();
        let parent_meta = Metadata::with_title("Parent");
        parent.push_metadata(parent_meta);
        
        let child = parent.create_child();
        let child_meta = Metadata::with_title("Child");
        child.push_metadata(child_meta);
        
        let final_meta = child.get_current_metadata().unwrap();
        // Child metadata should override parent metadata
        assert_eq!(final_meta.title.unwrap().to_string(), "Child");
    }
}

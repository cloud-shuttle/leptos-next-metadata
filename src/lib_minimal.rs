//! Minimal leptos-next-metadata implementation
//! 
//! This provides basic metadata types without complex features

use leptos::*;
use serde::{Deserialize, Serialize};

/// Basic metadata structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_title<T: Into<String>>(title: T) -> Self {
        Self {
            title: Some(title.into()),
            ..Default::default()
        }
    }
    
    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn description<T: Into<String>>(mut self, desc: T) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Metadata provider component
#[component]
pub fn MetadataProvider(children: Children) -> impl IntoView {
    view! {
        {children()}
    }
}

/// Simple macro for setting metadata
#[macro_export]
macro_rules! metadata {
    (title: $title:expr) => {
        leptos_meta::Title::new($title);
    };
    (description: $desc:expr) => {
        leptos_meta::Meta::new("description", $desc);
    };
}

pub mod prelude {
    pub use super::*;
    pub use leptos_meta::*;
}
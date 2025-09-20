# Metadata Core Component Design

## Overview
Design specification for the core metadata management component after refactoring.

## Current Problems
- Single 1,304-line file with mixed concerns
- Complex type hierarchy difficult to test
- Builder pattern scattered across implementations
- Validation logic intertwined with data structures

## Design Goals
- **Separation of Concerns**: Clear boundaries between types, validation, and serialization
- **Testability**: Each component independently testable
- **Performance**: Zero-cost abstractions where possible
- **Maintainability**: <300 lines per file, focused responsibilities

## Architecture

### 1. Core Types (`types.rs`)
**Responsibility**: Pure data structures and core enums
**Line Target**: 280 lines

```rust
//! Core metadata types and data structures

use serde::{Deserialize, Serialize};

/// Primary metadata container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<Title>,
    pub description: Option<String>,
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<Twitter>,
    pub robots: Option<Robots>,
    pub alternate_links: Vec<AlternateLink>,
    pub viewport: Option<Viewport>,
}

/// Title with template support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Title {
    Static(String),
    Template { template: String, default: String },
}

/// OpenGraph metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<OgImage>,
    pub url: Option<String>,
    pub site_name: Option<String>,
    pub article: Option<Article>,
}

// Additional types follow...
```

**Key Design Decisions**:
- No business logic in data structures
- Serde derives for serialization
- Optional fields use `Option<T>` consistently
- Enums for structured choices (Title variants)

### 2. Builder Pattern (`builder.rs`)
**Responsibility**: Fluent API for metadata construction
**Line Target**: 250 lines

```rust
//! Builder pattern implementation for metadata construction

use crate::metadata::types::*;

/// Fluent builder for Metadata
pub struct MetadataBuilder {
    inner: Metadata,
}

impl MetadataBuilder {
    pub fn new() -> Self {
        Self {
            inner: Metadata::default(),
        }
    }
    
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.inner.title = Some(title.into());
        self
    }
    
    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.inner.description = Some(desc.into());
        self
    }
    
    pub fn open_graph(mut self, og: OpenGraph) -> Self {
        self.inner.open_graph = Some(og);
        self
    }
    
    pub fn build(self) -> Metadata {
        self.inner
    }
}

/// Builder for OpenGraph metadata
pub struct OpenGraphBuilder {
    inner: OpenGraph,
}

impl OpenGraphBuilder {
    // Similar pattern for OpenGraph construction
}
```

**Key Features**:
- Fluent API with method chaining
- Type conversions via `Into` trait
- Separate builders for complex nested types
- Validation deferred to build step

### 3. Display Implementation (`display.rs`)  
**Responsibility**: String formatting and HTML generation
**Line Target**: 200 lines

```rust
//! Display and formatting implementations

use std::fmt;
use crate::metadata::types::*;

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            writeln!(f, "<title>{}</title>", title)?;
        }
        
        if let Some(description) = &self.description {
            writeln!(f, r#"<meta name="description" content="{}" />"#, description)?;
        }
        
        if let Some(og) = &self.open_graph {
            write!(f, "{}", og)?;
        }
        
        Ok(())
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Title::Static(s) => write!(f, "{}", s),
            Title::Template { template, default } => {
                // Template rendering logic
                write!(f, "{}", default)
            }
        }
    }
}
```

### 4. Serialization (`serde_impl.rs`)
**Responsibility**: Custom serialization logic
**Line Target**: 150 lines

```rust
//! Custom serialization implementations

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::metadata::types::*;

// Custom serialization for Title enum to support template syntax
impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Title::Static(s) => s.serialize(serializer),
            Title::Template { template, .. } => template.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Title, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains("{{") || s.contains("%{") {
            Ok(Title::Template {
                template: s.clone(),
                default: s,
            })
        } else {
            Ok(Title::Static(s))
        }
    }
}
```

### 5. Module Interface (`mod.rs`)
**Responsibility**: Clean public API and re-exports  
**Line Target**: 50 lines

```rust
//! Metadata management module

mod types;
mod builder;
mod display;
mod serde_impl;

pub use types::*;
pub use builder::{MetadataBuilder, OpenGraphBuilder};

// Convenience functions
pub fn metadata() -> MetadataBuilder {
    MetadataBuilder::new()
}

pub fn merge_metadata(base: Metadata, override_meta: Metadata) -> Metadata {
    // Merge logic moved to separate merge module
    crate::metadata::merge::merge_metadata_impl(base, override_meta)
}

// Re-export validation
pub use crate::metadata::validation::{validate_metadata, ValidationResult};
```

## Testing Strategy

### 1. Unit Tests per Module
```
tests/
├── types_test.rs          (100 lines - data structure tests)
├── builder_test.rs        (100 lines - builder pattern tests)  
├── display_test.rs        (80 lines - formatting tests)
├── serde_test.rs          (120 lines - serialization roundtrips)
└── integration_test.rs    (150 lines - cross-module tests)
```

### 2. Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn metadata_serde_roundtrip(title in any::<String>()) {
        let original = Metadata {
            title: Some(Title::Static(title)),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Metadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original, deserialized);
    }
}
```

## Migration Plan

### Phase 1: Extract Types (2 days)
1. Create `src/metadata/types.rs`
2. Move all struct/enum definitions
3. Add necessary imports
4. Update existing tests

### Phase 2: Extract Builders (2 days)  
1. Create `src/metadata/builder.rs`
2. Move all `impl` blocks for builders
3. Add builder-specific tests
4. Verify API compatibility

### Phase 3: Extract Display/Serde (2 days)
1. Create display and serde modules
2. Move formatting logic
3. Add serialization tests
4. Benchmark performance impact

### Phase 4: Integration & Validation (2 days)
1. Update mod.rs with clean exports
2. Run full test suite
3. Performance benchmarking
4. Documentation updates

## Success Metrics
- [ ] All files under 300 lines
- [ ] Zero functionality regression  
- [ ] Test coverage maintains >90%
- [ ] Build time impact <5%
- [ ] API remains backward compatible
- [ ] Clear module responsibilities

## Performance Considerations
- **Zero-cost abstractions**: Builder pattern compiles away
- **Minimal allocations**: Reuse strings where possible
- **Lazy evaluation**: Template rendering only when needed
- **Efficient serialization**: Custom implementations avoid copies

## Future Enhancements
- Async template rendering support
- Plugin system for custom metadata types
- Validation rule composition
- Metadata diffing and merging improvements

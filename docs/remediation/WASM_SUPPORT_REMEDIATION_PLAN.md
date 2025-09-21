# WASM Support Remediation Plan

**Date**: January 2025  
**Status**: 📋 PLANNING  
**Priority**: High  
**Timeline**: 8-12 weeks  
**Effort**: 3-4 developer weeks

## Executive Summary

This remediation plan outlines the systematic approach to implement WebAssembly (WASM) support for the leptos-next-metadata library. The plan follows a phased approach with selective WASM support, ensuring backward compatibility while enabling client-side metadata management capabilities.

## Remediation Strategy

### **🎯 Selected Approach: Selective WASM Support**

**Rationale:**
- ✅ Preserves existing server-side capabilities
- ✅ Enables new client-side use cases
- ✅ Moderate implementation effort
- ✅ Clear feature boundaries
- ✅ Low risk of breaking changes

## Phase 1: Foundation & Core Support (Weeks 1-3)

### **1.1 Dependency Remediation**

#### **Task 1.1.1: Replace Tokio with WASM-Compatible Alternatives**
**Priority**: P0 (Critical)  
**Effort**: 2 days  
**Owner**: Backend Developer

**Current State:**
```rust
// src/og_image/cache.rs:9
use tokio::sync::RwLock as AsyncRwLock;
```

**Target State:**
```rust
// Conditional compilation for WASM compatibility
#[cfg(target_arch = "wasm32")]
use parking_lot::RwLock as AsyncRwLock;

#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::RwLock as AsyncRwLock;
```

**Implementation Steps:**
1. [ ] Add `parking_lot` as optional dependency
2. [ ] Update cache.rs with conditional compilation
3. [ ] Test both native and WASM builds
4. [ ] Update documentation

**Acceptance Criteria:**
- [ ] WASM build compiles successfully
- [ ] Native build maintains performance
- [ ] No breaking changes to public API
- [ ] All existing tests pass

#### **Task 1.1.2: Feature Flag Restructuring**
**Priority**: P0 (Critical)  
**Effort**: 1 day  
**Owner**: Backend Developer

**Current Features:**
```toml
default = ["ssr", "og-images", "file-conventions", "macros", "json-ld"]
```

**New Feature Structure:**
```toml
# Core features (WASM compatible)
default = ["ssr", "macros", "json-ld"]
csr = ["leptos/csr"]
hydrate = ["leptos/hydrate"]

# Server-only features
ssr = ["leptos/ssr"]
og-images = ["image", "resvg", "usvg", "tiny-skia", "fontdue", "liquid"]
file-conventions = ["walkdir", "mime_guess", "image"]
http = ["reqwest"]
caching = ["cached", "lru"]

# WASM-specific features
wasm = ["csr", "json-ld", "macros", "basic-caching"]
wasm-advanced = ["wasm", "canvas-og-images", "web-storage"]
```

**Implementation Steps:**
1. [ ] Restructure Cargo.toml features
2. [ ] Update feature documentation
3. [ ] Create WASM-specific feature sets
4. [ ] Update CI/CD for WASM builds

### **1.2 Module Architecture Updates**

#### **Task 1.2.1: Conditional Module Compilation**
**Priority**: P0 (Critical)  
**Effort**: 2 days  
**Owner**: Backend Developer

**Modules to Feature-Gate:**
```rust
// Server-only modules
#[cfg(not(target_arch = "wasm32"))]
pub mod og_image;

#[cfg(not(target_arch = "wasm32"))]
pub mod conventions;

#[cfg(not(target_arch = "wasm32"))]
pub mod api;

// WASM-compatible modules
pub mod metadata;
pub mod json_ld;
pub mod macros;
pub mod utils;
```

**Implementation Steps:**
1. [ ] Add conditional compilation to module declarations
2. [ ] Create WASM-compatible module exports
3. [ ] Update prelude for WASM compatibility
4. [ ] Test module loading in both environments

#### **Task 1.2.2: WASM-Compatible Prelude**
**Priority**: P1 (High)  
**Effort**: 1 day  
**Owner**: Backend Developer

**New Prelude Structure:**
```rust
// src/lib.rs
#[cfg(target_arch = "wasm32")]
pub mod wasm_prelude {
    pub use crate::metadata::*;
    pub use crate::json_ld::*;
    pub use crate::macros::*;
    pub use crate::utils::*;
    
    // WASM-specific re-exports
    pub use crate::wasm::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod prelude {
    pub use crate::metadata::*;
    pub use crate::json_ld::*;
    pub use crate::macros::*;
    pub use crate::utils::*;
    pub use crate::og_image::*;
    pub use crate::conventions::*;
    pub use crate::api::*;
}
```

### **1.3 Build System Updates**

#### **Task 1.3.1: WASM Build Configuration**
**Priority**: P0 (Critical)  
**Effort**: 1 day  
**Owner**: DevOps Engineer

**CI/CD Updates:**
```yaml
# .github/workflows/wasm.yml
name: WASM Build
on: [push, pull_request]
jobs:
  wasm-build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: cargo install wasm-pack
      - name: Build WASM
        run: cargo build --target wasm32-unknown-unknown --features wasm
      - name: Test WASM
        run: wasm-pack test --node
```

**Makefile Updates:**
```makefile
# Makefile
.PHONY: build-wasm test-wasm
build-wasm:
	cargo build --target wasm32-unknown-unknown --features wasm

test-wasm:
	wasm-pack test --node

check-wasm:
	cargo check --target wasm32-unknown-unknown --features wasm
```

## Phase 2: WASM-Specific Implementations (Weeks 4-6)

### **2.1 Client-Side Metadata Management**

#### **Task 2.1.1: WASM Metadata Context**
**Priority**: P1 (High)  
**Effort**: 3 days  
**Owner**: Frontend Developer

**Implementation:**
```rust
// src/wasm/metadata_context.rs
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub struct WasmMetadataContext {
    metadata: Metadata,
    storage: Option<web_sys::Storage>,
}

#[wasm_bindgen]
impl WasmMetadataContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten());
            
        Self {
            metadata: Metadata::default(),
            storage,
        }
    }
    
    #[wasm_bindgen]
    pub fn set_metadata(&mut self, metadata: &Metadata) -> Result<(), JsValue> {
        self.metadata = metadata.clone();
        self.persist_metadata()?;
        Ok(())
    }
    
    #[wasm_bindgen]
    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    
    fn persist_metadata(&self) -> Result<(), JsValue> {
        if let Some(storage) = &self.storage {
            let serialized = serde_json::to_string(&self.metadata)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            storage.set_item("leptos_metadata", &serialized)?;
        }
        Ok(())
    }
}
```

#### **Task 2.1.2: Browser API Integration**
**Priority**: P1 (High)  
**Effort**: 2 days  
**Owner**: Frontend Developer

**Implementation:**
```rust
// src/wasm/browser_apis.rs
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlMetaElement};

pub fn update_document_metadata(metadata: &Metadata) -> Result<(), JsValue> {
    let window = window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    
    // Update title
    if let Some(title) = &metadata.title {
        document.set_title(&title.to_string());
    }
    
    // Update meta tags
    update_meta_tag(&document, "description", &metadata.description)?;
    update_meta_tag(&document, "keywords", &metadata.keywords)?;
    
    // Update Open Graph tags
    if let Some(og) = &metadata.open_graph {
        update_meta_tag(&document, "og:title", &og.title)?;
        update_meta_tag(&document, "og:description", &og.description)?;
        update_meta_tag(&document, "og:type", &og.r#type)?;
    }
    
    Ok(())
}

fn update_meta_tag(
    document: &Document,
    name: &str,
    content: &Option<String>,
) -> Result<(), JsValue> {
    if let Some(content) = content {
        let meta = document
            .query_selector(&format!("meta[name=\"{}\"]", name))?
            .unwrap_or_else(|| {
                let meta = document.create_element("meta")?;
                meta.set_attribute("name", name)?;
                document.head()?.append_child(&meta)?;
                Ok(meta)
            })?;
        
        meta.set_attribute("content", content)?;
    }
    Ok(())
}
```

### **2.2 WASM-Specific Caching**

#### **Task 2.2.1: Web Storage Cache Implementation**
**Priority**: P2 (Medium)  
**Effort**: 2 days  
**Owner**: Frontend Developer

**Implementation:**
```rust
// src/wasm/cache.rs
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use std::collections::HashMap;

pub struct WasmCache {
    memory_cache: HashMap<String, CacheEntry>,
    storage: Option<Storage>,
    max_memory_entries: usize,
}

impl WasmCache {
    pub fn new(max_entries: usize) -> Self {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten());
            
        Self {
            memory_cache: HashMap::new(),
            storage,
            max_memory_entries: max_entries,
        }
    }
    
    pub async fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        // Check memory cache first
        if let Some(entry) = self.memory_cache.get(key) {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            }
        }
        
        // Check persistent storage
        if let Some(storage) = &self.storage {
            if let Ok(Some(data)) = storage.get_item(key) {
                if let Ok(decoded) = base64::decode(&data) {
                    return Some(decoded);
                }
            }
        }
        
        None
    }
    
    pub async fn set(&mut self, key: &str, data: &[u8]) -> Result<(), JsValue> {
        let entry = CacheEntry::new(data.to_vec());
        
        // Update memory cache
        self.memory_cache.insert(key.to_string(), entry);
        
        // Persist to storage
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
}
```

## Phase 3: Advanced Features & Optimization (Weeks 7-9)

### **3.1 Client-Side OG Image Generation**

#### **Task 3.1.1: Canvas-Based OG Image Generation**
**Priority**: P2 (Medium)  
**Effort**: 5 days  
**Owner**: Frontend Developer

**Implementation:**
```rust
// src/wasm/canvas_og_image.rs
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
pub struct CanvasOgImageGenerator {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
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
        
        Ok(CanvasOgImageGenerator { canvas, ctx })
    }
    
    #[wasm_bindgen]
    pub fn generate_og_image(&self, params: &OgImageParams) -> Result<Vec<u8>, JsValue> {
        // Set background
        self.ctx.set_fill_style(&params.background_color.into());
        self.ctx.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        
        // Add title
        if let Some(title) = &params.title {
            self.ctx.set_font(&format!("{}px {}", params.font_size, params.font_family));
            self.ctx.set_fill_style(&params.text_color.into());
            self.ctx.fill_text(title, 50.0, 100.0)?;
        }
        
        // Add description
        if let Some(description) = &params.description {
            self.ctx.set_font(&format!("{}px {}", params.font_size - 8, params.font_family));
            self.ctx.fill_text(description, 50.0, 150.0)?;
        }
        
        // Convert to image data
        let image_data = self.ctx.get_image_data(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64)?;
        Ok(image_data.data().to_vec())
    }
}
```

### **3.2 Performance Optimization**

#### **Task 3.2.1: WASM Bundle Optimization**
**Priority**: P2 (Medium)  
**Effort**: 2 days  
**Owner**: DevOps Engineer

**Optimization Strategies:**
1. **Tree Shaking**: Remove unused code
2. **Dead Code Elimination**: Strip server-only code
3. **Compression**: Gzip/Brotli compression
4. **Lazy Loading**: Load features on demand

**Configuration:**
```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = "z"  # Optimize for size
```

## Phase 4: Testing & Documentation (Weeks 10-12)

### **4.1 Comprehensive Testing**

#### **Task 4.1.1: WASM Unit Tests**
**Priority**: P0 (Critical)  
**Effort**: 3 days  
**Owner**: QA Engineer

**Test Structure:**
```rust
// tests/wasm_tests.rs
use wasm_bindgen_test::*;
use leptos_next_metadata::wasm_prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_metadata_creation() {
    let metadata = Metadata {
        title: Some(Title::Static("Test Page".into())),
        description: Some("Test description".into()),
        ..Default::default()
    };
    
    assert_eq!(metadata.title.unwrap().to_string(), "Test Page");
}

#[wasm_bindgen_test]
fn test_metadata_context() {
    let mut context = WasmMetadataContext::new();
    let metadata = Metadata {
        title: Some(Title::Static("Test".into())),
        ..Default::default()
    };
    
    context.set_metadata(&metadata).unwrap();
    let retrieved = context.get_metadata();
    
    assert_eq!(retrieved.title, metadata.title);
}
```

#### **Task 4.1.2: E2E WASM Testing**
**Priority**: P1 (High)  
**Effort**: 2 days  
**Owner**: QA Engineer

**Playwright Tests:**
```typescript
// tests/e2e/wasm-metadata.spec.ts
import { test, expect } from '@playwright/test';

test('WASM metadata management', async ({ page }) => {
  await page.goto('/wasm-example');
  
  // Test metadata updates
  await page.evaluate(() => {
    const context = new WasmMetadataContext();
    const metadata = new Metadata({
      title: 'Dynamic Title',
      description: 'Dynamic Description'
    });
    context.setMetadata(metadata);
  });
  
  // Verify DOM updates
  await expect(page).toHaveTitle('Dynamic Title');
  await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', 'Dynamic Description');
});
```

### **4.2 Documentation Updates**

#### **Task 4.2.1: WASM-Specific Documentation**
**Priority**: P1 (High)  
**Effort**: 2 days  
**Owner**: Technical Writer

**Documentation Structure:**
```
docs/
├── wasm/
│   ├── getting-started.md
│   ├── api-reference.md
│   ├── examples/
│   │   ├── basic-usage.md
│   │   ├── advanced-features.md
│   │   └── performance-optimization.md
│   └── troubleshooting.md
```

## Risk Mitigation

### **Technical Risks**

| Risk | Mitigation Strategy | Owner | Timeline |
|------|-------------------|-------|----------|
| **Bundle Size Bloat** | Feature gating, tree shaking | DevOps | Week 3 |
| **Performance Issues** | Benchmarking, optimization | Frontend | Week 6 |
| **API Breaking Changes** | Backward compatibility testing | Backend | Week 2 |
| **Browser Compatibility** | Polyfill strategy | Frontend | Week 5 |

### **Project Risks**

| Risk | Mitigation Strategy | Owner | Timeline |
|------|-------------------|-------|----------|
| **Scope Creep** | Clear phase boundaries | PM | Ongoing |
| **Resource Constraints** | Prioritized task list | PM | Ongoing |
| **Quality Issues** | Comprehensive testing | QA | Week 10-12 |
| **Documentation Gaps** | Parallel documentation | Tech Writer | Week 10-12 |

## Success Criteria

### **Phase 1 Success Criteria**
- [ ] WASM build compiles successfully
- [ ] All existing tests pass
- [ ] No breaking changes to public API
- [ ] Basic metadata management works in WASM

### **Phase 2 Success Criteria**
- [ ] Client-side metadata updates work
- [ ] Browser API integration functional
- [ ] WASM-specific caching implemented
- [ ] Performance benchmarks meet targets

### **Phase 3 Success Criteria**
- [ ] Canvas-based OG image generation works
- [ ] Bundle size <500KB
- [ ] Performance parity >90% of native
- [ ] Advanced features functional

### **Phase 4 Success Criteria**
- [ ] Test coverage >80% for WASM features
- [ ] Documentation complete
- [ ] E2E tests passing
- [ ] Production deployment ready

## Resource Requirements

### **Team Composition**
- **Backend Developer** (2 weeks): Dependency remediation, module architecture
- **Frontend Developer** (4 weeks): WASM implementations, browser APIs
- **DevOps Engineer** (1 week): Build system, CI/CD
- **QA Engineer** (2 weeks): Testing, validation
- **Technical Writer** (1 week): Documentation

### **Timeline Summary**
- **Phase 1**: 3 weeks (Foundation)
- **Phase 2**: 3 weeks (Core WASM features)
- **Phase 3**: 3 weeks (Advanced features)
- **Phase 4**: 3 weeks (Testing & docs)
- **Total**: 12 weeks

## Next Steps

1. [ ] **Week 1**: Start Phase 1 implementation
2. [ ] **Week 2**: Complete dependency remediation
3. [ ] **Week 3**: Finish foundation work
4. [ ] **Week 4**: Begin WASM-specific implementations
5. [ ] **Week 6**: Complete core WASM features
6. [ ] **Week 9**: Finish advanced features
7. [ ] **Week 12**: Complete testing and documentation

---

**Plan created by**: AI Agent  
**Status**: 📋 Ready for implementation  
**Next milestone**: Phase 1 kickoff

# WASM Support Investigation & Analysis

**Date**: January 2025  
**Status**: 🔍 INVESTIGATION COMPLETE  
**Impact**: High - Enables client-side metadata management for Leptos applications

## Executive Summary

This document details the comprehensive investigation into WebAssembly (WASM) support for the leptos-next-metadata library. The investigation reveals that while the library is currently **not WASM-compatible**, selective WASM support is **technically feasible** with moderate implementation effort.

## Current State Analysis

### 🚨 **WASM Compatibility Status: NOT COMPATIBLE**

The leptos-next-metadata library currently **cannot compile for WASM targets** due to several critical incompatibilities.

### **Primary Blockers**

#### 1. **Tokio Dependency Chain Crisis**
- **Root Cause**: `tokio = { version = "1.38", features = ["full"] }` (Cargo.toml:45)
- **Impact**: Pulls in `mio v1.0.4` which explicitly blocks WASM compilation
- **Error**: `"This wasm target is unsupported by mio. If using Tokio, disable the net feature."`
- **Current Usage**: Only used in `src/og_image/cache.rs` for `AsyncRwLock`

#### 2. **Server-Side Dependencies**
- **HTTP Stack**: `axum`, `tower`, `tower-http` in dev-dependencies
- **Networking**: `reqwest` with full HTTP client features
- **File System**: `walkdir` for file system scanning
- **Temporary Files**: `tempfile` for temporary file operations

#### 3. **Image Processing Dependencies**
- **OG Image Generation**: `image`, `resvg`, `usvg`, `tiny-skia`, `fontdue`
- **Issue**: Native libraries that don't support WASM runtime
- **Impact**: Core feature (OG image generation) unavailable in WASM

### **Dependency Analysis**

**WASM-Incompatible Dependency Chain:**
```
tokio v1.47.1
├── leptos-next-metadata v1.3.0 (direct dependency)
├── axum v0.7.9 (dev-dependency)
├── hyper v1.7.0
├── tower-http v0.5.2 (dev-dependency)
└── mio v1.0.4 (transitive dependency - PRIMARY BLOCKER)
```

**Build Verification:**
```bash
cargo check --target wasm32-unknown-unknown --no-default-features
# Result: 48 compilation errors, primarily from mio crate
```

## Feasibility Assessment

### ✅ **WASM-Compatible Components**

| Component | Status | Dependencies | Effort |
|-----------|--------|--------------|--------|
| **Core Metadata Types** | ✅ Compatible | None (pure Rust) | None |
| **JSON-LD Support** | ✅ Compatible | `serde_json` only | None |
| **Procedural Macros** | ✅ Compatible | Compile-time only | None |
| **Basic Metadata Management** | ✅ Compatible | No external deps | None |
| **Metadata Validation** | ✅ Compatible | Pure Rust logic | None |

### ❌ **WASM-Incompatible Components**

| Component | Status | Blockers | Alternative |
|-----------|--------|----------|-------------|
| **OG Image Generation** | ❌ Incompatible | Native image libs | Canvas API |
| **File Convention Scanning** | ❌ Incompatible | File system access | Browser APIs |
| **HTTP Client Features** | ❌ Incompatible | Server networking | Fetch API |
| **Advanced Caching** | ❌ Incompatible | Tokio async primitives | Web Storage |

### 🎯 **Hybrid Components (Conditional Support)**

| Component | Server | WASM | Implementation |
|-----------|--------|------|----------------|
| **Metadata Context** | ✅ Full | ✅ Basic | Feature-gated |
| **Template System** | ✅ Liquid | ✅ Simple | Conditional compilation |
| **Caching** | ✅ Advanced | ✅ Basic | Dual implementation |

## Technical Analysis

### **Current Tokio Usage**

**Minimal Usage Pattern:**
```rust
// src/og_image/cache.rs:9
use tokio::sync::RwLock as AsyncRwLock;

// Used only for:
// - Async cache operations
// - Concurrent access to cache storage
// - No networking or I/O operations
```

**Replacement Strategy:**
```rust
// WASM-compatible alternative
#[cfg(target_arch = "wasm32")]
use parking_lot::RwLock as AsyncRwLock;

#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::RwLock as AsyncRwLock;
```

### **Feature Flag Analysis**

**Current Features:**
```toml
default = ["ssr", "og-images", "file-conventions", "macros", "json-ld"]
ssr = ["leptos/ssr"]
csr = ["leptos/csr"]
hydrate = ["leptos/hydrate"]
og-images = ["image", "resvg", "usvg", "tiny-skia", "fontdue", "liquid"]
file-conventions = ["walkdir", "mime_guess", "image"]
json-ld = ["serde_json"]
caching = ["cached", "lru"]
http = ["reqwest"]
macros = ["leptos-next-metadata-macros", "proc-macro2", "quote", "syn"]
```

**Proposed WASM Features:**
```toml
wasm = ["csr", "json-ld", "macros", "basic-caching"]
wasm-advanced = ["wasm", "canvas-og-images", "web-storage"]
```

## Use Case Analysis

### **Primary WASM Use Cases**

1. **Client-Side Metadata Management**
   - Dynamic metadata updates in SPAs
   - Real-time metadata generation
   - Client-side SEO optimization

2. **Leptos CSR/Hydrate Applications**
   - Full-stack applications with client-side metadata
   - Progressive enhancement scenarios
   - Interactive metadata editing

3. **Browser Extension Development**
   - Metadata analysis tools
   - SEO auditing extensions
   - Content management interfaces

### **Performance Considerations**

| Aspect | Server-Side | WASM | Impact |
|--------|-------------|------|--------|
| **Bundle Size** | N/A | +200-500KB | Moderate |
| **Initial Load** | Fast | Slower | High |
| **Runtime Performance** | Fast | Fast | Low |
| **Memory Usage** | Low | Higher | Moderate |

## Competitive Analysis

### **Next.js Metadata API**
- **WASM Support**: Limited (server-side focused)
- **Client-Side**: Basic metadata updates only
- **Our Advantage**: Full Rust performance + WASM compatibility

### **Other Metadata Libraries**
- **React Helmet**: Client-side only, no WASM
- **Vue Meta**: Client-side only, no WASM
- **Our Advantage**: Universal compatibility (SSR + CSR + WASM)

## Risk Assessment

### **Technical Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Bundle Size Bloat** | Medium | High | Feature gating, tree shaking |
| **Performance Degradation** | Low | Medium | Benchmarking, optimization |
| **Maintenance Overhead** | High | Medium | Automated testing, CI/CD |
| **API Fragmentation** | Medium | High | Unified API design |

### **Business Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **User Confusion** | Medium | Medium | Clear documentation |
| **Feature Parity Issues** | High | Medium | Gradual rollout |
| **Support Burden** | Medium | Medium | Comprehensive docs |

## Recommendations

### **🎯 Strategy 1: Selective WASM Support (RECOMMENDED)**

**Implementation Approach:**
1. **Feature-gate WASM-incompatible modules**
2. **Replace tokio with WASM-compatible alternatives**
3. **Create WASM-specific implementations**
4. **Maintain server-side capabilities**

**Benefits:**
- ✅ Immediate value for client-side applications
- ✅ Preserves server-side strengths
- ✅ Moderate implementation effort
- ✅ Clear feature boundaries

### **🎯 Strategy 2: Dual Architecture (ADVANCED)**

**Implementation Approach:**
1. **Split into core + server packages**
2. **Separate WASM and native implementations**
3. **Unified API across platforms**

**Benefits:**
- ✅ Clean separation of concerns
- ✅ Optimized for each platform
- ✅ Independent versioning

**Drawbacks:**
- ❌ Higher maintenance overhead
- ❌ Complex dependency management
- ❌ Longer implementation timeline

### **🎯 Strategy 3: Conditional Compilation (IMMEDIATE)**

**Implementation Approach:**
1. **Quick wins for partial WASM support**
2. **Minimal changes to existing code**
3. **Gradual feature addition**

**Benefits:**
- ✅ Fast implementation
- ✅ Low risk
- ✅ Incremental value

**Drawbacks:**
- ❌ Limited feature set
- ❌ Technical debt potential

## Implementation Roadmap

### **Phase 1: Core WASM Support (2-3 weeks)**
- [ ] Feature-gate server-only modules
- [ ] Replace tokio with parking_lot for WASM
- [ ] Add WASM feature flag
- [ ] Create WASM-compatible prelude
- [ ] Basic metadata management in WASM

### **Phase 2: Enhanced WASM Features (4-6 weeks)**
- [ ] Client-side metadata management
- [ ] Browser API integration
- [ ] WASM-specific examples
- [ ] Documentation updates
- [ ] Performance benchmarking

### **Phase 3: Advanced WASM Support (8-12 weeks)**
- [ ] Client-side OG image generation (Canvas API)
- [ ] WASM-specific caching strategies
- [ ] Performance optimizations
- [ ] E2E WASM testing
- [ ] Production deployment

## Success Metrics

### **Technical Metrics**
- [ ] WASM build success rate: 100%
- [ ] Bundle size increase: <500KB
- [ ] Performance parity: >90% of native
- [ ] Test coverage: >80% for WASM features

### **User Metrics**
- [ ] WASM feature adoption: >20% of users
- [ ] User satisfaction: >4.5/5
- [ ] Support ticket reduction: >30%
- [ ] Documentation usage: >50% increase

## Conclusion

**WASM support for leptos-next-metadata is technically feasible and strategically valuable.** The investigation reveals a clear path forward with selective WASM support that preserves the library's server-side strengths while enabling client-side capabilities.

**Key Findings:**
1. **Primary blocker**: Tokio dependency chain (easily resolved)
2. **Core features**: Compatible with minimal changes
3. **Advanced features**: Require WASM-specific implementations
4. **Implementation effort**: Moderate (2-3 months for full support)
5. **Business value**: High (enables new use cases and market expansion)

**Recommendation**: Proceed with **Strategy 1 (Selective WASM Support)** as it provides the best balance of value, effort, and risk.

---

**Investigation completed by**: AI Agent  
**Next steps**: Create remediation plan and design documents  
**Status**: ✅ Ready for implementation planning

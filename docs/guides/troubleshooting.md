# 🚨 Troubleshooting Guide

> **Navigation**: [📚 Documentation Index](../index.md) | [🚀 Quick Start](getting-started.md) | [📋 Production Roadmap](PRODUCTION_ROADMAP.md)

## 📖 **Overview**

This guide helps you resolve common issues when using `leptos-next-metadata`. Each section includes the problem description, root cause, and step-by-step solutions.

---

## 🔧 **Compilation Issues**

### **1. "Cannot find `metadata!` macro"**

**Problem:**
```rust
error[E0433]: failed to resolve: use of undeclared macro `metadata!`
  --> src/main.rs:5:5
  |
5 |     metadata! {
  |     ^^^^^^^^^
  |
  = help: consider importing this macro
```

**Root Cause:** Missing import or incorrect feature flag.

**Solution:**
```rust
// ✅ Add the prelude import
use leptos_next_metadata::prelude::*;

// Or import the macro directly
use leptos_next_metadata_macros::metadata;
```

**Alternative Solution:**
```toml
# In Cargo.toml, ensure you have the macros feature
[dependencies]
leptos-next-metadata = { version = "0.1.0-beta.1", features = ["macros"] }
```

---

### **2. "Cannot find `generate_metadata!` macro"**

**Problem:**
```rust
error[E0433]: failed to resolve: use of undeclared macro `generate_metadata!`
  --> src/main.rs:8:5
  |
8 |     #[generate_metadata]
  |     ^^^^^^^^^^^^^^^^^
```

**Root Cause:** Missing macros crate dependency.

**Solution:**
```toml
# In Cargo.toml, add the macros crate
[dependencies]
leptos-next-metadata = "0.1.0-beta.1"
leptos-next-metadata-macros = "0.1.0-beta.1"
```

```rust
// In your Rust file, import the macro
use leptos_next_metadata_macros::generate_metadata;
```

---

### **3. "Feature `og-images` is not enabled"**

**Problem:**
```rust
error: feature `og-images` is not enabled
  --> src/main.rs:10:5
  |
10 | use leptos_next_metadata::og_image::OgImageGenerator;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Root Cause:** Missing feature flag for optional dependencies.

**Solution:**
```toml
# In Cargo.toml, enable the og-images feature
[dependencies]
leptos-next_metadata = { version = "0.1.0-beta.1", features = ["og-images"] }
```

**Available Features:**
- `og-images` - Open Graph image generation
- `json-ld` - Structured data support
- `file-conventions` - File-based metadata scanning
- `caching` - Advanced caching strategies
- `ssr` - Server-side rendering support
- `macros` - Procedural macro support

---

### **4. "Type mismatch: expected `Title`, found `String`"**

**Problem:**
```rust
error[E0308]: mismatched types
  --> src/main.rs:6:7
  |
6 |     title: "My Page",  // ❌ Expected Title, found &str
  |     ^^^^^^^^^^^^^^^^^
```

**Root Cause:** The `metadata!` macro expects specific types, not raw strings.

**Solution:**
```rust
// ✅ The macro automatically converts strings to appropriate types
metadata! {
    title: "My Page",  // This works fine
    description: "Page description",
}
```

**If you need explicit types:**
```rust
use leptos_next_metadata::metadata::{Metadata, Title};

let metadata = Metadata {
    title: Some(Title::Static("My Page".into())),
    description: Some("Page description".into()),
    ..Default::default()
};
```

---

## 🚨 **Runtime Issues**

### **1. "Metadata validation failed"**

**Problem:**
```rust
Error: Metadata validation failed: Title is too short (minimum 10 characters)
```

**Root Cause:** Metadata doesn't meet SEO best practices.

**Solution:**
```rust
// ✅ Ensure metadata meets requirements
metadata! {
    title: "My Awesome Page Title",  // At least 10 characters
    description: "A comprehensive description of the page content that provides value to users and search engines.",  // At least 50 characters
    keywords: ["rust", "leptos", "metadata", "web", "development"],  // Relevant keywords
}
```

**Validation Rules:**
- **Title**: 10-60 characters
- **Description**: 50-160 characters
- **Keywords**: 3-10 relevant terms
- **OG Image**: Valid URL or path

---

### **2. "OG image generation failed"**

**Problem:**
```rust
Error: Failed to generate OG image: Template not found
```

**Root Cause:** Missing template file or incorrect path.

**Solution:**
```rust
// ✅ Ensure template exists and path is correct
let generator = OgImageGenerator::new()?;

// Check if template exists
if !std::path::Path::new("templates/blog-post.svg").exists() {
    eprintln!("Template file not found");
    return Err(Error::TemplateNotFound("blog-post.svg".into()));
}

let image_bytes = generator.generate_og_image(
    "Page Title",
    "Page description",
    &OgImageParams::default()
)?;
```

**Template Requirements:**
- SVG format (recommended) or PNG
- Valid XML structure
- Placeholder text: `{title}` and `{description}`
- Appropriate dimensions (1200x630 recommended)

---

### **3. "JSON-LD serialization failed"**

**Problem:**
```rust
Error: JSON-LD serialization failed: Invalid field value
```

**Root Cause:** Invalid data in structured data fields.

**Solution:**
```rust
use leptos_next_metadata::json_ld::{Article, SchemaOrg};

// ✅ Ensure all fields have valid values
let article = Article {
    headline: post.title.clone(),  // Must be non-empty
    description: post.excerpt.clone(),  // Must be non-empty
    author: Some(post.author.name.clone()),  // Must be valid if Some
    date_published: Some(post.published_at.clone()),  // Must be valid date
    date_modified: Some(post.updated_at.clone()),  // Must be valid date
    image: Some(post.featured_image.clone()),  // Must be valid URL
    ..Default::default()
};

// Validate before serialization
if article.headline.is_empty() {
    return Err(Error::ValidationError("Article headline cannot be empty".into()));
}
```

---

### **4. "File convention scan failed"**

**Problem:**
```rust
Error: File convention scan failed: Permission denied
```

**Root Cause:** Insufficient permissions or invalid path.

**Solution:**
```rust
use leptos_next_metadata::conventions::ConventionScanner;

// ✅ Check path exists and is accessible
let app_path = "./app";
if !std::path::Path::new(app_path).exists() {
    return Err(Error::PathNotFound(app_path.into()));
}

// Use absolute path if needed
let scanner = ConventionScanner::new(std::env::current_dir()?.join("app"));

// Handle permissions gracefully
let conventions = match scanner.scan() {
    Ok(conventions) => conventions,
    Err(Error::PermissionDenied(path)) => {
        eprintln!("Permission denied for path: {}", path);
        // Fall back to default conventions
        FileConventions::default()
    }
    Err(e) => return Err(e),
};
```

---

## 🐛 **Common Logic Errors**

### **1. "Metadata not updating"**

**Problem:** Dynamic metadata doesn't change when data updates.

**Root Cause:** Missing signal dependencies or incorrect closure.

**Solution:**
```rust
use leptos::*;

#[component]
fn DynamicPage() -> impl IntoView {
    let (title, set_title) = create_signal("Initial Title".into());
    let (description, set_description) = create_signal("Initial description".into());

    // ✅ Use move closures to capture signals
    metadata! {
        title: move || title.get(),
        description: move || description.get(),
    }

    view! {
        <div>
            <input 
                placeholder="Enter title"
                on:input=move |ev| set_title.set(event_target_value(&ev))
            />
            <input 
                placeholder="Enter description"
                on:input=move |ev| set_description.set(event_target_value(&ev))
            />
        </div>
    }
}
```

---

### **2. "Cache not working"**

**Problem:** Cached metadata is not being retrieved.

**Root Cause:** Incorrect cache key or TTL expiration.

**Solution:**
```rust
use leptos_next_metadata::utils::cache::{MetadataCache, generate_cache_key};
use std::time::Duration;

let cache = MetadataCache::new(1000);

// ✅ Generate consistent cache keys
let cache_key = generate_cache_key(
    &metadata.title,
    &metadata.description,
    &metadata.og_type
);

// Set with appropriate TTL
cache.set(&cache_key, metadata.clone(), Duration::from_secs(3600));

// Retrieve using same key
if let Some(cached) = cache.get(&cache_key) {
    println!("Using cached metadata");
    return cached;
}

println!("Generating new metadata");
// Generate and cache new metadata
```

---

### **3. "Metadata merge not working"**

**Problem:** Base metadata is not being applied to pages.

**Root Cause:** Incorrect merge order or missing base metadata.

**Solution:**
```rust
use leptos_next_metadata::metadata::merge_metadata;

// ✅ Define base metadata first
let base_metadata = Metadata {
    title: Some(Title::Template("{} - Site Name".into())),
    description: Some("Default site description".into()),
    og_type: Some("website".into()),
    og_site_name: Some("My Site".into()),
    ..Default::default()
};

// Define page-specific metadata
let page_metadata = Metadata {
    title: Some(Title::Static("Page Title".into())),
    og_image: Some("/page-image.jpg".into()),
    ..Default::default()
};

// Merge in correct order (base first, then overrides)
let merged = merge_metadata(&base_metadata, &page_metadata)?;

// Result: "Page Title - Site Name" with page-specific OG image
```

---

## 🔍 **Debugging Techniques**

### **1. Enable Debug Logging**

**Add logging to your application:**
```rust
use log::{debug, info, warn, error};

// In your metadata generation
metadata! {
    title: "My Page",
    description: "Page description",
}

// Log metadata for debugging
debug!("Generated metadata: {:?}", metadata);
```

**Configure logging in Cargo.toml:**
```toml
[dependencies]
log = "0.4"
env_logger = "0.10"

# Set log level
RUST_LOG=debug cargo run
```

---

### **2. Use Metadata Validation**

**Validate metadata before use:**
```rust
use leptos_next_metadata::metadata::validate_metadata;

let metadata = metadata! {
    title: "My Page",
    description: "Page description",
};

// Validate and log issues
let validation = validate_metadata(&metadata);
if validation.score < 80 {
    warn!("Low metadata score: {}", validation.score);
    for issue in &validation.issues {
        warn!("Issue: {}", issue);
    }
}
```

---

### **3. Test Individual Components**

**Create minimal test cases:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_generation() {
        let metadata = metadata! {
            title: "Test Page",
            description: "Test description",
        };
        
        assert_eq!(metadata.title, Some(Title::Static("Test Page".into())));
        assert_eq!(metadata.description, Some("Test description".into()));
    }

    #[test]
    fn test_metadata_merge() {
        let base = Metadata {
            title: Some(Title::Template("{} - Site".into())),
            ..Default::default()
        };
        
        let page = Metadata {
            title: Some(Title::Static("Page".into())),
            ..Default::default()
        };
        
        let merged = merge_metadata(&base, &page).unwrap();
        assert_eq!(merged.title, Some(Title::Static("Page - Site".into())));
    }
}
```

---

## 📱 **Platform-Specific Issues**

### **1. WASM Compatibility**

**Problem:** Code works in native Rust but fails in WASM.

**Root Cause:** Some dependencies don't support WASM.

**Solution:**
```rust
// ✅ Use conditional compilation for WASM
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(not(target_arch = "wasm32"))]
use std::println as console_log;

// In your code
console_log!("Metadata generated successfully");
```

---

### **2. Server-Side Rendering**

**Problem:** Metadata works in browser but not during SSR.

**Root Cause:** Missing SSR feature or incorrect context.

**Solution:**
```toml
# Enable SSR feature
[dependencies]
leptos-next-metadata = { version = "0.1.0-beta.1", features = ["ssr"] }
```

```rust
// Ensure metadata is available during SSR
#[component]
fn MyPage() -> impl IntoView {
    // Metadata must be available before view generation
    let metadata = metadata! {
        title: "My Page",
        description: "Page description",
    };
    
    // Use metadata in SSR context
    provide_context(metadata);
    
    view! { 
        <div>
            <h1>"My Page"</h1>
        </div>
    }
}
```

---

## 🆘 **Getting Help**

### **1. Self-Diagnosis Checklist**

Before asking for help, check:

- [ ] **Dependencies**: All required crates installed
- [ ] **Features**: Required features enabled
- [ ] **Imports**: Correct imports and prelude usage
- [ ] **Types**: Proper type usage in metadata
- [ ] **Validation**: Metadata meets requirements
- [ ] **Logs**: Check debug output and error messages
- [ ] **Tests**: Run unit tests to isolate issues

---

### **2. Reporting Issues**

**Include in your issue report:**

```markdown
## Problem Description
Brief description of what's not working

## Expected Behavior
What you expected to happen

## Actual Behavior
What actually happened

## Code Example
```rust
// Minimal code that reproduces the issue
metadata! {
    title: "My Page",
    description: "Page description",
}
```

## Error Messages
Full error output from compiler/runtime

## Environment
- Rust version: `rustc --version`
- leptos-next-metadata version: `0.1.0-beta.1`
- Platform: macOS/Linux/Windows
- Target: native/wasm

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3
```

---

### **3. Community Resources**

- **GitHub Issues**: [Report bugs](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- **Stack Overflow**: Tag with `leptos-next-metadata`
- **Leptos Discord**: Community chat and support

---

## 🔗 **Related Documentation**

- **[Quick Start](getting-started.md)** - Get up and running quickly
- **[Core API](../api/core.md)** - Core metadata types and functions
- **[Macros API](../api/macros.md)** - Procedural macro documentation
- **[Performance Guide](performance-guide.md)** - Optimization techniques

---

## 🎯 **Quick Fix Reference**

| Problem | Quick Fix |
|---------|-----------|
| **Macro not found** | `use leptos_next_metadata::prelude::*;` |
| **Feature not enabled** | Add `features = ["feature-name"]` to Cargo.toml |
| **Type mismatch** | Use `metadata!` macro (handles conversions automatically) |
| **Validation failed** | Check title length (10-60 chars) and description length (50-160 chars) |
| **Template not found** | Verify SVG template exists and path is correct |
| **Cache not working** | Use `generate_cache_key` for consistent keys |
| **Metadata not updating** | Use `move ||` closures with signals |

---

*Last Updated: September 4, 2025*  
*Next: [Performance Guide](performance-guide.md) | [Production Roadmap](PRODUCTION_ROADMAP.md)*

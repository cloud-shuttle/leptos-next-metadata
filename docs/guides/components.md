# 🧩 Components Guide

> **Navigation**: [📚 Documentation Index](../index.md) | [🚀 Quick Start](getting-started.md) | [🔧 API Reference](../api/core.md)

Learn how to use the powerful metadata components in `leptos-next-metadata` for flexible and dynamic metadata management.

---

## 📖 **Overview**

Our component system provides a more flexible and dynamic approach to metadata management compared to the traditional macro-based approach. Each component is designed to work seamlessly with Leptos's reactive system and provides fine-grained control over metadata rendering.

---

## 🏗️ **Core Components**

### **`MetaTags` Component**

The `MetaTags` component injects meta tags into the document head during server-side rendering.

#### **Basic Usage**

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MetaTags />
        // ... rest of your app
    }
}
```

#### **When to Use**

- **SSR Applications**: Essential for server-side rendering
- **Meta Tag Injection**: Automatically injects metadata from context
- **SEO Optimization**: Ensures meta tags are present in HTML head

---

### **`Body` Component**

The `Body` component allows you to set attributes on the document's `<body>` element.

#### **Basic Usage**

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Body class="dark-theme" lang="en" dir="ltr" />
        // ... rest of your app
    }
}
```

#### **Available Attributes**

- `class` - CSS classes for the body element
- `lang` - Language attribute
- `dir` - Text direction (ltr/rtl)
- `id` - Unique identifier

#### **Example with All Attributes**

```rust
<Body 
    class="dark-theme app-body" 
    lang="en" 
    dir="ltr" 
    id="main-body" 
/>
```

---

### **`Html` Component**

The `Html` component allows you to set attributes on the document's `<html>` element.

#### **Basic Usage**

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Html lang="en" dir="ltr" />
        // ... rest of your app
    }
}
```

#### **Available Attributes**

- `lang` - Language attribute
- `dir` - Text direction (ltr/rtl)
- `data-*` - Custom data attributes
- `id` - Unique identifier

#### **Example with Custom Attributes**

```rust
<Html 
    lang="en" 
    dir="ltr" 
    data-theme="dark" 
    data-mode="compact" 
    id="main-html" 
/>
```

---

### **`HashedStylesheet` Component**

The `HashedStylesheet` component injects a hashed stylesheet link for cargo-leptos integration.

#### **Basic Usage**

```rust
use leptos_next_metadata::prelude::*;
use leptos::prelude::LeptosOptions;

#[component]
fn App() -> impl IntoView {
    let options = LeptosOptions::builder()
        .output_name("my-app")
        .build();
        
    view! {
        <HashedStylesheet options=options />
        // ... rest of your app
    }
}
```

#### **Advanced Usage**

```rust
<HashedStylesheet 
    options=options 
    id="main-stylesheet" 
    root="/assets" 
/>
```

#### **Parameters**

- `options` - LeptosOptions containing build configuration
- `id` - Optional ID for the stylesheet
- `root` - Optional base URL for assets

---

### **`EnhancedTitle` Component**

The `EnhancedTitle` component provides advanced title formatting with multiple options.

#### **Basic Usage**

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <EnhancedTitle text="My Page" />
        // ... rest of your app
    }
}
```

#### **Formatter Function**

```rust
let formatter = |text: &str| format!("{} | My Site", text);

<EnhancedTitle 
    text="My Page" 
    formatter=formatter 
/>
```

#### **Template String**

```rust
<EnhancedTitle 
    text="My Page" 
    template="{} | My Site"
/>
```

#### **Prefix and Suffix**

```rust
<EnhancedTitle 
    text="My Page" 
    prefix="Welcome to"
    suffix="| My Site"
/>
```

#### **Priority System**

The component follows a priority system:

1. **Formatter** (highest priority)
2. **Template** (medium priority)
3. **Prefix + Suffix** (lowest priority)

```rust
// This will use the formatter, ignoring template and prefix/suffix
<EnhancedTitle 
    text="My Page" 
    formatter=|text| format!("Formatted: {}", text)
    template="{} | My Site"
    prefix="Welcome to"
    suffix="| My Site"
/>
```

---

## 🔄 **Component Interactions**

### **Working Together**

All components are designed to work seamlessly together:

```rust
use leptos_next_metadata::prelude::*;
use leptos::prelude::LeptosOptions;

#[component]
fn App() -> impl IntoView {
    let options = LeptosOptions::builder()
        .output_name("my-app")
        .build();
        
    view! {
        <Html lang="en" dir="ltr" data-theme="dark" />
        <Body class="app-body" lang="en" />
        <MetaTags />
        <EnhancedTitle 
            text="My App" 
            template="{} | My Site"
        />
        <HashedStylesheet 
            options=options 
            id="main-stylesheet" 
        />
        
        <main>
            <h1>"Welcome to My App"</h1>
            <p>"This app uses all our metadata components!"</p>
        </main>
    }
}
```

### **Dynamic Updates**

Components work with Leptos's reactive system:

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn DynamicApp() -> impl IntoView {
    let (title, set_title) = create_signal("Initial Title".to_string());
    
    view! {
        <EnhancedTitle text=title />
        
        <button on:click=move |_| {
            set_title.set("Updated Title".to_string());
        }>
            "Update Title"
        </button>
    }
}
```

---

## 🎯 **Best Practices**

### **1. Component Order**

Place components in the correct order for optimal rendering:

```rust
view! {
    <Html lang="en" />
    <Body class="app" />
    <MetaTags />
    <EnhancedTitle text="Page Title" />
    <HashedStylesheet options=options />
    // ... rest of your app
}
```

### **2. Performance Optimization**

- Use `EnhancedTitle` with templates for static titles
- Use formatters only when dynamic formatting is needed
- Place `HashedStylesheet` early in the component tree

### **3. Accessibility**

Always include proper language attributes:

```rust
<Html lang="en" dir="ltr" />
<Body lang="en" />
```

### **4. SEO Optimization**

Use descriptive titles and proper formatting:

```rust
<EnhancedTitle 
    text="Product Name" 
    template="{} | Company Name"
/>
```

---

## 🚨 **Common Pitfalls**

### **1. Missing MetaTags in SSR**

```rust
// ❌ Wrong - MetaTags missing in SSR
view! {
    <EnhancedTitle text="My Page" />
    // ... app content
}

// ✅ Correct - Include MetaTags for SSR
view! {
    <MetaTags />
    <EnhancedTitle text="My Page" />
    // ... app content
}
```

### **2. Incorrect Priority Usage**

```rust
// ❌ Wrong - Confusing priority
<EnhancedTitle 
    text="My Page" 
    formatter=|text| format!("Formatted: {}", text)
    template="{} | My Site"  // This will be ignored
/>

// ✅ Correct - Clear priority
<EnhancedTitle 
    text="My Page" 
    formatter=|text| format!("Formatted: {}", text)
/>
```

### **3. Missing LeptosOptions**

```rust
// ❌ Wrong - Missing options
<HashedStylesheet />

// ✅ Correct - Provide options
<HashedStylesheet options=leptos_options />
```

---

## 🔧 **Advanced Usage**

### **Custom Formatters**

Create complex formatters for dynamic titles:

```rust
let smart_formatter = |text: &str| {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() > 5 {
        format!("{} ({} words) | My Site", text, words.len())
    } else {
        format!("{} | My Site", text)
    }
};

<EnhancedTitle 
    text="This is a very long title with many words"
    formatter=smart_formatter
/>
```

### **Conditional Components**

Use components conditionally based on app state:

```rust
#[component]
fn ConditionalApp() -> impl IntoView {
    let (is_dark, set_is_dark) = create_signal(false);
    
    view! {
        <Html 
            lang="en" 
            data-theme=move || if is_dark() { "dark" } else { "light" }
        />
        <Body 
            class=move || if is_dark() { "dark-theme" } else { "light-theme" }
        />
        
        <button on:click=move |_| set_is_dark.toggle()>
            "Toggle Theme"
        </button>
    }
}
```

---

## 📚 **Related Documentation**

- [🚀 Quick Start Guide](getting-started.md) - Get started quickly
- [🔧 Core API Reference](../api/core.md) - Detailed API documentation
- [📋 Production Roadmap](PRODUCTION_ROADMAP.md) - Production deployment guide
- [🔍 Troubleshooting Guide](troubleshooting.md) - Common issues and solutions

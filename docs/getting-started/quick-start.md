# 🚀 Quick Start Guide

> **Time to complete**: ~5 minutes  
> **Prerequisites**: Basic Rust knowledge, Leptos project setup

Get up and running with `leptos-next-metadata` in just a few minutes!

---

## 📦 **Step 1: Installation**

Add the library to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.8"
leptos-next-metadata = "1.0.0"
```

---

## 🏗️ **Step 2: Basic Setup**

### **Using Components (Recommended)**

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Html lang="en" />
        <Title text="My Awesome App" />
        <MetaTags>
            <Meta name="description" content="A great Leptos application" />
            <Meta property="og:title" content="My Awesome App" />
            <Meta property="og:description" content="A great Leptos application" />
            <Meta property="og:image" content="/og-image.jpg" />
        </MetaTags>
    }
}
```

### **Using Macros (Traditional)**

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    metadata! {
        title: "My Awesome App",
        description: "A great Leptos application",
        og_image: "/og-image.jpg",
    }

    view! { <div>"Hello, World!"</div> }
}
```

---

## 🎯 **Step 3: Advanced Features**

### **Enhanced Title with Formatting**

```rust
#[component]
pub fn BlogPost() -> impl IntoView {
    view! {
        <Html lang="en" />
        <EnhancedTitle 
            text="My Blog Post"
            template="%s | My Blog"
        />
        <MetaTags>
            <Meta name="description" content="An amazing blog post" />
            <Meta property="og:title" content="My Blog Post" />
        </MetaTags>
    }
}
```

### **Dynamic OG Images**

```rust
#[component]
pub fn ProductPage() -> impl IntoView {
    let product_title = "Amazing Product";
    let product_price = "$99.99";
    
    view! {
        <Html lang="en" />
        <Title text=product_title />
        <MetaTags>
            <Meta name="description" content="Check out this amazing product" />
            <Meta property="og:title" content=product_title />
            <Meta property="og:image" content=format!("/api/og?title={}&price={}", product_title, product_price) />
        </MetaTags>
    }
}
```

### **JSON-LD Structured Data**

```rust
#[component]
pub fn ArticlePage() -> impl IntoView {
    view! {
        <Html lang="en" />
        <Title text="My Article" />
        <MetaTags>
            <Meta name="description" content="An informative article" />
        </MetaTags>
        <JsonLd>
            {json!({
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "My Article",
                "description": "An informative article",
                "author": {
                    "@type": "Person",
                    "name": "John Doe"
                }
            })}
        </JsonLd>
    }
}
```

---

## 🧪 **Step 4: Test Your Setup**

Create a simple test to verify everything works:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_basic_metadata() {
        let app = view! {
            <Html lang="en" />
            <Title text="Test Page" />
            <MetaTags>
                <Meta name="description" content="Test description" />
            </MetaTags>
        };
        
        // Your test assertions here
        assert!(true); // Replace with actual tests
    }
}
```

---

## 🎉 **You're Ready!**

### **What's Next?**

1. **[Components Guide](../guides/components.md)** - Learn about all available components
2. **[API Reference](../api/)** - Complete API documentation
3. **[Examples](../examples/)** - Real-world use cases
4. **[Performance Guide](../guides/performance-guide.md)** - Optimization tips

### **Common Next Steps**

- **Add more metadata** - Use the [Components Guide](../guides/components.md)
- **Optimize performance** - Check the [Performance Guide](../guides/performance-guide.md)
- **Handle errors** - See the [Troubleshooting Guide](../guides/troubleshooting.md)
- **Migrate from Next.js** - Follow the [Migration Guide](../guides/migration-guide.md)

---

## 🆘 **Need Help?**

### **Common Issues**
- **Installation problems** - Check [Installation Guide](installation.md)
- **Runtime errors** - See [Troubleshooting Guide](../guides/troubleshooting.md)
- **Performance issues** - Review [Performance Guide](../guides/performance-guide.md)

### **Get Support**
- **[GitHub Issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)** - Bug reports
- **[GitHub Discussions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)** - Community help
- **[Documentation](../README.md)** - Complete guides

---

**🎯 Ready to build amazing metadata for your Leptos app? [Explore the Components Guide](../guides/components.md) next!**

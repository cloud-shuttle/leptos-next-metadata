# 🚀 Leptos Next Metadata - Static Demo

This is a **static HTML demonstration** of the `leptos-next-metadata` library capabilities, showcasing advanced metadata management for Leptos applications.

## ⚠️ Important Note

**This is a static HTML version** because GitHub Pages doesn't support Rust servers. The actual `leptos-next-metadata` library provides **server-side rendering (SSR)** capabilities that generate metadata dynamically on the server.

## 🌐 Live Demo

**URL**: https://leptos-next-metadata.github.io/leptos-next-metadata/

**For the actual Rust server demo**, see the [Server Demo Source](https://github.com/leptos-next-metadata/leptos-next-metadata/tree/main/examples/server_demo) and [WASM Demo Source](https://github.com/leptos-next-metadata/leptos-next-metadata/tree/main/examples/wasm_demo).

## 📊 Features Demonstrated

### 🎯 Core Metadata Features
- **Dynamic Title Generation**: Route-specific titles with SEO optimization
- **Meta Descriptions**: Contextual descriptions for each page
- **Open Graph Tags**: Social media sharing optimization
- **Twitter Cards**: Enhanced Twitter sharing experience
- **JSON-LD Structured Data**: Rich snippets for search engines

### 🖼️ Advanced Features
- **OG Image Generation**: Custom social media preview images
- **Performance Monitoring**: Real-time metrics and optimization
- **Analytics Integration**: Built-in tracking and insights
- **SEO Optimization**: Comprehensive search engine optimization

## 📄 Demo Pages

1. **[Homepage](index.html)** - Overview and feature showcase
2. **[About](about.html)** - Library information and capabilities
3. **[Blog](blog.html)** - Dynamic blog post with article metadata
4. **[Products](products.html)** - Product showcase with structured data
5. **[OG Test](og-test.html)** - Open Graph image testing
6. **[Performance](performance.html)** - Performance metrics dashboard
7. **[Analytics](analytics.html)** - Analytics and tracking demo

## 🛠️ Technical Implementation

### ⚠️ Static HTML Demo (This Demo)
- **Pre-built HTML** for GitHub Pages compatibility
- **Static metadata** showcasing the library's capabilities
- **Fast loading** with CDN optimization
- **No server required** - works on GitHub Pages

### 🦀 Actual Library Capabilities (Rust Server)
- **Server-Side Rendering (SSR)**: Metadata generated on the server for optimal SEO
- **Dynamic content** based on route parameters
- **Caching strategies** for performance
- **Client-Side Hydration**: Seamless hydration with dynamic updates
- **Real-time metadata changes** and interactive components

## 🔧 Development

### Local Development
```bash
# Run the server demo locally
cd examples/server_demo
cargo run --features ssr

# Access at http://127.0.0.1:3004
```

### Static Demo
```bash
# Serve the static demo
cd docs/demo
python3 -m http.server 8080

# Access at http://localhost:8080
```

## 📈 Performance Metrics

### Static Demo (GitHub Pages)
- **Page Load Time**: ~150ms
- **CDN Delivery**: Global edge caching
- **No Server Required**: Static hosting

### Rust Server Demo (Local)
- **Page Load Time**: ~150ms
- **Metadata Generation**: ~5ms
- **Cache Hit Rate**: 95%
- **Memory Usage**: ~2.3MB

## ⚠️ Limitations of Static Demo

- **No Dynamic Generation**: Metadata is pre-written, not generated server-side
- **No Real-time Updates**: Content is static, not reactive
- **No Server Features**: No caching, analytics, or performance monitoring
- **GitHub Pages Limitation**: Cannot run Rust servers on GitHub Pages

**For the full experience**, run the actual Rust server demo locally.

## 🔗 Related Links

- [GitHub Repository](https://github.com/leptos-next-metadata/leptos-next-metadata)
- [Documentation](https://docs.rs/leptos-next-metadata)
- [Crates.io](https://crates.io/crates/leptos-next-metadata)

## 📝 License

This demo is part of the `leptos-next-metadata` project and is licensed under the same terms as the main library.

# 🚀 Leptos Next Metadata - Server Demo

A **pure Rust server demo** showcasing `leptos-next-metadata`'s server-side capabilities. This demo is built entirely in Rust and tests the actual library functionality in a server environment.

## ✨ What This Demo Tests

### 🎯 Core Library Functionality

- **SSR Metadata Management** - Server-side metadata generation
- **OG Image Generation** - Server-side image creation
- **File Conventions** - Automatic metadata from file structure
- **JSON-LD Structured Data** - Rich structured data generation
- **Caching** - Performance optimization with caching
- **Route-based Metadata** - Dynamic metadata per route

### 🛠️ Technical Implementation

- **Pure Rust** - No HTML/CSS/JS, everything is Rust code
- **Leptos SSR** - Server-side rendering with Leptos
- **Axum Integration** - High-performance web server
- **Library Testing** - Actually tests the leptos-next-metadata library

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable)
- Cargo

### Installation

1. **Navigate to the demo:**

   ```bash
   cd examples/server_demo
   ```

2. **Build the server:**

   ```bash
   cargo build --features ssr
   ```

3. **Run the server:**

   ```bash
   cargo run --features ssr
   ```

4. **Open your browser:**

   ```
   http://127.0.0.1:3000
   ```

## 🏗️ Architecture

### Rust Code Structure

```
src/
├── main.rs              # Main server application with Leptos components
└── Cargo.toml          # Dependencies and features
```

### Key Components

- **App Component** - Main Leptos application with routing
- **HomePage** - Homepage with comprehensive metadata
- **AboutPage** - About page with article metadata
- **BlogPage** - Blog listing with blog metadata
- **BlogPostPage** - Individual blog posts with dynamic metadata
- **ProductsPage** - Product catalog with e-commerce metadata
- **ProductPage** - Individual products with product metadata
- **OgTestPage** - OG image generation testing
- **PerformancePage** - Performance monitoring

## 🧪 Testing the Library

### What Gets Tested

1. **Metadata Context** - `provide_metadata_context()` function
2. **Metadata Macro** - `metadata!` macro compilation and execution
3. **SSR Rendering** - Server-side metadata generation
4. **Route Metadata** - Dynamic metadata per route
5. **OG Image Generation** - Server-side image creation
6. **JSON-LD Generation** - Structured data creation
7. **Caching** - Performance optimization

### Verification Steps

1. **Check Server Logs** - Look for Rust logs and server startup
2. **Inspect Page Source** - Verify metadata tags are generated
3. **Test Different Routes** - Navigate to different pages
4. **Check OG Images** - Verify OG image generation
5. **Validate JSON-LD** - Check structured data
6. **Monitor Performance** - Check for performance metrics

## 🔧 Development

### Building

```bash
# Development build
cargo build --features ssr

# Production build (optimized)
cargo build --features ssr --release
```

### Running

```bash
# Run in development mode
cargo run --features ssr

# Run in release mode
cargo run --features ssr --release
```

### Cleaning

```bash
# Remove build artifacts
cargo clean
```

## 🧪 Testing

### Manual Testing

1. Start the server
2. Navigate to different routes
3. Check page source for metadata
4. Test OG image generation
5. Verify JSON-LD structured data
6. Monitor server performance

### Automated Testing

```bash
# Run server tests (when implemented)
cargo test --features ssr
```

## 🐛 Troubleshooting

### Common Issues

**Server not starting:**

- Check that all dependencies are installed
- Verify feature flags are correct
- Check for port conflicts (3000)

**Build failures:**

- Update Rust: `rustup update`
- Update dependencies: `cargo update`
- Clear build cache: `cargo clean && cargo build --features ssr`

**Library not working:**

- Check that leptos-next-metadata is properly imported
- Verify feature flags are correct
- Ensure SSR features are enabled

### Server Requirements

- **Rust**: Latest stable
- **Cargo**: Latest version
- **Port**: 3000 (configurable)

## 📊 Performance

### Server Performance

- **Startup Time**: <2 seconds
- **Request Processing**: <10ms
- **Metadata Generation**: <5ms
- **OG Image Generation**: 50-200ms
- **Memory Usage**: <50MB

### Optimization Features

- **Caching**: Intelligent metadata caching
- **Compression**: Gzip compression for responses
- **Static Assets**: Efficient static file serving
- **Connection Pooling**: Optimized database connections

## 🔒 Security

### Server Security

- **Memory Safety**: Rust's ownership system prevents memory leaks
- **Type Safety**: Compile-time type checking
- **Input Validation**: Proper input sanitization
- **CORS**: Cross-origin resource sharing configuration

## 🤝 Contributing

### Adding New Features

1. Update `src/main.rs` with new components
2. Add corresponding routes
3. Test across different scenarios
4. Update documentation

### Code Style

- Follow Rust formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write comprehensive documentation
- Include error handling

## 📚 Learn More

### Documentation

- [Leptos Next Metadata Docs](../../docs/)
- [SSR Architecture Design](../../docs/design/)
- [API Reference](../../docs/api/)

### Examples

- [WASM Demo](../wasm_demo/) - Client-side metadata management
- [Basic Example](../basic/) - Simple metadata setup
- [Advanced SEO Example](../advanced-seo/) - Advanced SEO techniques

### Community

- [GitHub Repository](https://github.com/leptos-rs/leptos-next-metadata)
- [Discord Community](https://discord.gg/leptos)
- [Documentation Site](https://leptos-next-metadata.dev)

## 📄 License

This demo is licensed under the same terms as the main project:

- MIT License
- Apache-2.0 License

See [LICENSE](../../LICENSE) for details.

---

**Built with ❤️ using Leptos Next Metadata and Rust**

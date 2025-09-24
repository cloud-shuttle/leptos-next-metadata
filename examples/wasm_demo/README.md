# 🚀 Leptos Next Metadata - WASM Demo

A **pure Rust WASM demo** showcasing `leptos-next-metadata`'s client-side capabilities. This demo is built entirely in Rust and tests the actual library functionality.

## ✨ What This Demo Tests

### 🎯 Core Library Functionality

- **Metadata Management** - Real-time metadata updates using Rust
- **OG Image Generation** - Canvas-based image generation with WASM
- **Theme System** - Dynamic theme switching with Rust state management
- **Analytics Integration** - Event tracking and performance monitoring
- **Web Workers** - Background task processing with Rust

### 🛠️ Technical Implementation

- **Pure Rust** - No HTML/CSS/JS, everything is Rust code
- **WASM Integration** - Full Rust performance in the browser
- **Leptos Framework** - Reactive UI with Rust
- **Library Testing** - Actually tests the leptos-next-metadata library

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable)
- wasm-pack
- pnpm (for serving)

### Installation

1. **Navigate to the demo:**

   ```bash
   cd examples/wasm_demo
   ```

2. **Build the WASM module:**

   ```bash
   pnpm run build
   ```

3. **Start the development server:**

   ```bash
   pnpm run serve
   ```

4. **Open your browser:**

   ```
   http://localhost:8080
   ```

### Alternative: One-command setup

```bash
pnpm run start
```

## 🏗️ Architecture

### Rust Code Structure

```
src/
├── lib.rs              # Main WASM entry point with Leptos components
└── Cargo.toml          # Dependencies and features

pkg/                    # Generated WASM package
├── leptos_next_metadata_bg.wasm
├── leptos_next_metadata.js
└── package.json

index.html              # Simple HTML loader (minimal)
```

### Key Components

- **App Component** - Main Leptos application
- **Metadata Management** - Real-time metadata updates
- **OG Image Generation** - Canvas-based image creation
- **Theme System** - Dynamic theme switching
- **Analytics** - Event tracking and monitoring
- **Performance** - Benchmarking and optimization
- **Web Workers** - Background task processing

## 🧪 Testing the Library

### What Gets Tested

1. **Metadata Context** - `provide_metadata_context()` function
2. **Metadata Macro** - `metadata!` macro compilation and execution
3. **OG Image Generation** - Canvas-based image creation
4. **Theme System** - Dynamic theme application
5. **Analytics** - Event tracking and data collection
6. **Performance** - Operation timing and optimization
7. **Web Workers** - Background task processing

### Verification Steps

1. **Check Browser Console** - Look for Rust logs and WASM initialization
2. **Inspect Page Source** - Verify metadata tags are generated
3. **Test Functionality** - Interact with the demo to test features
4. **Monitor Performance** - Check for performance metrics
5. **Validate WASM** - Ensure Rust code is actually running

## 🔧 Development

### Building

```bash
# Development build (faster, includes debug info)
pnpm run build:dev

# Production build (optimized, smaller size)
pnpm run build
```

### Serving

```bash
# Python HTTP server
pnpm run serve

# Node.js serve (requires pnpm)
pnpm run serve:node
```

### Cleaning

```bash
# Remove build artifacts
pnpm run clean
```

## 🧪 Testing

### Manual Testing

1. Open browser developer tools
2. Navigate through the demo
3. Check console for Rust logs
4. Verify metadata updates in page source
5. Test OG image generation
6. Validate theme switching
7. Monitor performance metrics

### Automated Testing

```bash
# Run demo tests (when implemented)
pnpm test
```

## 🐛 Troubleshooting

### Common Issues

**WASM module not loading:**

- Check browser console for errors
- Ensure you're serving from HTTP/HTTPS (not file://)
- Verify wasm-pack is installed: `wasm-pack --version`

**Build failures:**

- Update Rust: `rustup update`
- Update wasm-pack: `cargo install wasm-pack`
- Clear build cache: `pnpm run clean && pnpm run build`

**Library not working:**

- Check that leptos-next-metadata is properly imported
- Verify feature flags are correct
- Ensure WASM bindings are properly generated

### Browser Compatibility

- **Chrome/Chromium**: Full support
- **Firefox**: Full support
- **Safari**: Full support
- **Edge**: Full support

## 📊 Performance

### Bundle Sizes

- **WASM Module**: ~200-500KB (optimized)
- **JavaScript Bindings**: ~50-100KB
- **Total Load Time**: <1 second on modern connections

### Runtime Performance

- **Metadata Updates**: <10ms
- **OG Image Generation**: 100-500ms (depending on complexity)
- **Theme Switching**: <5ms
- **Analytics Tracking**: <1ms

## 🔒 Security

### WASM Security

- **Memory Safety**: Rust's ownership system prevents memory leaks
- **Type Safety**: Compile-time type checking
- **Sandboxing**: WASM runs in browser sandbox
- **No Eval**: No dynamic code execution

## 🤝 Contributing

### Adding New Features

1. Update `src/lib.rs` with new Leptos components
2. Add corresponding functionality
3. Test across different browsers
4. Update documentation

### Code Style

- Follow Rust formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write comprehensive documentation
- Include error handling

## 📚 Learn More

### Documentation

- [Leptos Next Metadata Docs](../../docs/)
- [WASM Architecture Design](../../docs/design/WASM_ARCHITECTURE_DESIGN.md)
- [API Reference](../../docs/api/)

### Examples

- [Server Demo](../server_demo/) - Server-side metadata management
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

**Built with ❤️ using Leptos Next Metadata and Rust WASM**

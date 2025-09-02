# Installation

This guide will help you add leptos-next-metadata to your Leptos project and configure it for your specific needs.

## Requirements

- **Rust**: 1.70.0 or later (MSRV)
- **Leptos**: 0.8.8 or later
- **Target**: `wasm32-unknown-unknown` for client-side code

## Basic Installation

Add leptos-next-metadata to your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.8", features = ["nightly"] }
leptos-next-metadata = "0.1"
```

For server-side rendering, also add:

```toml
[dependencies]
leptos-next-metadata = { version = "0.1", features = ["ssr"] }
```

## Feature Flags

leptos-next-metadata uses feature flags to allow you to include only the functionality you need:

### Core Features

```toml
[dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = [
        "ssr",              # Server-side rendering support
        "og-images",        # OG image generation
        "file-conventions", # File-based metadata detection
    ]
}
```

### All Available Features

| Feature | Description | Default | Bundle Impact |
|---------|-------------|---------|---------------|
| `ssr` | Server-side rendering support | ✅ | +30KB |
| `og-images` | OG image generation with resvg | ✅ | +150KB |
| `file-conventions` | File-based metadata scanning | ✅ | +20KB |
| `json-ld` | JSON-LD structured data support | ❌ | +25KB |
| `validation` | SEO validation and suggestions | ❌ | +15KB |
| `templates` | Advanced OG image templates | ❌ | +40KB |

### Minimal Configuration

For the smallest bundle size with basic metadata only:

```toml
[dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    default-features = false,
    features = ["ssr"]  # Only if using SSR
}
```

This gives you ~50KB bundle size with core metadata functionality.

### Full Configuration

For all features and maximum functionality:

```toml
[dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = [
        "ssr", 
        "og-images", 
        "file-conventions", 
        "json-ld",
        "validation",
        "templates"
    ]
}
```

This provides the complete feature set at ~280KB bundle size.

## Platform-Specific Setup

### Web Assembly (Client-Side)

No additional setup required. The library automatically detects the WASM environment and optimizes accordingly.

### Server-Side (Native)

For server-side rendering, you may want additional async runtime support:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
leptos-next-metadata = { version = "0.1", features = ["ssr"] }
```

### OG Image Generation Dependencies

If using the `og-images` feature, you might need system dependencies:

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get install libfontconfig1-dev
```

#### macOS
```bash
# Usually no additional dependencies needed
brew install fontconfig  # If you encounter issues
```

#### Windows
```bash
# Usually works out of the box
# If you encounter issues, install Visual Studio Build Tools
```

## Development Dependencies

For development and testing, add these to your `[dev-dependencies]`:

```toml
[dev-dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = ["testing", "dev-tools"] 
}
```

## Version Compatibility

| leptos-next-metadata | Leptos | Rust MSRV |
|---------------------|--------|-----------|
| 0.1.x               | 0.8.x  | 1.70.0    |

## Cargo Features Reference

### Production Features

- **`ssr`**: Enables server-side rendering optimizations and server functions
- **`og-images`**: Includes resvg-based image generation (requires native dependencies)
- **`file-conventions`**: Enables automatic metadata file detection and processing
- **`json-ld`**: Adds structured data support with Schema.org types

### Development Features

- **`dev-tools`**: Development-time SEO validation and helpful warnings
- **`hot-reload`**: File watching for metadata conventions (development only)
- **`testing`**: Testing utilities and mock implementations

### Advanced Features

- **`templates`**: Extended OG image template system with custom layouts
- **`validation`**: Runtime SEO validation and optimization suggestions  
- **`plugins`**: Plugin system for extending functionality
- **`i18n`**: Internationalization support for multilingual metadata

## Environment-Specific Configuration

### Development

```toml
# Cargo.toml
[dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = ["ssr", "dev-tools", "hot-reload"] 
}
```

### Production

```toml
# Cargo.toml  
[dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = ["ssr", "og-images"] 
}
```

### Testing

```toml
# Cargo.toml
[dev-dependencies]
leptos-next-metadata = { 
    version = "0.1", 
    features = ["testing", "validation"] 
}
```

## Next Steps

After installation, continue with:

1. **[Quick Start](quick-start.md)** - Create your first metadata-enhanced component
2. **[Project Setup](project-setup.md)** - Configure leptos-next-metadata for your project structure

## Troubleshooting

### Common Issues

**Build errors with OG images on Linux:**
```bash
# Install required system dependencies
sudo apt-get install libfontconfig1-dev pkg-config
```

**WASM compilation issues:**
```bash
# Ensure you have the WASM target installed
rustup target add wasm32-unknown-unknown
```

**Feature conflicts:**
- Don't enable `ssr` and `csr` features simultaneously
- Use `default-features = false` if you need minimal bundle size

### Getting Help

- **Documentation**: Continue through this guide
- **GitHub Issues**: [Report bugs and feature requests](https://github.com/yourusername/leptos-next-metadata/issues)
- **Discussions**: [Ask questions and share ideas](https://github.com/yourusername/leptos-next-metadata/discussions)

---

**Ready for the next step?** Continue to [Quick Start](quick-start.md) to create your first metadata-enhanced component.
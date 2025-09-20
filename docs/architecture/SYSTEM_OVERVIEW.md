# System Architecture Overview

## Overview

`leptos-next-metadata` is a comprehensive metadata management system for Leptos applications, providing Next.js-style APIs with type-safe metadata handling, Open Graph image generation, and SEO optimization.

## Architecture Principles

### 1. Modular Design
The system is built with clear separation of concerns:
- **Core Metadata**: Type-safe metadata structures and validation
- **OG Image Generation**: Template-based image generation with caching
- **File Conventions**: Automatic file detection and metadata extraction
- **JSON-LD**: Structured data generation for search engines
- **API Contracts**: Validation and middleware for HTTP APIs

### 2. Feature-Gated Functionality
Optional features are gated behind Cargo features:
- `og-images`: Open Graph image generation
- `json-ld`: JSON-LD structured data
- `file-conventions`: File-based metadata conventions
- `webp-support`: WebP image format support
- `ssr`: Server-side rendering support
- `csr`: Client-side rendering support

### 3. Performance-First
- **Lazy Loading**: Metadata is loaded only when needed
- **Caching**: Multi-level caching (memory, disk, HTTP)
- **Incremental Generation**: Only regenerate what's changed
- **Async Operations**: Non-blocking I/O for all operations

## Core Components

### Metadata System (`src/metadata/`)

The heart of the system, providing type-safe metadata management:

```
metadata/
├── types.rs          # Core metadata types and enums
├── builder.rs        # Builder pattern for metadata construction
├── display.rs        # Display and Debug implementations
├── serde_impl.rs     # Serialization/deserialization
├── merge.rs          # Metadata merging logic
├── context.rs        # Context management and providers
├── validation/       # Validation system
│   ├── core.rs       # Core validation logic
│   ├── rules.rs      # Validation rules and constraints
│   ├── types.rs      # Validation types and error codes
│   └── utils.rs      # Validation utilities
└── tests/            # Comprehensive test suite
```

**Key Types:**
- `Metadata`: Main metadata container
- `Title`: Template-based title system
- `Description`: SEO-optimized descriptions
- `OpenGraph`: Open Graph protocol support
- `Twitter`: Twitter Card support
- `Robots`: Search engine directives

### OG Image Generation (`src/og_image/`)

Template-based Open Graph image generation:

```
og_image/
├── types.rs          # Configuration and parameter types
├── generator.rs      # Main generation logic
├── template.rs       # Template rendering system
├── encoder.rs        # Image format encoding
└── mod.rs           # Public API
```

**Features:**
- SVG-based templates for scalability
- Multiple output formats (PNG, JPEG, WebP)
- Custom fonts and styling
- Automatic text wrapping and sizing
- Template caching and reuse

### File Conventions (`src/conventions/`)

Automatic metadata extraction from file conventions:

```
conventions/
├── scanner.rs        # File system scanning
├── patterns.rs       # Path pattern matching
├── mime_types.rs     # MIME type detection
├── config.rs         # Scanner configuration
└── mod.rs           # Public API
```

**Supported Conventions:**
- `opengraph-image.*`: OG images
- `twitter-image.*`: Twitter images
- `icon.*`: Favicons and app icons
- `manifest.json`: Web app manifest
- `robots.txt`: Search engine directives
- `sitemap.xml`: Site structure

### JSON-LD Support (`src/json_ld/`)

Structured data generation for search engines:

```
json_ld/
├── mod.rs           # Main JSON-LD types
├── schemas/         # Schema.org implementations
│   ├── article.rs   # Article schema
│   ├── person.rs    # Person schema
│   ├── organization.rs # Organization schema
│   └── web_page.rs  # WebPage schema
└── tests/           # Schema validation tests
```

**Supported Schemas:**
- Article, BlogPosting, NewsArticle
- Person, Organization
- WebPage, WebSite
- BreadcrumbList
- FAQPage

### API Contracts (`src/api/contracts/`)

Validation and middleware for HTTP APIs:

```
api/contracts/
├── types.rs         # Contract types and validation rules
├── validator.rs     # Core validation logic
├── middleware.rs    # HTTP middleware
├── tests.rs         # Comprehensive test suite
└── mod.rs          # Public API
```

**Features:**
- OpenAPI schema validation
- Field-level validation
- Error reporting and suggestions
- Middleware integration

## Data Flow

### 1. Metadata Construction
```
User Code → MetadataBuilder → Validation → Cached Metadata
```

### 2. OG Image Generation
```
Template + Data → SVG Rendering → Image Encoding → Cached Image
```

### 3. File Convention Processing
```
File System → Pattern Matching → Metadata Extraction → Context Update
```

### 4. JSON-LD Generation
```
Metadata → Schema Selection → JSON-LD Generation → HTML Injection
```

## Caching Strategy

### Multi-Level Caching
1. **Memory Cache**: Fast access for frequently used metadata
2. **Disk Cache**: Persistent storage for generated images
3. **HTTP Cache**: Browser caching with proper headers

### Cache Invalidation
- **Content-Based**: Cache keys based on content hash
- **Time-Based**: TTL for different cache types
- **Dependency-Based**: Invalidate when dependencies change

## Error Handling

### Structured Error System
- **Error Codes**: Consistent error identification
- **Context Information**: Detailed error context
- **Recovery Strategies**: Automatic retry and fallback
- **User-Friendly Messages**: Clear error messages for users

### Error Types
- `ValidationError`: Data validation failures
- `ImageError`: Image generation failures
- `ConfigError`: Configuration issues
- `CacheError`: Caching failures
- `NetworkError`: Network-related issues

## Performance Considerations

### Build-Time Optimizations
- **Feature Gating**: Compile only needed features
- **LTO**: Link-time optimization for release builds
- **Code Generation**: Single codegen unit for better optimization

### Runtime Optimizations
- **Lazy Loading**: Load resources only when needed
- **Connection Pooling**: Reuse HTTP connections
- **Async Operations**: Non-blocking I/O
- **Memory Management**: Efficient memory usage

## Security Considerations

### Input Validation
- **Sanitization**: Clean user inputs
- **Validation**: Strict data validation
- **Rate Limiting**: Prevent abuse

### Output Security
- **XSS Prevention**: Safe HTML generation
- **CSRF Protection**: Token-based protection
- **Content Security Policy**: Strict CSP headers

## Testing Strategy

### Test Coverage
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component testing
- **E2E Tests**: Full application testing
- **Property Tests**: Random input testing

### Test Types
- **Golden Tests**: Visual regression testing
- **Performance Tests**: Benchmarking
- **Security Tests**: Vulnerability testing
- **Compatibility Tests**: Cross-platform testing

## Deployment Considerations

### Environment Configuration
- **Development**: Debug features enabled
- **Staging**: Production-like with debugging
- **Production**: Optimized for performance

### Monitoring
- **Metrics**: Performance and usage metrics
- **Logging**: Structured logging
- **Alerting**: Error and performance alerts
- **Tracing**: Request tracing

## Future Roadmap

### Planned Features
- **AI Integration**: Automated metadata generation
- **Advanced Templates**: More OG image templates
- **Performance Monitoring**: Built-in metrics
- **Plugin System**: Extensible architecture

### Performance Goals
- **Sub-100ms**: Metadata generation time
- **Sub-500ms**: OG image generation time
- **99.9%**: Uptime and reliability
- **<1MB**: Bundle size for core features

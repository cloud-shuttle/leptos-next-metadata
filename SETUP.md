# 🚀 Development Environment Setup

This guide will help you set up the development environment for `leptos-next-metadata` with Nix, Rust, PNPM, and Playwright.

**Last Updated**: September 3rd, 2025  
**Leptos Version**: 0.8+  
**Rust Version**: 1.75+

## 🎯 What You'll Get

- **Rust 1.75+** with latest toolchain and modern language features
- **Leptos 0.8+** with latest reactive patterns and signal system
- **Node.js 20** with PNPM package manager
- **Playwright** with Chromium, Firefox, and WebKit browsers
- **Development tools** like `cargo-watch`, `just`, and more
- **System dependencies** for image processing and SVG rendering

## 🐧 Prerequisites

### Required

- **Nix** (with flakes support)
- **Git**
- **Rust 1.75+** (for async traits and modern language features)

### Optional

- **VS Code** with Rust and TypeScript extensions
- **Just** command runner (included in Nix shell)

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/cloud-shuttle/leptos-next-metadata.git
cd leptos-next-metadata
```

### 2. Enter the Development Environment

```bash
# Using Nix flakes (recommended)
nix develop

# Or using traditional Nix
nix-shell
```

### 3. Install Dependencies

```bash
# Install Node.js dependencies
pnpm install

# Install Playwright browsers
pnpm run install:playwright
```

### 4. Verify Setup

```bash
# Check Rust
cargo --version
rustc --version

# Check Node.js
node --version
pnpm --version

# Check Playwright
npx playwright --version

# Verify Leptos version
cargo tree -p leptos
```

## 🔧 Available Shells

### Default Shell (`nix develop`)

Full development environment with all tools and dependencies.

### Minimal Shell (`nix develop .#minimal`)

Basic environment with just Rust, Node.js, and PNPM.

### CI Shell (`nix develop .#ci`)

Environment optimized for continuous integration.

## 📦 Package Management

### Rust Dependencies

```bash
# Add a dependency
cargo add <package-name>

# Add a dev dependency
cargo add --dev <package-name>

# Update dependencies
cargo update

# Check for outdated packages
cargo outdated
```

### Node.js Dependencies

```bash
# Add a dependency
pnpm add <package-name>

# Add a dev dependency
pnpm add --dev <package-name>

# Update dependencies
pnpm update

# Check for outdated packages
pnpm outdated
```

## 🧪 Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Check examples
cargo check --examples
cargo run --example dynamic_metadata
```

### Playwright E2E Tests

```bash
# Run all E2E tests
pnpm test:e2e

# Run with UI
pnpm test:e2e:ui

# Run in headed mode
pnpm test:e2e:headed

# Run specific test
pnpm test:e2e --grep "test name"
```

### Development Server

```bash
# Start basic example
cargo run --example basic

# Start dynamic metadata example
cargo run --example dynamic_metadata

# Watch for changes
cargo watch -x 'run --example dynamic_metadata'

# Or use the script
pnpm run dev:watch
```

## 🎭 Playwright Setup

### Browser Installation

```bash
# Install all browsers
pnpm run install:playwright

# Install specific browser
npx playwright install chromium
npx playwright install firefox
npx playwright install webkit
```

### Test Configuration

- **Base URL**: `http://localhost:3000`
- **Test Server**: Automatically starts `cargo run --example dynamic_metadata`
- **Browsers**: Chrome, Firefox, Safari (desktop + mobile)
- **Parallel Execution**: Enabled for faster testing

## 🔍 Troubleshooting

### Common Issues

#### 1. Nix Flakes Not Enabled

```bash
# Enable flakes in your Nix configuration
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# Or use traditional Nix
nix-shell
```

#### 2. Port 3000 Already in Use

```bash
# Check what's using the port
lsof -i :3000

# Kill the process
kill -9 <PID>

# Or use a different port
PORT=3001 cargo run --example dynamic_metadata
```

#### 3. Playwright Browsers Not Found

```bash
# Reinstall browsers
pnpm run test:e2e:install-deps

# Check browser path
echo $PLAYWRIGHT_BROWSERS_PATH

# Manual installation
npx playwright install
```

#### 4. Image Processing Dependencies Missing

```bash
# Ensure you're in the Nix shell
nix develop

# Check pkg-config
pkg-config --version

# Verify libraries
pkg-config --exists libpng && echo "libpng found"
pkg-config --exists libjpeg && echo "libjpeg found"
```

#### 5. Leptos Version Compatibility Issues

```bash
# Check current Leptos version
cargo tree -p leptos

# Update to latest Leptos 0.8.x
cargo update -p leptos

# Verify compatibility
cargo check
```

### Performance Issues

#### Slow Test Execution

```bash
# Run tests in parallel
pnpm test:e2e --workers=4

# Use specific browser only
pnpm test:e2e --project=chromium

# Skip mobile tests
pnpm test:e2e --project="!Mobile*"
```

#### High Memory Usage

```bash
# Reduce browser instances
pnpm test:e2e --workers=2

# Use headless mode only
pnpm test:e2e --headed=false
```

## 🚀 Development Workflow

### 1. Start Development

```bash
nix develop
pnpm install
```

### 2. Make Changes

- Edit Rust code in `src/`
- Edit macros in `macros/src/`
- Edit examples in `examples/`
- Edit tests in `tests/`

### 3. Test Changes

```bash
# Quick Rust check
cargo check

# Check macros specifically
cargo check --package leptos-next-metadata-macros

# Run Rust tests
cargo test

# Start dev server
pnpm run dev:test-server

# Run E2E tests
pnpm test:e2e
```

### 4. Commit and Push

```bash
git add .
git commit -m "feat: add new feature"
git push
```

## 📚 Additional Resources

### Nix

- [Nix Flakes Guide](https://nixos.wiki/wiki/Flakes)
- [Nix Shell Tutorial](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html)

### Rust

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [Async Rust](https://rust-lang.github.io/async-book/)

### Leptos

- [Leptos Documentation](https://leptos.dev/)
- [Leptos Book](https://leptos.dev/book/)
- [Migration Guide](https://leptos.dev/book/0.8/migrating.html)

### Playwright

- [Playwright Documentation](https://playwright.dev/)
- [Testing Best Practices](https://playwright.dev/docs/best-practices)

### PNPM

- [PNPM Documentation](https://pnpm.io/)
- [Workspace Guide](https://pnpm.io/workspaces)

## 🤝 Getting Help

### Issues

- Check the [troubleshooting section](#-troubleshooting)
- Search existing [GitHub issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- Create a new issue with detailed information

### Community

- Join our [Discord server](https://discord.gg/your-server)
- Check the [documentation](https://docs.rs/leptos-next-metadata)
- Review the [examples](examples/)

### Development

- Read the [contributing guide](CONTRIBUTING.md)
- Check the [code of conduct](CODE_OF_CONDUCT.md)
- Review the [RFCs](docs/rfcs/) for design decisions

## 🚧 Current Project Status

**Project Status**: Active Development  
**Release Target**: Q4 2025  
**Current Version**: 0.1.0-alpha

### ✅ Completed Features

- Core metadata structures and types
- `metadata!` macro for static metadata
- `generate_metadata!` macro for dynamic metadata
- Basic OpenGraph and Twitter Card support
- JSON-LD foundation with conditional compilation
- Procedural macro system
- Integration with Leptos 0.8+ signal system

### 🔄 In Progress

- OG image generation pipeline
- File convention scanner
- Advanced caching strategies
- Performance optimizations
- Integration with Leptos ecosystem

### 📋 Planned Features

- Comprehensive testing suite with Playwright
- Performance benchmarking
- Migration tools from Next.js
- Advanced template system
- Complete documentation book

---

## 🤖 AI-Generated Content Disclosure

**Note**: This setup guide has been generated and enhanced using Large Language Models (LLMs) to ensure comprehensive coverage and clarity. The installation steps, troubleshooting guides, and development workflows are accurate and have been reviewed for correctness. This project represents a modern Rust implementation leveraging the latest Leptos v0.8+ features and best practices as of September 2025.

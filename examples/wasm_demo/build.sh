#!/bin/bash
# Build script for leptos-next-metadata WASM demo

set -e

echo "🚀 Building Leptos Next Metadata WASM Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Not in the demo directory. Please run from examples/wasm_demo/"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg/ target/

# Build with wasm-pack
echo "📦 Building WASM module..."
wasm-pack build \
    --target web \
    --out-dir pkg \
    --scope leptos-next-metadata \
    --features wasm-advanced \
    --release

# Check if build was successful
if [ ! -f "pkg/leptos_next_metadata_bg.wasm" ]; then
    echo "❌ WASM build failed"
    exit 1
fi

# Get file sizes
WASM_SIZE=$(stat -f%z pkg/leptos_next_metadata_bg.wasm 2>/dev/null || stat -c%s pkg/leptos_next_metadata_bg.wasm 2>/dev/null || echo "unknown")
JS_SIZE=$(stat -f%z pkg/leptos_next_metadata.js 2>/dev/null || stat -c%s pkg/leptos_next_metadata.js 2>/dev/null || echo "unknown")

echo "📊 Build Statistics:"
echo "   WASM size: $WASM_SIZE bytes"
echo "   JS size: $JS_SIZE bytes"

# Check if sizes are reasonable
if [ "$WASM_SIZE" != "unknown" ] && [ "$WASM_SIZE" -gt 1048576 ]; then
    echo "⚠️  WASM bundle is larger than 1MB. Consider optimization."
else
    echo "✅ WASM bundle size is within acceptable limits"
fi

echo "🎉 Build complete!"
echo "📁 Output directory: pkg/"
echo "🌐 To run the demo:"
echo "   pnpm run serve"
echo "   # or"
echo "   python3 -m http.server 8080"
echo ""
echo "🔗 Then open: http://localhost:8080"

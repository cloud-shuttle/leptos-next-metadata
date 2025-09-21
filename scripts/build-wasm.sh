#!/bin/bash
# Optimized WASM build script for leptos-next-metadata

set -e

echo "🚀 Building optimized WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Check if wasm-opt is installed
if ! command -v wasm-opt &> /dev/null; then
    echo "⚠️  wasm-opt not found. Install for additional optimizations:"
    echo "   npm install -g binaryen"
fi

# Build with wasm-pack
echo "📦 Building with wasm-pack..."
wasm-pack build \
    --target web \
    --out-dir pkg \
    --scope leptos-next-metadata \
    --features wasm \
    --release

# Optimize with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing with wasm-opt..."
    wasm-opt -O4 -s pkg/leptos_next_metadata_bg.wasm -o pkg/leptos_next_metadata_bg.wasm
    echo "✅ wasm-opt optimization complete"
else
    echo "⚠️  Skipping wasm-opt optimization (not installed)"
fi

# Generate bundle analysis
echo "📊 Generating bundle analysis..."
if [ -f "pkg/leptos_next_metadata_bg.wasm" ]; then
    WASM_SIZE=$(stat -f%z pkg/leptos_next_metadata_bg.wasm 2>/dev/null || stat -c%s pkg/leptos_next_metadata_bg.wasm 2>/dev/null || echo "unknown")
    echo "📦 WASM bundle size: $WASM_SIZE bytes"

    if [ "$WASM_SIZE" != "unknown" ] && [ "$WASM_SIZE" -gt 1048576 ]; then
        echo "⚠️  Bundle size is larger than 1MB. Consider optimization."
    else
        echo "✅ Bundle size is within acceptable limits"
    fi
fi

echo "🎉 WASM build complete!"
echo "📁 Output directory: pkg/"
echo "🌐 Ready for web deployment"

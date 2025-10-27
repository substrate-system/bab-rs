#!/bin/bash

# Build script for compiling bab_rs to WebAssembly

set -e  # Exit on error

echo "Building bab for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Please install it with: cargo install wasm-pack"
    exit 1
fi

# Parse command line arguments
TARGET="${1:-web}"

# Build for specified target
echo "Building for $TARGET target..."
wasm-pack build --target "$TARGET" --scope substrate-system --features wasm,william3

echo ""
echo "Build complete! Output is in pkg/"
echo ""

if [ "$TARGET" = "web" ]; then
    echo "Usage in browser (ES modules):"
    echo "  import init, { william3_hash, William3HasherWasm } from './pkg/bab.js';"
    echo "  await init();"
    echo "  const hash = william3_hash(new TextEncoder().encode('hello world'));"
elif [ "$TARGET" = "nodejs" ]; then
    echo "Usage in Node.js:"
    echo "  import { william3_hash, William3HasherWasm } from './pkg/bab.js';"
    echo "  const hash = william3_hash(new TextEncoder().encode('hello world'));"
elif [ "$TARGET" = "bundler" ]; then
    echo "Usage with bundlers (webpack, rollup, etc.):"
    echo "  import init, { william3_hash, William3HasherWasm } from '@substrate-system/bab';"
    echo "  await init();"
    echo "  const hash = william3_hash(new TextEncoder().encode('hello world'));"
fi

echo ""
echo "Other available targets: web, nodejs, bundler, no-modules"
echo "Usage: ./build-wasm.sh [target]"

#!/bin/bash
set -e

# Source rustup environment
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "🦀 Building Rust core..."

cd "$(dirname "$0")/../../core"

# Build for macOS (universal binary)
echo "Building for aarch64-apple-darwin..."
cargo build --release --target aarch64-apple-darwin

# x86_64 slice only when the toolchain has it (Homebrew rust ships arm64 only)
if echo "Building for x86_64-apple-darwin..." && cargo build --release --target x86_64-apple-darwin 2>/dev/null; then
    echo "Creating universal binary..."
    lipo -create \
        target/aarch64-apple-darwin/release/libgonhanh_core.a \
        target/x86_64-apple-darwin/release/libgonhanh_core.a \
        -output ../platforms/macos/libgonhanh_core.a
else
    echo "⚠️  x86_64 target missing, building arm64-only"
    cp target/aarch64-apple-darwin/release/libgonhanh_core.a ../platforms/macos/libgonhanh_core.a
fi

echo "✅ Rust core built successfully!"
echo "📦 Output: platforms/macos/libgonhanh_core.a"

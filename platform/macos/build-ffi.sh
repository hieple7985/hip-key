#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BUILD_DIR="$REPO_ROOT/target/release"
MACOS_DIR="$REPO_ROOT/platform/macos"
FRAMEWORK_DIR="$MACOS_DIR/HipKeyInputMethod/Frameworks"

echo "=== Building hip-key FFI library (release) ==="
cd "$REPO_ROOT"
cargo build --release -p hip-key-ffi

echo "=== Creating framework directory ==="
mkdir -p "$FRAMEWORK_DIR"

echo "=== Copying FFI library ==="
cp "$BUILD_DIR/libhip_key_ffi.a" "$FRAMEWORK_DIR/"
cp "$BUILD_DIR/libhip_key_ffi.dylib" "$FRAMEWORK_DIR/"

echo "=== Copying C header ==="
cp "$REPO_ROOT/ffi/hip-key.h" "$FRAMEWORK_DIR/"

echo "=== Bridging header for Swift ==="
cat > "$FRAMEWORK_DIR/HipKeyBridge.h" << 'EOF'
#ifndef HIP_KEY_BRIDGE_H
#define HIP_KEY_BRIDGE_H
#include "hip-key.h"
#endif
EOF

echo ""
echo "Build artifacts ready at: $FRAMEWORK_DIR"
echo ""
echo "Next steps to complete macOS build:"
echo "  1. Open HipKeyInputMethod.xcodeproj in Xcode"
echo "  2. Add libhip_key_ffi.a to 'Link Binary With Libraries'"
echo "  3. Add HipKeyBridge.h as Objective-C Bridging Header"
echo "  4. Build & Run from Xcode"
echo "  5. Enable HipKey in System Preferences > Keyboard > Input Sources"

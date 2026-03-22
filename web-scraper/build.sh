#!/usr/bin/env bash
set -e

BINARY_NAME="web-differ"
OUT_DIR="dist"
mkdir -p "$OUT_DIR"

build_platform() {
  local PLATFORM="$1"
  local TARGET="$2"
  local EXT=""

  case "$PLATFORM" in windows-*) EXT=".exe" ;; esac

  echo "Building $BINARY_NAME for $PLATFORM ($TARGET)..."
  cargo build --release --target "$TARGET"
  cp "target/$TARGET/release/${BINARY_NAME}${EXT}" "$OUT_DIR/${BINARY_NAME}-${PLATFORM}${EXT}"
  echo "  → $OUT_DIR/${BINARY_NAME}-${PLATFORM}${EXT} ($(du -h "$OUT_DIR/${BINARY_NAME}-${PLATFORM}${EXT}" | cut -f1))"
}

PLATFORMS="${@:-darwin-arm64 darwin-amd64}"

for PLATFORM in $PLATFORMS; do
  case "$PLATFORM" in
    darwin-arm64)   build_platform "$PLATFORM" "aarch64-apple-darwin" ;;
    darwin-amd64)   build_platform "$PLATFORM" "x86_64-apple-darwin" ;;
    linux-arm64)    build_platform "$PLATFORM" "aarch64-unknown-linux-gnu" ;;
    linux-amd64)    build_platform "$PLATFORM" "x86_64-unknown-linux-gnu" ;;
    windows-arm64)  build_platform "$PLATFORM" "aarch64-pc-windows-msvc" ;;
    windows-amd64)  build_platform "$PLATFORM" "x86_64-pc-windows-msvc" ;;
    *) echo "Unknown platform: $PLATFORM" ;;
  esac
done

echo ""
echo "Built binaries:"
ls -lh "$OUT_DIR/"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")"/.. && pwd)"
DIST="$ROOT/dist"
rm -rf "$DIST"
mkdir -p "$DIST"

# Build Rust binaries for common targets
for TARGET in x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin; do
  cargo build --manifest-path "$ROOT/aider-cli/Cargo.toml" --release --target "$TARGET"
  BIN="$ROOT/aider-cli/target/$TARGET/release/aider-cli"
  EXT=""
  ARCHIVE="tar czf"
  OUT="aider-cli-$TARGET"
  if [[ "$TARGET" == *windows* ]]; then
    EXT=".exe"
    ARCHIVE="zip"
  fi
  mkdir -p "$DIST/$OUT"
  cp "$BIN$EXT" "$DIST/$OUT/"
  if [[ "$ARCHIVE" == zip ]]; then
    (cd "$DIST/$OUT" && zip "../$OUT.zip" "aider-cli$EXT")
  else
    (cd "$DIST/$OUT" && tar czf "../$OUT.tar.gz" "aider-cli$EXT")
  fi
  rm -rf "$DIST/$OUT"
done

# Build Go binaries for common platforms
for OS in linux windows darwin; do
  EXT=""
  OUT="go-shell-$OS"
  if [[ "$OS" == windows ]]; then
    EXT=".exe"
  fi
  mkdir -p "$DIST/$OUT"
  GOOS=$OS GOARCH=amd64 go build -o "$DIST/$OUT/go-shell$EXT" "$ROOT/go-shell"
  if [[ "$OS" == windows ]]; then
    (cd "$DIST/$OUT" && zip "../$OUT.zip" "go-shell$EXT")
  else
    (cd "$DIST/$OUT" && tar czf "../$OUT.tar.gz" "go-shell$EXT")
  fi
  rm -rf "$DIST/$OUT"
done

# Build Flutter desktop bundles
cd "$ROOT/flutter_app"
for PLATFORM in linux windows macos; do
  flutter build "$PLATFORM" --release
  BDIR="build/$PLATFORM"
  if [[ -d "$BDIR" ]]; then
    if [[ "$PLATFORM" == windows ]]; then
      (cd build && zip -r "$DIST/flutter_app-$PLATFORM.zip" "$PLATFORM")
    else
      (cd build && tar czf "$DIST/flutter_app-$PLATFORM.tar.gz" "$PLATFORM")
    fi
  fi
done

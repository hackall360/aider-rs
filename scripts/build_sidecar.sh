#!/usr/bin/env bash
set -euo pipefail

workspace_dir="$(dirname "$0")/../aider-core"
cd "$workspace_dir"

# Build native release binary
cargo build --release -p sidecar "$@"

# Example cross-compile for Windows (requires appropriate toolchain)
cargo build --release -p sidecar --target x86_64-pc-windows-gnu "$@"

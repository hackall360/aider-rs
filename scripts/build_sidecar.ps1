$workspace = Join-Path $PSScriptRoot "..\aider-core"
Push-Location $workspace

# Build release binary for the current platform
cargo build --release -p sidecar @Args

# Example cross-compile
if ($Args.Length -eq 0) {
    cargo build --release -p sidecar --target x86_64-unknown-linux-gnu
}

Pop-Location

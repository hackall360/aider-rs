# aider-core Workspace

This workspace hosts the Rust implementation for aider's core features and user interface.

## Crates

- `core` – async runtime, HTTP, git, parsing, configuration, persistence and more.
- `cli_tui` – command line interface and terminal UI using `clap`, `ratatui` and `crossterm`.
- `sidecar` – REST service exposing `core` functionality for integration with other languages.

## Building the Sidecar

### Linux / macOS

```bash
./scripts/build_sidecar.sh
```

### Windows

```powershell
powershell -ExecutionPolicy Bypass -File scripts/build_sidecar.ps1
```

Both scripts produce a release build of the sidecar and demonstrate cross-platform compilation targets.

# Contributing to the Project

We welcome contributions in the form of bug reports, feature requests and pull requests (PRs). This document explains how to get started.

## Bug Reports and Feature Requests

Please submit issues through GitHub so they can be discussed and tracked.

## Pull Requests

For small changes, feel free to submit a PR directly. For larger or significant changes, open an issue first to discuss your proposal.

## Licensing

Before contributing a PR, please review our [Individual Contributor License Agreement](https://aider.chat/docs/legal/contributor-agreement.html). All contributors will be asked to complete the agreement as part of the PR process.

## Setting up a Development Environment

The project is implemented in Rust with Go and Dart front‑ends. Each language has its own tooling and tests.

### Rust

1. Install Rust using [rustup](https://rustup.rs/).
2. From the project root run:
   ```
   cd aider-core
   cargo build
   cargo test
   ```
3. Format and lint the code:
   ```
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   ```

### Go

1. Install [Go](https://go.dev/doc/install) (1.21 or newer).
2. From the `go-shell` directory run:
   ```
   go fmt ./...
   go vet ./...
   go test ./...
   ```

### Dart

1. Install the [Dart SDK](https://dart.dev/get-dart) or Flutter.
2. From the `dart_cli` directory run:
   ```
   dart pub get
   dart format .
   dart analyze
   dart test
   ```

## Coding Standards

### Rust
- Use the 2021 edition.
- Code must be formatted with `cargo fmt` and pass `cargo clippy` checks.

### Go
- Use `gofmt` for formatting and ensure code passes `go vet`.

### Dart
- Use `dart format` for formatting and `dart analyze` for static analysis.

Follow these guidelines and run the associated tests before submitting your PR to help keep the project consistent across languages and platforms.


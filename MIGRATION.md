# Migration Plan

This document tracks the effort to retire Python helpers in favor of Rust equivalents.

## Milestones

| Milestone | Status | Notes |
|-----------|--------|-------|
| M1: Rust git helpers replace `GitRepo` | In progress | `git_sidecar` binary lists repository files |
| M2: Rust repo map available, Python map deprecated | Pending | Rust `RepoMap` module exists |
| M3: Command runner & other utilities ported | Pending | |
| M4: Python layer removed | Pending | Final cleanup |

## Compatibility

Set the environment variable `AIDER_USE_RUST=1` to have existing Python modules
invoke Rust sidecars such as `git_sidecar`.  When the variable is not set the
original Python implementations act as a fallback during the transition.

Progress on this migration should be updated here as milestones are completed.

## Building Components

```bash
cargo build --workspace
go build ./go-shell
dart compile exe dart_cli/bin/aider.dart -o build/dart/aider
```

## Test Suite

```bash
cargo test --workspace
go test ./...
dart test
```

## Coverage

Generate coverage reports for each language and compare them across releases to track progress:

```bash
# Rust coverage
cargo llvm-cov --workspace --html --output-dir coverage/rust

# Go coverage
go test ./... -coverprofile=coverage/go.out
go tool cover -html=coverage/go.out -o coverage/go.html

# Dart coverage
dart test --coverage=coverage/dart
format_coverage --lcov --in coverage/dart --out coverage/dart/lcov.info

# Compare coverage across releases
diff -ru coverage/release-1 coverage/release-2
```

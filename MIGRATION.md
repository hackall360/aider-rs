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

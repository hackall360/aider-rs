# Architecture Overview

This project is composed of a Rust core surrounded by Go and Dart front-ends. A sidecar process mediates communication between components and coordinates access to shared resources.

## Components

- **Rust Core** – Implements the primary application logic, model management and shared state. The core is built as a Cargo workspace containing the `core`, `sidecar`, and `cli_tui` crates.
- **Go Front‑end** – Provides a shell interface that embeds the sidecar client and forwards user commands to the Rust core.
- **Dart Front‑end** – Supplies command line and Flutter-based interfaces. These clients communicate with the sidecar to access core functionality.
- **Sidecar** – Runs alongside the front‑ends, translating requests and responses between the user interfaces and the Rust core. All front‑ends talk to the sidecar through local IPC.
- **Shared Resources** – Model files, configuration, and logs are shared across languages. The sidecar manages access to these resources so that the core remains language agnostic.

## Data Flow

```mermaid
flowchart LR
    GoCLI[Go Shell] -->|IPC| Sidecar
    DartCLI[Dart/Flutter] -->|IPC| Sidecar
    Sidecar --> RustCore["Rust Core"]
    RustCore --> Resources[Shared Resources]
```

## Sidecar Interaction Sequence

```mermaid
sequenceDiagram
    participant UI as Front-end (Go/Dart)
    participant SC as Sidecar
    participant RC as Rust Core
    UI->>SC: Request
    SC->>RC: Translate & forward
    RC-->>SC: Result
    SC-->>UI: Response
```

The diagrams above depict how front-ends communicate with the Rust core through the sidecar and how data flows to shared resources.


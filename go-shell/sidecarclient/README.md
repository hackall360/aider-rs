# Sidecar Client

This package provides a Go client for the Rust sidecar using JSON-RPC over HTTP.
Configuration values are loaded via `viper` and can be set through environment
variables or config files:

- `SIDECAR_HTTP` – base URL for HTTP requests (default `http://localhost:8080`)
- `SIDECAR_WS` – base WebSocket URL (default `ws://localhost:8080`)
- `SIDECAR_TOKEN` – optional bearer token for authentication

## Example

```go
package main

import (
    "context"
    "fmt"

    "github.com/aider-rs/go-shell/sidecarclient"
)

func main() {
    client := sidecarclient.New()
    ctx := context.Background()

    out, err := client.Git(ctx, []string{"status"})
    if err != nil { panic(err) }
    fmt.Println(out)

    msg := []sidecarclient.ChatMessage{{Role: "user", Content: "hello"}}
    resp, _ := client.LLMChat(ctx, msg)
    fmt.Println(resp)

    models, _ := client.LLMModels(ctx)
    fmt.Println(models)
}
```

The JSON-RPC API supports the following methods:

- `git` – execute git commands (`{ "args": ["status"] }`)
- `repo_map` – return a repository map
- `llm.chat` – perform a chat completion (`{ "messages": [...] }`)
- `llm.models` – list available LLM models

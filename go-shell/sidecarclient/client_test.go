package sidecarclient

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	retryablehttp "github.com/hashicorp/go-retryablehttp"
)

func TestRepoMap(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/rpc" {
			t.Fatalf("unexpected path: %s", r.URL.Path)
		}
		var req rpcRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("decode: %v", err)
		}
		if req.Method != "repo.map" {
			t.Fatalf("expected method repo.map, got %s", req.Method)
		}
		resp := rpcResponse{Result: json.RawMessage(`"repo data"`)}
		if err := json.NewEncoder(w).Encode(resp); err != nil {
			t.Fatalf("encode: %v", err)
		}
	}))
	defer srv.Close()

	c := &Client{http: retryablehttp.NewClient(), httpURL: srv.URL}
	out, err := c.RepoMap(context.Background())
	if err != nil {
		t.Fatalf("RepoMap: %v", err)
	}
	if out != "repo data" {
		t.Fatalf("expected repo data, got %s", out)
	}
}

func TestCommandEcho(t *testing.T) {
	up := websocket.Upgrader{CheckOrigin: func(r *http.Request) bool { return true }}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/command" {
			t.Fatalf("unexpected path: %s", r.URL.Path)
		}
		conn, err := up.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("upgrade: %v", err)
		}
		defer conn.Close()
		if _, _, err := conn.ReadMessage(); err != nil {
			t.Fatalf("read: %v", err)
		}
		if err := conn.WriteJSON(map[string]interface{}{"type": "stdout", "data": "Hello, World!\n"}); err != nil {
			t.Fatalf("write stdout: %v", err)
		}
		if err := conn.WriteJSON(map[string]interface{}{"type": "exit", "code": 0}); err != nil {
			t.Fatalf("write exit: %v", err)
		}
	}))
	defer srv.Close()

	wsURL := "ws" + strings.TrimPrefix(srv.URL, "http")
	c := &Client{wsURL: wsURL}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	outCh, codeCh, err := c.Command(ctx, "echo", []string{"Hello, World!"})
	if err != nil {
		t.Fatalf("Command: %v", err)
	}
	var out strings.Builder
	for s := range outCh {
		out.WriteString(s)
	}
	code := <-codeCh
	if out.String() != "Hello, World!\n" {
		t.Fatalf("expected output %q, got %q", "Hello, World!\n", out.String())
	}
	if code != 0 {
		t.Fatalf("expected exit code 0, got %d", code)
	}
}

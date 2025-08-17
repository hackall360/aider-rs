package sidecarclient

import (
	"context"
	"os/exec"
	"testing"
	"time"
)

func TestVersionCheck(t *testing.T) {
	cmd := exec.Command("cargo", "run", "-p", "sidecar")
	cmd.Dir = "../../aider-core"
	if err := cmd.Start(); err != nil {
		t.Fatalf("failed to start sidecar: %v", err)
	}
	defer cmd.Process.Kill()
	time.Sleep(30 * time.Second)

	c := New()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	info, err := c.VersionCheck(ctx)
	if err != nil {
		t.Fatalf("VersionCheck failed: %v", err)
	}
	if info.Current == "" {
		t.Fatalf("expected current version, got empty")
	}
}

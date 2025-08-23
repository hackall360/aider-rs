package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestRunSidecarEcho(t *testing.T) {
	dir := t.TempDir()
	scriptPath := filepath.Join(dir, "cli_tui")
	script := "#!/bin/sh\necho \"$@\"\n"
	if err := os.WriteFile(scriptPath, []byte(script), 0o755); err != nil {
		t.Fatalf("write script: %v", err)
	}
	oldPath := os.Getenv("PATH")
	defer os.Setenv("PATH", oldPath)
	os.Setenv("PATH", dir+string(os.PathListSeparator)+oldPath)

	out, err := runSidecar("Hello, World!")
	if err != nil {
		t.Fatalf("runSidecar returned error: %v", err)
	}
	expected := "--message Hello, World!\n"
	if out != expected {
		t.Fatalf("expected %q, got %q", expected, out)
	}
}

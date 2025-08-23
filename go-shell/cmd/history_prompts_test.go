package cmd

import (
	"bytes"
	"strings"
	"testing"
)

func TestHistoryPrompts(t *testing.T) {
	cmd := newHistoryPromptsCmd()
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"--aider-line", "- example entry"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	out := buf.String()
	if !strings.Contains(out, "- example entry") {
		t.Fatalf("expected output to contain aider line, got: %s", out)
	}
	if strings.Contains(out, "{aider_line}") {
		t.Fatalf("placeholder not replaced in output: %s", out)
	}
}

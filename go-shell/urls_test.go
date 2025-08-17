package main

import (
	"net/http"
	"testing"
)

func TestURLs(t *testing.T) {
	urls := []string{
		"https://aider.chat/",
		"https://aider.chat/docs/faq.html#how-can-i-add-all-the-files-to-the-chat",
		"https://aider.chat/docs/troubleshooting/edit-errors.html",
		"https://aider.chat/docs/git.html",
		"https://aider.chat/docs/install/optional.html#enable-playwright",
		"https://aider.chat/assets/icons/favicon-32x32.png",
		"https://aider.chat/docs/llms/warnings.html",
		"https://aider.chat/docs/troubleshooting/token-limits.html",
		"https://aider.chat/docs/llms.html",
		"https://aider.chat/docs/faq.html#can-i-use-aider-in-a-large-mono-repo",
		"https://github.com/Aider-AI/aider/issues/new",
		"https://github.com/Aider-AI/aider/issues/211",
		"https://aider.chat/docs/troubleshooting/imports.html",
		"https://aider.chat/docs/more/analytics.html",
		"https://aider.chat/HISTORY.html#release-notes",
		"https://aider.chat/docs/more/edit-formats.html",
		"https://aider.chat/docs/troubleshooting/models-and-keys.html",
	}
	for _, url := range urls {
		resp, err := http.Get(url)
		if err != nil {
			t.Fatalf("GET %s: %v", url, err)
		}
		resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Fatalf("url %s returned %d", url, resp.StatusCode)
		}
	}
}

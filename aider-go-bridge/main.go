package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"

	"golang.org/x/time/rate"
)

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type CompleteRequest struct {
	Model       string    `json:"model"`
	Messages    []Message `json:"messages"`
	Tools       any       `json:"tools,omitempty"`
	MaxTokens   int       `json:"max_tokens,omitempty"`
	Temperature float32   `json:"temperature,omitempty"`
}

type Backend interface {
	Complete(ctx context.Context, req CompleteRequest, w http.ResponseWriter) error
}

type EchoBackend struct{}

func (e *EchoBackend) Complete(ctx context.Context, req CompleteRequest, w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/x-ndjson")
	flusher, ok := w.(http.Flusher)
	if !ok {
		return fmt.Errorf("streaming not supported")
	}
	if len(req.Messages) == 0 {
		return nil
	}
	text := req.Messages[len(req.Messages)-1].Content
	for _, r := range text {
		out, _ := json.Marshal(map[string]string{"token": string(r)})
		fmt.Fprintln(w, string(out))
		flusher.Flush()
		time.Sleep(10 * time.Millisecond)
	}
	out, _ := json.Marshal(map[string]bool{"done": true})
	fmt.Fprintln(w, string(out))
	flusher.Flush()
	return nil
}

type OpenAIBackend struct {
	apiKey string
	client *http.Client
}

func (o *OpenAIBackend) Complete(ctx context.Context, req CompleteRequest, w http.ResponseWriter) error {
	body := map[string]any{
		"model":    req.Model,
		"messages": req.Messages,
		"stream":   true,
	}
	if req.MaxTokens > 0 {
		body["max_tokens"] = req.MaxTokens
	}
	if req.Temperature > 0 {
		body["temperature"] = req.Temperature
	}
	b, _ := json.Marshal(body)
	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost, "https://api.openai.com/v1/chat/completions", bytes.NewReader(b))
	if err != nil {
		return err
	}
	httpReq.Header.Set("Authorization", "Bearer "+o.apiKey)
	httpReq.Header.Set("Content-Type", "application/json")
	resp, err := o.client.Do(httpReq)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		w.WriteHeader(resp.StatusCode)
		io.Copy(w, resp.Body)
		return nil
	}
	w.Header().Set("Content-Type", "application/x-ndjson")
	flusher, ok := w.(http.Flusher)
	if !ok {
		return fmt.Errorf("streaming not supported")
	}
	scanner := bufio.NewScanner(resp.Body)
	for scanner.Scan() {
		line := scanner.Text()
		if !strings.HasPrefix(line, "data:") {
			continue
		}
		data := strings.TrimSpace(strings.TrimPrefix(line, "data:"))
		if data == "[DONE]" {
			break
		}
		var v struct {
			Choices []struct {
				Delta struct {
					Content string `json:"content"`
				} `json:"delta"`
			} `json:"choices"`
		}
		if err := json.Unmarshal([]byte(data), &v); err != nil {
			continue
		}
		if len(v.Choices) > 0 {
			token := v.Choices[0].Delta.Content
			if token != "" {
				out, _ := json.Marshal(map[string]string{"token": token})
				w.Write(out)
				w.Write([]byte("\n"))
				flusher.Flush()
			}
		}
	}
	out, _ := json.Marshal(map[string]bool{"done": true})
	w.Write(out)
	w.Write([]byte("\n"))
	flusher.Flush()
	return nil
}

func newBackend() Backend {
	if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		return &OpenAIBackend{apiKey: key, client: &http.Client{}}
	}
	return &EchoBackend{}
}

var (
	backend  Backend
	limiters sync.Map // map[string]*rate.Limiter
)

func getLimiter(model string) *rate.Limiter {
	if v, ok := limiters.Load(model); ok {
		return v.(*rate.Limiter)
	}
	envName := "RATE_LIMIT_" + strings.ToUpper(strings.ReplaceAll(model, "-", "_"))
	perMin := 60
	if s := os.Getenv(envName); s != "" {
		if i, err := strconv.Atoi(s); err == nil && i > 0 {
			perMin = i
		}
	}
	l := rate.NewLimiter(rate.Every(time.Minute/time.Duration(perMin)), perMin)
	limiters.Store(model, l)
	return l
}

func completeHandler(w http.ResponseWriter, r *http.Request) {
	var req CompleteRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	if req.Model == "" {
		http.Error(w, "model required", http.StatusBadRequest)
		return
	}
	if !getLimiter(req.Model).Allow() {
		http.Error(w, "rate limit exceeded", http.StatusTooManyRequests)
		return
	}
	if err := backend.Complete(r.Context(), req, w); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func healthHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("{\"status\":\"ok\"}"))
}

func main() {
	backend = newBackend()
	http.HandleFunc("/complete", completeHandler)
	http.HandleFunc("/health", healthHandler)
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	log.Printf("listening on :%s", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}

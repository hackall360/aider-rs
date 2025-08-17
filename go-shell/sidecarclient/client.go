package sidecarclient

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gorilla/websocket"
	retryablehttp "github.com/hashicorp/go-retryablehttp"
	"github.com/spf13/viper"
)

type Client struct {
	http    *retryablehttp.Client
	httpURL string
	wsURL   string
	token   string
}

func New() *Client {
	viper.AutomaticEnv()
	c := &Client{
		http:    retryablehttp.NewClient(),
		httpURL: viper.GetString("SIDECAR_HTTP"),
		wsURL:   viper.GetString("SIDECAR_WS"),
		token:   viper.GetString("SIDECAR_TOKEN"),
	}
	if c.httpURL == "" {
		c.httpURL = "http://localhost:8080"
	}
	if c.wsURL == "" {
		c.wsURL = "ws://localhost:8080"
	}
	return c
}

type rpcRequest struct {
	Method string      `json:"method"`
	Params interface{} `json:"params"`
}

type rpcResponse struct {
	Result json.RawMessage `json:"result"`
	Error  string          `json:"error"`
}

func (c *Client) call(ctx context.Context, method string, params interface{}, result interface{}) error {
	reqObj := rpcRequest{Method: method, Params: params}
	body, err := json.Marshal(reqObj)
	if err != nil {
		return err
	}
	req, err := retryablehttp.NewRequest(http.MethodPost, c.httpURL+"/rpc", bytes.NewReader(body))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	if c.token != "" {
		req.Header.Set("Authorization", "Bearer "+c.token)
	}
	resp, err := c.http.Do(req.WithContext(ctx))
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	var respObj rpcResponse
	if err := json.NewDecoder(resp.Body).Decode(&respObj); err != nil {
		return err
	}
	if respObj.Error != "" {
		return fmt.Errorf("%s", respObj.Error)
	}
	if result != nil {
		return json.Unmarshal(respObj.Result, result)
	}
	return nil
}

func (c *Client) Git(ctx context.Context, args []string) (string, error) {
	var out string
	err := c.call(ctx, "git", map[string]interface{}{"args": args}, &out)
	return out, err
}

func (c *Client) RepoMap(ctx context.Context) (string, error) {
        var out string
        err := c.call(ctx, "repo.map", nil, &out)
        return out, err
}

func (c *Client) RepoWatch(ctx context.Context) ([]string, error) {
        var out []string
        err := c.call(ctx, "repo.watch", nil, &out)
        return out, err
}

type ChatMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type Model struct {
	ID      string             `json:"id"`
	Pricing map[string]float64 `json:"pricing"`
}

type VersionInfo struct {
	Current      string `json:"current"`
	Latest       string `json:"latest"`
	URL          string `json:"url"`
	Instructions string `json:"instructions"`
}

func (c *Client) LLMChat(ctx context.Context, msgs []ChatMessage) (string, error) {
	var out string
	params := map[string]interface{}{"messages": msgs}
	err := c.call(ctx, "llm.chat", params, &out)
	return out, err
}

func (c *Client) LLMModels(ctx context.Context) ([]Model, error) {
	var out []Model
	err := c.call(ctx, "llm.models", nil, &out)
	return out, err
}

func (c *Client) ScrapeURL(ctx context.Context, url string) (string, error) {
	var out string
	params := map[string]interface{}{"url": url}
	err := c.call(ctx, "scrape.url", params, &out)
	return out, err
}

func (c *Client) CoderSearchReplace(ctx context.Context, content, search, replace string) (string, error) {
	var out string
	params := map[string]interface{}{"content": content, "search": search, "replace": replace}
	err := c.call(ctx, "coder.search_replace", params, &out)
	return out, err
}

func (c *Client) VoiceRecord(ctx context.Context) (string, error) {
	var out string
	err := c.call(ctx, "voice.record", nil, &out)
	return out, err
}

func (c *Client) AnalyticsEvent(ctx context.Context, event string, props map[string]interface{}) error {
	params := map[string]interface{}{"event": event, "properties": props}
	return c.call(ctx, "analytics_event", params, nil)
}

func (c *Client) VersionCheck(ctx context.Context) (VersionInfo, error) {
	var out VersionInfo
	err := c.call(ctx, "version.check", nil, &out)
	return out, err
}

func (c *Client) DialWS(ctx context.Context, path string) (*websocket.Conn, *http.Response, error) {
	url := c.wsURL + path
	h := http.Header{}
	if c.token != "" {
		h.Set("Authorization", "Bearer "+c.token)
	}
	return websocket.DefaultDialer.DialContext(ctx, url, h)
}

func (c *Client) Command(ctx context.Context, cmd string, args []string) (<-chan string, <-chan int, error) {
	conn, _, err := c.DialWS(ctx, "/command")
	if err != nil {
		return nil, nil, err
	}
	req := map[string]interface{}{"cmd": cmd, "args": args}
	body, err := json.Marshal(req)
	if err != nil {
		return nil, nil, err
	}
	if err := conn.WriteMessage(websocket.TextMessage, body); err != nil {
		return nil, nil, err
	}
	outCh := make(chan string)
	codeCh := make(chan int, 1)
	go func() {
		defer close(outCh)
		defer close(codeCh)
		for {
			_, msg, err := conn.ReadMessage()
			if err != nil {
				return
			}
			var resp map[string]interface{}
			if err := json.Unmarshal(msg, &resp); err != nil {
				continue
			}
			switch resp["type"] {
			case "stdout", "stderr":
				if s, ok := resp["data"].(string); ok {
					outCh <- s
				}
			case "exit":
				if v, ok := resp["code"].(float64); ok {
					codeCh <- int(v)
				}
				return
			}
		}
	}()
	return outCh, codeCh, nil
}

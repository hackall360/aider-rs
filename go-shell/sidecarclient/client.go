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
	err := c.call(ctx, "repo_map", nil, &out)
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

func (c *Client) AnalyticsEvent(ctx context.Context, event string, props map[string]interface{}) error {
	params := map[string]interface{}{"event": event, "properties": props}
	return c.call(ctx, "analytics_event", params, nil)
}

func (c *Client) DialWS(ctx context.Context, path string) (*websocket.Conn, *http.Response, error) {
	url := c.wsURL + path
	h := http.Header{}
	if c.token != "" {
		h.Set("Authorization", "Bearer "+c.token)
	}
	return websocket.DefaultDialer.DialContext(ctx, url, h)
}

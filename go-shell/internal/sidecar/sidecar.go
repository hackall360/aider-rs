package sidecar

import (
	"context"
	"io"

	"github.com/gorilla/websocket"
	retryablehttp "github.com/hashicorp/go-retryablehttp"
	"github.com/spf13/viper"
)

func Ping(ctx context.Context) (string, error) {
	url := viper.GetString("sidecar_http")
	if url == "" {
		url = "http://localhost:3030/ping"
	}
	req, err := retryablehttp.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}
	resp, err := retryablehttp.NewClient().Do(req.WithContext(ctx))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}
	return string(body), nil
}

func DialWS(ctx context.Context) (*websocket.Conn, error) {
	url := viper.GetString("sidecar_ws")
	if url == "" {
		url = "ws://localhost:3030/ws"
	}
	d := websocket.Dialer{}
	conn, _, err := d.DialContext(ctx, url, nil)
	return conn, err
}

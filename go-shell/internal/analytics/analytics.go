package analytics

import (
	"context"

	sc "github.com/aider-rs/go-shell/sidecarclient"
)

type Client struct {
	sc *sc.Client
}

func New(scClient *sc.Client) *Client {
	return &Client{sc: scClient}
}

func (c *Client) Event(ctx context.Context, event string, props map[string]interface{}) error {
	return c.sc.AnalyticsEvent(ctx, event, props)
}

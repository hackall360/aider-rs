package cmd

import (
	"context"
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"go.uber.org/zap"

	"github.com/aider-rs/go-shell/internal/config"
	"github.com/aider-rs/go-shell/internal/resources"
	"github.com/aider-rs/go-shell/internal/sidecar"
	"github.com/aider-rs/go-shell/internal/tui"
)

var rootCmd = &cobra.Command{
	Use:   "go-shell",
	Short: "Go-based shell for interacting with aider",
	RunE: func(cmd *cobra.Command, args []string) error {
		db, logger, err := config.Init()
		if err != nil {
			return err
		}
		defer db.Close()
		defer logger.Sync()

		// Demonstrate loading shared resources.
		if _, err := resources.LoadJSON("../resources/model-metadata.json"); err != nil {
			logger.Debug("resource load", zap.Error(err))
		}

		ctx := context.Background()
		if msg, err := sidecar.Ping(ctx); err == nil {
			logger.Info("sidecar", zap.String("msg", msg))
		} else {
			logger.Warn("sidecar ping failed", zap.Error(err))
		}

		if err := tui.New().Start(); err != nil {
			return err
		}
		return nil
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().String("config", "", "config file")
	viper.BindPFlag("config", rootCmd.PersistentFlags().Lookup("config"))
}

package config

import (
	"errors"
	"os"
	"path/filepath"
	"time"

	"github.com/spf13/viper"
	"go.etcd.io/bbolt"
	"go.uber.org/zap"
)

func Init() (*bbolt.DB, *zap.Logger, error) {
	cfgDir, err := os.UserConfigDir()
	if err != nil {
		return nil, nil, err
	}
	cfgDir = filepath.Join(cfgDir, "go-shell")
	if err := os.MkdirAll(cfgDir, 0o755); err != nil {
		return nil, nil, err
	}

	viper.AddConfigPath(cfgDir)
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	if err := viper.SafeWriteConfig(); err != nil {
		var pathErr *os.PathError
		if !errors.As(err, &pathErr) {
			return nil, nil, err
		}
	}
	if err := viper.ReadInConfig(); err != nil {
		// ignore missing config file
	}

	dbPath := filepath.Join(cfgDir, "session.db")
	db, err := bbolt.Open(dbPath, 0o600, nil)
	if err != nil {
		return nil, nil, err
	}
	if err := db.Update(func(tx *bbolt.Tx) error {
		b, err := tx.CreateBucketIfNotExists([]byte("session"))
		if err != nil {
			return err
		}
		return b.Put([]byte("last_run"), []byte(time.Now().Format(time.RFC3339)))
	}); err != nil {
		db.Close()
		return nil, nil, err
	}

	logger, err := zap.NewProduction()
	if err != nil {
		db.Close()
		return nil, nil, err
	}
	return db, logger, nil
}

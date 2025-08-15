package resources

import (
	"encoding/json"
	"os"

	toml "github.com/pelletier/go-toml/v2"
	"gopkg.in/yaml.v3"
)

// LoadJSON reads a JSON file from disk.
func LoadJSON(path string) (map[string]any, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var v map[string]any
	if err := json.Unmarshal(b, &v); err != nil {
		return nil, err
	}
	return v, nil
}

// LoadYAML reads a YAML file from disk.
func LoadYAML(path string) (map[string]any, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var v map[string]any
	if err := yaml.Unmarshal(b, &v); err != nil {
		return nil, err
	}
	return v, nil
}

// LoadTOML reads a TOML file from disk.
func LoadTOML(path string) (map[string]any, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var v map[string]any
	if err := toml.Unmarshal(b, &v); err != nil {
		return nil, err
	}
	return v, nil
}

// LoadPrompt reads a plain text prompt template.
func LoadPrompt(path string) (string, error) {
	b, err := os.ReadFile(path)
	return string(b), err
}

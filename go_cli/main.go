package main

import (
	"os"
	"os/exec"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/spf13/cobra"
)

// model holds output from the sidecar binary.
type model struct {
	output string
}

func (m model) Init() tea.Cmd                           { return nil }
func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) { return m, tea.Quit }
func (m model) View() string                            { return m.output + "\n" }

func runSidecar(message string) (string, error) {
	cmd := exec.Command("cli_tui", "--message", message)
	out, err := cmd.CombinedOutput()
	return string(out), err
}

func main() {
	var message string
	rootCmd := &cobra.Command{
		Use:   "go-cli",
		Short: "Go front-end for cli_tui",
		RunE: func(cmd *cobra.Command, args []string) error {
			out, err := runSidecar(message)
			if err != nil {
				return err
			}
			p := tea.NewProgram(model{output: out})
			_, err = p.Run()
			return err
		},
	}
	rootCmd.Flags().StringVarP(&message, "message", "m", "Hello from Go", "Message to display")
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

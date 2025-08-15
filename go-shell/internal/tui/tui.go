package tui

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/glamour"
	"github.com/charmbracelet/lipgloss"
)

type model struct {
	content string
}

func New() *tea.Program {
	m := model{content: "# go-shell\nPress q to quit."}
	return tea.NewProgram(m)
}

func (m model) Init() tea.Cmd { return nil }

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		if msg.Type == tea.KeyCtrlC || msg.String() == "q" {
			return m, tea.Quit
		}
	}
	return m, nil
}

func (m model) View() string {
	r, _ := glamour.Render(m.content, "dark")
	style := lipgloss.NewStyle().Margin(1, 2)
	return style.Render(r)
}

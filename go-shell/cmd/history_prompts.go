package cmd

import (
	"fmt"
	"strings"

	"github.com/spf13/cobra"
)

const historyPromptTemplate = `Update the history markdown doc with changes shown in the diffs.
Succinctly describe actual user-facing changes, not every single commit or detail that was made implementing them.

Only add new items not already listed in the history markdown.
Do NOT edit or update existing history entries.
Do NOT add duplicate entries for changes that have existing history entries.
Do NOT add additional entries for small tweaks to features which are already listed in the existing history.

Pay attention to see if changes are later modified or superseded in the commit logs.
The history doc should only reflect the *final* version of changes which have evolved within a version's commit history.
If the history doc already describes the final behavior, don't document the changes that led us there.

Bullet each item at the start of the line with ` + "`-`" + `.
End each bullet with a period.

If the change was made by someone other than Paul Gauthier note it at the end of the bullet point as ", by XXX."

Be sure to attribute changes to the proper .x version.
Changes in the .x-dev version should be listed under a "### main branch" heading

Start a new "### main branch" section at the top of the file if needed.

Also, add this as the last bullet under the "### main branch" section, replacing an existing version if present:
{aider_line}`

func newHistoryPromptsCmd() *cobra.Command {
	var aiderLine string
	cmd := &cobra.Command{
		Use:   "history-prompts",
		Short: "Print the history update prompt",
		RunE: func(cmd *cobra.Command, args []string) error {
			prompt := strings.ReplaceAll(historyPromptTemplate, "{aider_line}", aiderLine)
			fmt.Fprint(cmd.OutOrStdout(), prompt)
			return nil
		},
	}
	cmd.Flags().StringVar(&aiderLine, "aider-line", "", "Line to add under main branch")
	cmd.MarkFlagRequired("aider-line")
	return cmd
}

var historyPromptsCmd = newHistoryPromptsCmd()

func init() {
	rootCmd.AddCommand(historyPromptsCmd)
}

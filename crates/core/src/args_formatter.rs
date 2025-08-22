/// Utilities for generating example configuration text used by the CLI help.
///
/// The original Python implementation exposed custom `argparse` formatters to
/// emit sample `.env` and YAML configuration files.  For the Rust port we simply
/// provide helper functions that return those strings so they can be written to
/// stdout when needed.

pub fn sample_dotenv() -> String {
    r#"##########################################################
# Sample aider .env file.
# Place at the root of your git repo.
# Or use `aider --env <fname>` to specify.
##########################################################

#################
# LLM parameters:
#
# Include xxx_API_KEY parameters and other params needed for your LLMs.
# See https://aider.chat/docs/llms for details.
#
## OpenAI
#OPENAI_API_KEY=

## Anthropic
#ANTHROPIC_API_KEY=

##...
"#
    .to_string()
}

pub fn sample_yaml_config() -> String {
    r#"##########################################################
# Sample .aider.conf.yml
# This file lists *all* the valid configuration entries.
# Place in your home dir, or at the root of your git repo.
##########################################################

# Note: You can only put OpenAI and Anthropic API keys in the YAML
# config file. Keys for all APIs can be stored in a .env file
# https://aider.chat/docs/config/dotenv.html
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotenv_contains_heading() {
        let text = sample_dotenv();
        assert!(text.contains("Sample aider .env"));
    }
}

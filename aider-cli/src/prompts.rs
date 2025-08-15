use anyhow::Result;
use tera::{Context, Tera};

/// Template handling using `tera`, replacing the Python `prompts.py` module.
pub struct Prompts {
    tera: Tera,
}

impl Default for Prompts {
    fn default() -> Self {
        Self { tera: Tera::default() }
    }
}

impl Prompts {
    /// Load templates from the provided glob pattern.
    pub fn new(glob: &str) -> Result<Self> {
        Ok(Self { tera: Tera::new(glob)? })
    }

    /// Render a template string with the supplied context.
    pub fn render_str(&mut self, template: &str, ctx: &Context) -> Result<String> {
        Ok(self.tera.render_str(template, ctx)?)
    }
}


use anyhow::Result;
use tera::{Context, Tera};

/// Template handling using `tera`, replacing the Python `prompts.py` module.
#[derive(Default)]
pub struct Prompts {
    tera: Tera,
}

impl Prompts {
    /// Load templates from the provided glob pattern.
    #[allow(dead_code)]
    pub fn new(glob: &str) -> Result<Self> {
        Ok(Self {
            tera: Tera::new(glob)?,
        })
    }

    /// Render a template string with the supplied context.
    pub fn render_str(&mut self, template: &str, ctx: &Context) -> Result<String> {
        Ok(self.tera.render_str(template, ctx)?)
    }
}

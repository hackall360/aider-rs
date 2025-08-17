use anyhow::Result;
use tera::{Context, Tera};

/// Template handling using `tera`, replacing the Python `prompts.py` module.
pub struct Prompts {
    tera: Tera,
}

impl Prompts {
    /// Load templates from the `resources/templates` directory.
    pub fn new() -> Result<Self> {
        Ok(Self {
            tera: Tera::new("resources/templates/**/*")?,
        })
    }

    /// Render a named template file with the supplied context.
    pub fn render(&self, name: &str, ctx: &Context) -> Result<String> {
        Ok(self.tera.render(name, ctx)?)
    }

    /// Render a raw template string with the supplied context.
    pub fn render_str(&mut self, template: &str, ctx: &Context) -> Result<String> {
        Ok(self.tera.render_str(template, ctx)?)
    }
}

use anyhow::Result;
use serde::Serialize;
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

    /// Render a named template using any serializable data as context.
    pub fn render_with<T: Serialize>(&self, name: &str, data: &T) -> Result<String> {
        let ctx = Context::from_serialize(data)?;
        self.render(name, &ctx)
    }

    /// Render a raw template string with the supplied context.
    pub fn render_str(&mut self, template: &str, ctx: &Context) -> Result<String> {
        Ok(self.tera.render_str(template, ctx)?)
    }

    /// Render a raw template string using any serializable data.
    pub fn render_str_with<T: Serialize>(&mut self, template: &str, data: &T) -> Result<String> {
        let ctx = Context::from_serialize(data)?;
        self.render_str(template, &ctx)
    }
}

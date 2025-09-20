//! Template rendering for OG image generation
//!
//! This module handles loading and rendering SVG templates using Liquid templating.

use crate::{Error, Result};

#[cfg(feature = "og-images")]
use liquid::Object;

impl super::OgImageGenerator {
    #[cfg(feature = "og-images")]
    /// Load template from file or embedded template
    pub async fn load_template(&self, template_name: &str) -> Result<String> {
        let template = match template_name {
            "simple" => include_str!("../../templates/simple.svg"),
            _ => {
                return Err(Error::TemplateError(format!(
                    "Template '{}' not found",
                    template_name
                )))
            }
        };

        Ok(template.to_string())
    }

    #[cfg(feature = "og-images")]
    /// Render template with data using Liquid
    pub async fn render_template(&self, template: &str, data: &Object) -> Result<String> {
        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|e| Error::TemplateError(format!("Failed to create Liquid parser: {}", e)))?;

        let template = parser
            .parse(template)
            .map_err(|e| Error::TemplateError(format!("Failed to parse template: {}", e)))?;

        let rendered = template
            .render(data)
            .map_err(|e| Error::TemplateError(format!("Failed to render template: {}", e)))?;

        Ok(rendered)
    }
}

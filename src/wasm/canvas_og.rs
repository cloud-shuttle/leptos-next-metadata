//! Canvas-based OG image generation for WASM
//!
//! Provides client-side Open Graph image generation using HTML5 Canvas

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement};

// Re-export shared types
pub use crate::canvas_types::{
    CanvasOgParams, GradientType, ImageLayer, LogoPosition, OgImageLayer, OgImageTemplate,
    ShapeLayer, ShapeType, TextAlign, TextGradient, TextLayer, TextOutline, TextShadow,
};

/// Canvas-based OG image generator
#[derive(Debug, Clone)]
pub struct CanvasOgGenerator {
    /// Canvas element
    canvas: Option<HtmlCanvasElement>,
    /// Rendering context
    context: Option<CanvasRenderingContext2d>,
    /// Default dimensions
    width: u32,
    height: u32,
    /// Background color
    background_color: String,
    /// Text color
    text_color: String,
    /// Font family
    font_family: String,
}

/// Generated canvas image result
#[derive(Debug, Clone)]
pub struct CanvasOgResult {
    /// Base64 encoded image data
    pub data_url: String,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// MIME type
    pub mime_type: String,
}

impl Default for CanvasOgGenerator {
    fn default() -> Self {
        Self {
            canvas: None,
            context: None,
            width: 1200,
            height: 630,
            background_color: "#1a1a1a".to_string(),
            text_color: "#ffffff".to_string(),
            font_family: "system-ui, -apple-system, sans-serif".to_string(),
        }
    }
}

impl CanvasOgGenerator {
    /// Create a new canvas OG generator
    pub fn new() -> Result<Self, JsValue> {
        let mut generator = Self::default();
        generator.initialize_canvas()?;
        Ok(generator)
    }

    /// Create with custom dimensions
    pub fn with_dimensions(width: u32, height: u32) -> Result<Self, JsValue> {
        let mut generator = Self {
            width,
            height,
            ..Default::default()
        };
        generator.initialize_canvas()?;
        Ok(generator)
    }

    /// Initialize canvas and context
    fn initialize_canvas(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;

        // Create canvas element
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        // Set canvas dimensions
        canvas.set_width(self.width);
        canvas.set_height(self.height);

        // Get 2D context
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2D context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        self.canvas = Some(canvas);
        self.context = Some(context);

        Ok(())
    }

    /// Generate OG image from parameters
    pub fn generate(&mut self, params: CanvasOgParams) -> Result<CanvasOgResult, JsValue> {
        let canvas = self.canvas.as_ref().ok_or("Canvas not initialized")?;
        let context = self.context.as_ref().ok_or("Context not initialized")?;

        // Set dimensions
        let width = params.width.unwrap_or(self.width);
        let height = params.height.unwrap_or(self.height);
        canvas.set_width(width);
        canvas.set_height(height);

        // Clear canvas
        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        // Draw background
        let bg_color = params
            .background_color
            .as_ref()
            .unwrap_or(&self.background_color);
        context.set_fill_style(&bg_color.into());
        context.fill_rect(0.0, 0.0, width as f64, height as f64);

        // Draw logo if provided
        if let Some(logo_url) = &params.logo_url {
            self.draw_logo(context, logo_url, &params.logo_position, width, height)?;
        }

        // Draw text content
        self.draw_text(context, &params, width, height)?;

        // Export as data URL
        let data_url = canvas.to_data_url_with_type("image/png")?;

        Ok(CanvasOgResult {
            data_url,
            width,
            height,
            mime_type: "image/png".to_string(),
        })
    }

    /// Draw logo on canvas
    fn draw_logo(
        &self,
        context: &CanvasRenderingContext2d,
        _logo_url: &str,
        position: &Option<LogoPosition>,
        width: u32,
        height: u32,
    ) -> Result<(), JsValue> {
        // For now, we'll create a placeholder logo
        // In a real implementation, you'd load the image from URL
        let logo_size = 80.0;
        let position = position.as_ref().unwrap_or(&LogoPosition::TopLeft);

        let (x, y) = match position {
            LogoPosition::TopLeft => (20.0, 20.0),
            LogoPosition::TopRight => (width as f64 - logo_size - 20.0, 20.0),
            LogoPosition::BottomLeft => (20.0, height as f64 - logo_size - 20.0),
            LogoPosition::BottomRight => (
                width as f64 - logo_size - 20.0,
                height as f64 - logo_size - 20.0,
            ),
            LogoPosition::Center => (
                (width as f64 - logo_size) / 2.0,
                (height as f64 - logo_size) / 2.0,
            ),
        };

        // Draw placeholder logo (colored rectangle)
        context.set_fill_style(&"#4f46e5".into());
        context.fill_rect(x, y, logo_size, logo_size);

        // Add logo text
        context.set_fill_style(&"#ffffff".into());
        context.set_font("16px system-ui, sans-serif");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text("LOGO", x + logo_size / 2.0, y + logo_size / 2.0)?;

        Ok(())
    }

    /// Draw text content on canvas
    fn draw_text(
        &self,
        context: &CanvasRenderingContext2d,
        params: &CanvasOgParams,
        width: u32,
        _height: u32,
    ) -> Result<(), JsValue> {
        let padding = params.padding.unwrap_or(40) as f64;
        let text_color = params.text_color.as_ref().unwrap_or(&self.text_color);
        let font_family = params.font_family.as_ref().unwrap_or(&self.font_family);

        context.set_fill_style(&text_color.into());

        // Calculate text area
        let text_width = width as f64 - (padding * 2.0);
        let text_x = match params.text_align.as_ref().unwrap_or(&TextAlign::Left) {
            TextAlign::Left => padding,
            TextAlign::Center => (width as f64 - text_width) / 2.0,
            TextAlign::Right => width as f64 - padding - text_width,
        };

        // Draw title
        let title_font_size = params.title_font_size.unwrap_or(48);
        context.set_font(&format!("{}px {}", title_font_size, font_family));
        context.set_text_align("left");
        context.set_text_baseline("top");

        let title_y = padding + 100.0; // Leave space for logo
        self.draw_wrapped_text(
            context,
            &params.title,
            text_x,
            title_y,
            text_width,
            title_font_size as f64,
        )?;

        // Draw description if provided
        if let Some(description) = &params.description {
            let desc_font_size = params.description_font_size.unwrap_or(24);
            context.set_font(&format!("{}px {}", desc_font_size, font_family));

            // Calculate description Y position (after title)
            let title_height =
                self.calculate_text_height(&params.title, text_width, title_font_size as f64);
            let desc_y = title_y + title_height + 20.0;

            self.draw_wrapped_text(
                context,
                description,
                text_x,
                desc_y,
                text_width,
                desc_font_size as f64,
            )?;
        }

        Ok(())
    }

    /// Draw wrapped text that fits within specified width
    fn draw_wrapped_text(
        &self,
        context: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
        line_height: f64,
    ) -> Result<(), JsValue> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_y = y;

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            // Estimate text width (simplified approach)
            let estimated_width = test_line.len() as f64 * 0.6 * (line_height * 0.6);
            if estimated_width > max_width && !current_line.is_empty() {
                // Draw current line and start new line
                context.fill_text(&current_line, x, current_y)?;
                current_line = word.to_string();
                current_y += line_height;
            } else {
                current_line = test_line;
            }
        }

        // Draw remaining text
        if !current_line.is_empty() {
            context.fill_text(&current_line, x, current_y)?;
        }

        Ok(())
    }

    /// Calculate text height for wrapped text
    fn calculate_text_height(&self, text: &str, max_width: f64, line_height: f64) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = 1;
        let mut current_line = String::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            // Estimate width (this is approximate)
            let estimated_width = test_line.len() as f64 * 0.6 * (line_height * 0.6);
            if estimated_width > max_width && !current_line.is_empty() {
                lines += 1;
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }

        lines as f64 * line_height
    }

    /// Generate a simple OG image with just title
    pub fn generate_simple(&mut self, title: &str) -> Result<CanvasOgResult, JsValue> {
        let params = CanvasOgParams {
            title: title.to_string(),
            description: None,
            width: None,
            height: None,
            background_color: None,
            text_color: None,
            font_family: None,
            title_font_size: None,
            description_font_size: None,
            logo_url: None,
            logo_position: None,
            text_align: None,
            padding: None,
        };

        self.generate(params)
    }

    /// Generate OG image with title and description
    pub fn generate_with_description(
        &mut self,
        title: &str,
        description: &str,
    ) -> Result<CanvasOgResult, JsValue> {
        let params = CanvasOgParams {
            title: title.to_string(),
            description: Some(description.to_string()),
            width: None,
            height: None,
            background_color: None,
            text_color: None,
            font_family: None,
            title_font_size: None,
            description_font_size: None,
            logo_url: None,
            logo_position: None,
            text_align: None,
            padding: None,
        };

        self.generate(params)
    }

    /// Set custom colors
    pub fn set_colors(&mut self, background: &str, text: &str) {
        self.background_color = background.to_string();
        self.text_color = text.to_string();
    }

    /// Set custom font
    pub fn set_font(&mut self, font_family: &str) {
        self.font_family = font_family.to_string();
    }

    /// Get current dimensions
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Set dimensions
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.width = width;
        self.height = height;

        if let Some(canvas) = &self.canvas {
            canvas.set_width(width);
            canvas.set_height(height);
        }

        Ok(())
    }

    /// Generate an OG image from a predefined template
    pub fn generate_from_template(
        &mut self,
        template: &OgImageTemplate,
        data: HashMap<String, String>,
    ) -> Result<CanvasOgResult, JsValue> {
        let mut params = template.default_params.clone();

        // Apply data to template
        for (key, value) in data {
            match key.as_str() {
                "title" => params.title = value,
                "description" => params.description = Some(value),
                _ => {
                    // Handle custom template variables
                    if let Some(ref mut layers) = params.layers {
                        for layer in layers {
                            match layer {
                                OgImageLayer::Text(text_layer) => {
                                    if text_layer.content.contains(&format!("{{{}}}", key)) {
                                        text_layer.content = text_layer
                                            .content
                                            .replace(&format!("{{{}}}", key), &value);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        self.generate(params)
    }

    /// Load a custom font from a URL
    pub async fn load_font(&self, font_name: &str, font_url: &str) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Create a link element to load the font
                let link = document.create_element("link")?;
                link.set_attribute("rel", "stylesheet")?;
                link.set_attribute("href", font_url)?;

                // Add to document head
                if let Some(head) = document.head() {
                    head.append_child(&link)?;
                }
            }
        }
        Ok(())
    }

    /// Render a single layer onto the canvas
    fn render_layer(
        &self,
        context: &CanvasRenderingContext2d,
        layer: &OgImageLayer,
    ) -> Result<(), JsValue> {
        match layer {
            OgImageLayer::Text(text_layer) => self.render_text_layer(context, text_layer),
            OgImageLayer::Image(image_layer) => self.render_image_layer(context, image_layer),
            OgImageLayer::Shape(shape_layer) => self.render_shape_layer(context, shape_layer),
        }
    }

    /// Render a text layer with advanced effects
    fn render_text_layer(
        &self,
        context: &CanvasRenderingContext2d,
        layer: &TextLayer,
    ) -> Result<(), JsValue> {
        // Set font
        context.set_font(&format!("{}px {}", layer.font_size, layer.font_family));

        // Apply text shadow if specified
        if let Some(shadow) = &layer.shadow {
            context.set_shadow_color(&shadow.color);
            context.set_shadow_blur(shadow.blur);
            context.set_shadow_offset_x(shadow.offset_x);
            context.set_shadow_offset_y(shadow.offset_y);
        }

        // Apply text gradient if specified
        if let Some(gradient) = &layer.gradient {
            let canvas_gradient = match gradient.gradient_type {
                GradientType::Linear => context.create_linear_gradient(
                    gradient.start_x,
                    gradient.start_y,
                    gradient.end_x,
                    gradient.end_y,
                ),
                GradientType::Radial => context.create_radial_gradient(
                    gradient.start_x,
                    gradient.start_y,
                    0.0,
                    gradient.end_x,
                    gradient.end_y,
                    100.0,
                ),
            };

            // Add color stops
            for (i, color) in gradient.colors.iter().enumerate() {
                let stop = i as f64 / (gradient.colors.len() - 1) as f64;
                canvas_gradient.add_color_stop(stop, color)?;
            }

            context.set_fill_style(&canvas_gradient.into());
        } else {
            context.set_fill_style(&layer.color.into());
        }

        // Draw text with outline if specified
        if let Some(outline) = &layer.outline {
            context.set_stroke_style(&outline.color.into());
            context.set_line_width(outline.width);
            context.stroke_text(&layer.content, layer.x, layer.y)?;
        }

        // Draw filled text
        context.fill_text(&layer.content, layer.x, layer.y)?;

        // Reset shadow
        context.set_shadow_color("transparent");
        context.set_shadow_blur(0.0);
        context.set_shadow_offset_x(0.0);
        context.set_shadow_offset_y(0.0);

        Ok(())
    }

    /// Render an image layer
    fn render_image_layer(
        &self,
        context: &CanvasRenderingContext2d,
        layer: &ImageLayer,
    ) -> Result<(), JsValue> {
        // Set global alpha for opacity
        context.set_global_alpha(layer.opacity);

        // Set blend mode if specified
        if let Some(blend_mode) = &layer.blend_mode {
            context.set_global_composite_operation(blend_mode)?;
        }

        // For now, draw a placeholder rectangle
        // In a real implementation, you would load and draw the actual image
        context.set_fill_style(&"#cccccc".into());
        context.fill_rect(layer.x, layer.y, layer.width, layer.height);

        // Reset composite operation and alpha
        context.set_global_composite_operation("source-over")?;
        context.set_global_alpha(1.0);

        Ok(())
    }

    /// Render a shape layer
    fn render_shape_layer(
        &self,
        context: &CanvasRenderingContext2d,
        layer: &ShapeLayer,
    ) -> Result<(), JsValue> {
        match &layer.shape_type {
            ShapeType::Rectangle => {
                if let Some(fill_color) = &layer.fill_color {
                    context.set_fill_style(&fill_color.into());
                    context.fill_rect(layer.x, layer.y, layer.width, layer.height);
                }

                if let Some(stroke_color) = &layer.stroke_color {
                    context.set_stroke_style(&stroke_color.into());
                    if let Some(stroke_width) = layer.stroke_width {
                        context.set_line_width(stroke_width);
                    }
                    context.stroke_rect(layer.x, layer.y, layer.width, layer.height);
                }
            }
            ShapeType::Circle => {
                context.begin_path();
                let center_x = layer.x + layer.width / 2.0;
                let center_y = layer.y + layer.height / 2.0;
                let radius = (layer.width.min(layer.height)) / 2.0;
                context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)?;

                if let Some(fill_color) = &layer.fill_color {
                    context.set_fill_style(&fill_color.into());
                    context.fill();
                }

                if let Some(stroke_color) = &layer.stroke_color {
                    context.set_stroke_style(&stroke_color.into());
                    if let Some(stroke_width) = layer.stroke_width {
                        context.set_line_width(stroke_width);
                    }
                    context.stroke();
                }
            }
            ShapeType::Line { x2, y2 } => {
                context.begin_path();
                context.move_to(layer.x, layer.y);
                context.line_to(*x2, *y2);

                if let Some(stroke_color) = &layer.stroke_color {
                    context.set_stroke_style(&stroke_color.into());
                    if let Some(stroke_width) = layer.stroke_width {
                        context.set_line_width(stroke_width);
                    }
                    context.stroke();
                }
            }
        }

        Ok(())
    }

    /// Internal helper for text layout
    fn layout_text(
        &self,
        text_layer: &TextLayer,
        canvas_width: u32,
        canvas_height: u32,
    ) -> Vec<TextLine> {
        let mut lines = Vec::new();
        let words: Vec<&str> = text_layer.content.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_y = text_layer.y;

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            // Estimate text width (simplified approach)
            let estimated_width =
                test_line.len() as f64 * 0.6 * (text_layer.font_size as f64 * 0.6);
            if estimated_width > text_layer.max_width && !current_line.is_empty() {
                lines.push(TextLine {
                    text: current_line,
                    x: text_layer.x,
                    y: current_y,
                });
                current_line = word.to_string();
                current_y += text_layer.line_height;
            } else {
                current_line = test_line;
            }
        }

        if !current_line.is_empty() {
            lines.push(TextLine {
                text: current_line,
                x: text_layer.x,
                y: current_y,
            });
        }

        lines
    }
}

/// Text line for layout calculations
#[derive(Debug, Clone)]
struct TextLine {
    text: String,
    x: f64,
    y: f64,
}

/// Utility functions for canvas OG generation
pub struct CanvasOgUtils;

impl CanvasOgUtils {
    /// Create a default OG image generator
    pub fn create_generator() -> Result<CanvasOgGenerator, JsValue> {
        CanvasOgGenerator::new()
    }

    /// Generate a quick OG image
    pub fn quick_generate(title: &str) -> Result<String, JsValue> {
        let mut generator = CanvasOgGenerator::new()?;
        let result = generator.generate_simple(title)?;
        Ok(result.data_url)
    }

    /// Generate OG image with custom parameters
    pub fn generate_custom(params: CanvasOgParams) -> Result<CanvasOgResult, JsValue> {
        let mut generator = CanvasOgGenerator::new()?;
        generator.generate(params)
    }

    /// Check if canvas is supported
    pub fn is_supported() -> bool {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(canvas) = document.create_element("canvas") {
                    if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                        return canvas.get_context("2d").is_ok();
                    }
                }
            }
        }
        false
    }

    /// Get recommended dimensions for OG images
    pub fn get_recommended_dimensions() -> (u32, u32) {
        (1200, 630) // Standard OG image dimensions
    }

    /// Validate OG image parameters
    pub fn validate_params(params: &CanvasOgParams) -> Result<(), String> {
        if params.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        if let Some(width) = params.width {
            if width < 200 || width > 2000 {
                return Err("Width must be between 200 and 2000 pixels".to_string());
            }
        }

        if let Some(height) = params.height {
            if height < 200 || height > 2000 {
                return Err("Height must be between 200 and 2000 pixels".to_string());
            }
        }

        Ok(())
    }
}

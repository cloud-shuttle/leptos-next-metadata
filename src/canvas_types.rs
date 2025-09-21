//! Advanced Canvas Types
//!
//! Shared types for advanced canvas features that can be used in both WASM and native environments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Layer types for advanced canvas composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OgImageLayer {
    Text(TextLayer),
    Image(ImageLayer),
    Shape(ShapeLayer),
}

/// Text layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLayer {
    pub content: String,
    pub font_family: String,
    pub font_size: u32,
    pub color: String,
    pub x: f64,
    pub y: f64,
    pub max_width: f64,
    pub line_height: f64,
    pub text_align: TextAlign,
    pub z_index: u32,
    pub gradient: Option<TextGradient>,
    pub shadow: Option<TextShadow>,
    pub outline: Option<TextOutline>,
}

/// Image layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLayer {
    pub src: String, // URL or Base64
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub opacity: f64,
    pub blend_mode: Option<String>, // e.g., "multiply", "screen"
    pub z_index: u32,
}

/// Shape layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeLayer {
    pub shape_type: ShapeType,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill_color: Option<String>,
    pub stroke_color: Option<String>,
    pub stroke_width: Option<f64>,
    pub z_index: u32,
}

/// Shape types for drawing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Line { x2: f64, y2: f64 },
}

/// Text gradient configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextGradient {
    pub gradient_type: GradientType,
    pub colors: Vec<String>,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

/// Gradient types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradientType {
    Linear,
    Radial,
}

/// Text shadow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextShadow {
    pub color: String,
    pub blur: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

/// Text outline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOutline {
    pub color: String,
    pub width: f64,
}

/// Text alignment options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// OG image template for reusable layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgImageTemplate {
    pub name: String,
    pub description: String,
    pub default_params: CanvasOgParams,
    pub layers: Vec<OgImageLayer>,
    pub version: String,
}

/// Canvas OG parameters (simplified for cross-platform use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasOgParams {
    /// Image title
    pub title: String,
    /// Image description
    pub description: Option<String>,
    /// Image width
    pub width: Option<u32>,
    /// Image height
    pub height: Option<u32>,
    /// Background color (hex or CSS color)
    pub background_color: Option<String>,
    /// Text color (hex or CSS color)
    pub text_color: Option<String>,
    /// Font family
    pub font_family: Option<String>,
    /// Font size for title
    pub title_font_size: Option<u32>,
    /// Font size for description
    pub description_font_size: Option<u32>,
    /// Logo URL (optional)
    pub logo_url: Option<String>,
    /// Custom font URLs (font_name -> url)
    pub font_urls: Option<HashMap<String, String>>,
    /// Default font family
    pub default_font_family: Option<String>,
    /// Define elements as layers
    pub layers: Option<Vec<OgImageLayer>>,
    /// Background image URL
    pub background_image_url: Option<String>,
    /// Background image opacity
    pub background_image_opacity: Option<f64>,
    /// Text gradient definition
    pub text_gradient: Option<TextGradient>,
    /// Text shadow definition
    pub text_shadow: Option<TextShadow>,
    /// Text outline definition
    pub text_outline: Option<TextOutline>,
    /// Logo position
    pub logo_position: Option<LogoPosition>,
    /// Text alignment
    pub text_align: Option<TextAlign>,
    /// Padding
    pub padding: Option<u32>,
}

/// Logo position options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogoPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

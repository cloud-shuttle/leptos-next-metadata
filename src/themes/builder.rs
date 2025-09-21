//! Theme Builder
//!
//! Provides a fluent API for building custom themes with validation and
//! intelligent defaults.

use super::{
    BorderEffects, BorderStyle, ContentAlignment, FontWeights, GradientConfig, GradientType,
    LogoPosition, PatternConfig, PatternType, TextAlignment, TextGradient, TextOutline, TextShadow,
    Theme, ThemeCategory, ThemeMetadata,
};
use crate::error::{ErrorKind, MetadataError};

/// Fluent builder for creating custom themes
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(id: String, name: String, description: String, category: ThemeCategory) -> Self {
        Self {
            theme: Theme::new(id, name, description, category),
        }
    }

    /// Set the theme version
    pub fn version(mut self, version: String) -> Self {
        self.theme.version = version;
        self
    }

    /// Set the primary color
    pub fn primary_color(mut self, color: String) -> Self {
        self.theme.colors.primary = color;
        self
    }

    /// Set the secondary color
    pub fn secondary_color(mut self, color: String) -> Self {
        self.theme.colors.secondary = color;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: String) -> Self {
        self.theme.colors.background = color;
        self
    }

    /// Set the text color
    pub fn text_color(mut self, color: String) -> Self {
        self.theme.colors.text = color;
        self
    }

    /// Set the accent color
    pub fn accent_color(mut self, color: String) -> Self {
        self.theme.colors.accent = color;
        self
    }

    /// Add additional colors
    pub fn additional_colors(mut self, colors: Vec<String>) -> Self {
        self.theme.colors.additional = colors;
        self
    }

    /// Set the primary font
    pub fn primary_font(mut self, font: String) -> Self {
        self.theme.typography.primary_font = font;
        self
    }

    /// Set the secondary font
    pub fn secondary_font(mut self, font: String) -> Self {
        self.theme.typography.secondary_font = font;
        self
    }

    /// Set the title font size
    pub fn title_size(mut self, size: u32) -> Self {
        self.theme.typography.title_size = size;
        self
    }

    /// Set the description font size
    pub fn description_size(mut self, size: u32) -> Self {
        self.theme.typography.description_size = size;
        self
    }

    /// Set font weights
    pub fn font_weights(mut self, weights: FontWeights) -> Self {
        self.theme.typography.weights = weights;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: u32) -> Self {
        self.theme.layout.padding = padding;
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.theme.layout.spacing = spacing;
        self
    }

    /// Set content alignment
    pub fn content_alignment(mut self, alignment: ContentAlignment) -> Self {
        self.theme.layout.alignment = alignment;
        self
    }

    /// Set logo position
    pub fn logo_position(mut self, position: LogoPosition) -> Self {
        self.theme.layout.logo_position = position;
        self
    }

    /// Set text alignment
    pub fn text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.theme.layout.text_alignment = alignment;
        self
    }

    /// Add text shadow
    pub fn text_shadow(mut self, shadow: TextShadow) -> Self {
        self.theme.effects.text.shadow = Some(shadow);
        self
    }

    /// Add text outline
    pub fn text_outline(mut self, outline: TextOutline) -> Self {
        self.theme.effects.text.outline = Some(outline);
        self
    }

    /// Add text gradient
    pub fn text_gradient(mut self, gradient: TextGradient) -> Self {
        self.theme.effects.text.gradient = Some(gradient);
        self
    }

    /// Add text glow effect
    pub fn text_glow(mut self, glow: super::GlowEffect) -> Self {
        self.theme.effects.text.glow = Some(glow);
        self
    }

    /// Set background type
    pub fn background_type(mut self, background_type: super::BackgroundType) -> Self {
        self.theme.effects.background.background_type = background_type;
        self
    }

    /// Add background gradient
    pub fn background_gradient(mut self, gradient: GradientConfig) -> Self {
        self.theme.effects.background.gradient = Some(gradient);
        self
    }

    /// Add background pattern
    pub fn background_pattern(mut self, pattern: PatternConfig) -> Self {
        self.theme.effects.background.pattern = Some(pattern);
        self
    }

    /// Set background blur
    pub fn background_blur(mut self, blur: f64) -> Self {
        self.theme.effects.background.blur = Some(blur);
        self
    }

    /// Set border effects
    pub fn border(mut self, border: BorderEffects) -> Self {
        self.theme.effects.border = border;
        self
    }

    /// Set theme metadata
    pub fn metadata(mut self, metadata: ThemeMetadata) -> Self {
        self.theme.metadata = metadata;
        self
    }

    /// Set theme author
    pub fn author(mut self, author: String) -> Self {
        self.theme.metadata.author = author;
        self
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.theme.metadata.tags = tags;
        self
    }

    /// Set license
    pub fn license(mut self, license: String) -> Self {
        self.theme.metadata.license = license;
        self
    }

    /// Set preview URL
    pub fn preview_url(mut self, url: String) -> Self {
        self.theme.metadata.preview_url = Some(url);
        self
    }

    /// Build the theme with validation
    pub fn build(self) -> Result<Theme, MetadataError> {
        self.theme.validate()?;
        Ok(self.theme)
    }

    /// Build the theme without validation (for internal use)
    pub fn build_unchecked(self) -> Theme {
        self.theme
    }
}

/// Helper functions for creating common theme elements
pub struct ThemeHelpers;

impl ThemeHelpers {
    /// Create a simple text shadow
    pub fn simple_text_shadow(
        color: String,
        blur: f64,
        offset_x: f64,
        offset_y: f64,
    ) -> TextShadow {
        TextShadow {
            color,
            blur,
            offset_x,
            offset_y,
        }
    }

    /// Create a simple text outline
    pub fn simple_text_outline(color: String, width: f64) -> TextOutline {
        TextOutline { color, width }
    }

    /// Create a linear text gradient
    pub fn linear_text_gradient(
        colors: Vec<String>,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> TextGradient {
        TextGradient {
            gradient_type: GradientType::Linear,
            colors,
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    /// Create a radial text gradient
    pub fn radial_text_gradient(
        colors: Vec<String>,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> TextGradient {
        TextGradient {
            gradient_type: GradientType::Radial,
            colors,
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    /// Create a linear background gradient
    pub fn linear_background_gradient(
        colors: Vec<String>,
        angle: f64,
        stops: Vec<f64>,
    ) -> GradientConfig {
        GradientConfig {
            gradient_type: GradientType::Linear,
            colors,
            angle,
            stops,
        }
    }

    /// Create a radial background gradient
    pub fn radial_background_gradient(
        colors: Vec<String>,
        angle: f64,
        stops: Vec<f64>,
    ) -> GradientConfig {
        GradientConfig {
            gradient_type: GradientType::Radial,
            colors,
            angle,
            stops,
        }
    }

    /// Create a dot pattern
    pub fn dot_pattern(color: String, opacity: f64, size: u32) -> PatternConfig {
        PatternConfig {
            pattern_type: PatternType::Dots,
            color,
            opacity,
            size,
        }
    }

    /// Create a line pattern
    pub fn line_pattern(color: String, opacity: f64, size: u32) -> PatternConfig {
        PatternConfig {
            pattern_type: PatternType::Lines,
            color,
            opacity,
            size,
        }
    }

    /// Create a grid pattern
    pub fn grid_pattern(color: String, opacity: f64, size: u32) -> PatternConfig {
        PatternConfig {
            pattern_type: PatternType::Grid,
            color,
            opacity,
            size,
        }
    }

    /// Create a simple border
    pub fn simple_border(width: f64, color: String, radius: f64) -> BorderEffects {
        BorderEffects {
            width,
            color,
            radius,
            style: BorderStyle::Solid,
        }
    }

    /// Create a dashed border
    pub fn dashed_border(width: f64, color: String, radius: f64) -> BorderEffects {
        BorderEffects {
            width,
            color,
            radius,
            style: BorderStyle::Dashed,
        }
    }

    /// Create a dotted border
    pub fn dotted_border(width: f64, color: String, radius: f64) -> BorderEffects {
        BorderEffects {
            width,
            color,
            radius,
            style: BorderStyle::Dotted,
        }
    }

    /// Create a double border
    pub fn double_border(width: f64, color: String, radius: f64) -> BorderEffects {
        BorderEffects {
            width,
            color,
            radius,
            style: BorderStyle::Double,
        }
    }

    /// Create a text glow effect
    pub fn text_glow(color: String, intensity: f64, radius: f64) -> super::GlowEffect {
        super::GlowEffect {
            color,
            intensity,
            radius,
        }
    }

    /// Validate hex color format
    pub fn is_valid_hex_color(color: &str) -> bool {
        if color.starts_with('#') && color.len() == 7 {
            color[1..].chars().all(|c| c.is_ascii_hexdigit())
        } else {
            false
        }
    }

    /// Convert RGB to hex
    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Convert hex to RGB
    pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), MetadataError> {
        if !Self::is_valid_hex_color(hex) {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Invalid hex color format".to_string(),
            ));
        }

        let hex = &hex[1..]; // Remove #
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
            MetadataError::new(ErrorKind::Validation, "Invalid hex color".to_string())
        })?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
            MetadataError::new(ErrorKind::Validation, "Invalid hex color".to_string())
        })?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
            MetadataError::new(ErrorKind::Validation, "Invalid hex color".to_string())
        })?;

        Ok((r, g, b))
    }

    /// Get complementary color
    pub fn get_complementary_color(hex: &str) -> Result<String, MetadataError> {
        let (r, g, b) = Self::hex_to_rgb(hex)?;
        Ok(Self::rgb_to_hex(255 - r, 255 - g, 255 - b))
    }

    /// Get analogous colors
    pub fn get_analogous_colors(hex: &str) -> Result<Vec<String>, MetadataError> {
        let (r, g, b) = Self::hex_to_rgb(hex)?;

        // Convert to HSL for better color manipulation
        let (h, s, l) = Self::rgb_to_hsl(r, g, b);

        // Generate analogous colors (±30 degrees)
        let analogous1 = Self::hsl_to_rgb((h + 30.0) % 360.0, s, l);
        let analogous2 = Self::hsl_to_rgb((h - 30.0 + 360.0) % 360.0, s, l);

        Ok(vec![
            Self::rgb_to_hex(analogous1.0, analogous1.1, analogous1.2),
            Self::rgb_to_hex(analogous2.0, analogous2.1, analogous2.2),
        ])
    }

    /// Convert RGB to HSL
    fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let diff = max - min;

        let l = (max + min) / 2.0;

        let s = if diff == 0.0 {
            0.0
        } else {
            diff / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if diff == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / diff) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / diff + 2.0)
        } else {
            60.0 * ((r - g) / diff + 4.0)
        };

        (h, s, l)
    }

    /// Convert HSL to RGB
    fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
        )
    }
}

/// Quick theme creation functions
pub struct QuickThemes;

impl QuickThemes {
    /// Create a dark theme
    pub fn dark_theme(id: String, name: String) -> Result<Theme, MetadataError> {
        ThemeBuilder::new(
            id,
            name,
            "Dark theme with high contrast".to_string(),
            ThemeCategory::Modern,
        )
        .background_color("#1a1a1a".to_string())
        .text_color("#ffffff".to_string())
        .primary_color("#3b82f6".to_string())
        .secondary_color("#6b7280".to_string())
        .accent_color("#f59e0b".to_string())
        .build()
    }

    /// Create a light theme
    pub fn light_theme(id: String, name: String) -> Result<Theme, MetadataError> {
        ThemeBuilder::new(
            id,
            name,
            "Light theme with clean design".to_string(),
            ThemeCategory::Minimalist,
        )
        .background_color("#ffffff".to_string())
        .text_color("#1f2937".to_string())
        .primary_color("#3b82f6".to_string())
        .secondary_color("#6b7280".to_string())
        .accent_color("#10b981".to_string())
        .build()
    }

    /// Create a monochrome theme
    pub fn monochrome_theme(id: String, name: String) -> Result<Theme, MetadataError> {
        ThemeBuilder::new(
            id,
            name,
            "Monochrome theme with grayscale colors".to_string(),
            ThemeCategory::Minimalist,
        )
        .background_color("#ffffff".to_string())
        .text_color("#000000".to_string())
        .primary_color("#000000".to_string())
        .secondary_color("#666666".to_string())
        .accent_color("#000000".to_string())
        .build()
    }

    /// Create a colorful theme
    pub fn colorful_theme(id: String, name: String) -> Result<Theme, MetadataError> {
        ThemeBuilder::new(
            id,
            name,
            "Colorful theme with vibrant colors".to_string(),
            ThemeCategory::Creative,
        )
        .background_color("#fef3c7".to_string())
        .text_color("#1f2937".to_string())
        .primary_color("#ec4899".to_string())
        .secondary_color("#8b5cf6".to_string())
        .accent_color("#f59e0b".to_string())
        .additional_colors(vec![
            "#10b981".to_string(),
            "#ef4444".to_string(),
            "#06b6d4".to_string(),
        ])
        .build()
    }
}

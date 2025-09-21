//! Theme Support for OG Images
//!
//! Provides a comprehensive theming system for customizable OG image generation
//! with predefined themes, custom theme creation, and dynamic theme switching.

pub mod builder;
pub mod predefined;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::canvas_types::{
    CanvasOgParams, GradientType, OgImageLayer, TextGradient, TextOutline, TextShadow,
};
use crate::error::{ErrorKind, MetadataError};

/// Theme identifier
pub type ThemeId = String;

/// Theme version for compatibility tracking
pub type ThemeVersion = String;

/// Theme category for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeCategory {
    Business,
    Technology,
    Creative,
    Minimalist,
    Bold,
    Elegant,
    Modern,
    Classic,
    Custom,
}

impl fmt::Display for ThemeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeCategory::Business => write!(f, "Business"),
            ThemeCategory::Technology => write!(f, "Technology"),
            ThemeCategory::Creative => write!(f, "Creative"),
            ThemeCategory::Minimalist => write!(f, "Minimalist"),
            ThemeCategory::Bold => write!(f, "Bold"),
            ThemeCategory::Elegant => write!(f, "Elegant"),
            ThemeCategory::Modern => write!(f, "Modern"),
            ThemeCategory::Classic => write!(f, "Classic"),
            ThemeCategory::Custom => write!(f, "Custom"),
        }
    }
}

/// Color palette for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary color (hex)
    pub primary: String,
    /// Secondary color (hex)
    pub secondary: String,
    /// Background color (hex)
    pub background: String,
    /// Text color (hex)
    pub text: String,
    /// Accent color (hex)
    pub accent: String,
    /// Additional colors for gradients and effects
    pub additional: Vec<String>,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: "#4f46e5".to_string(),
            secondary: "#7c3aed".to_string(),
            background: "#ffffff".to_string(),
            text: "#1f2937".to_string(),
            accent: "#f59e0b".to_string(),
            additional: vec!["#10b981".to_string(), "#ef4444".to_string()],
        }
    }
}

/// Typography settings for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Primary font family
    pub primary_font: String,
    /// Secondary font family
    pub secondary_font: String,
    /// Title font size
    pub title_size: u32,
    /// Description font size
    pub description_size: u32,
    /// Font weights
    pub weights: FontWeights,
}

/// Font weight settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeights {
    pub light: u32,
    pub normal: u32,
    pub medium: u32,
    pub bold: u32,
}

impl Default for FontWeights {
    fn default() -> Self {
        Self {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            primary_font: "Inter, sans-serif".to_string(),
            secondary_font: "Georgia, serif".to_string(),
            title_size: 48,
            description_size: 24,
            weights: FontWeights::default(),
        }
    }
}

/// Layout configuration for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    /// Padding around content
    pub padding: u32,
    /// Spacing between elements
    pub spacing: u32,
    /// Content alignment
    pub alignment: ContentAlignment,
    /// Logo position
    pub logo_position: LogoPosition,
    /// Text alignment
    pub text_alignment: TextAlignment,
}

/// Content alignment options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Logo position options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogoPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    None,
}

/// Text alignment options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            padding: 60,
            spacing: 30,
            alignment: ContentAlignment::Center,
            logo_position: LogoPosition::None,
            text_alignment: TextAlignment::Center,
        }
    }
}

/// Visual effects for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffects {
    /// Background effects
    pub background: BackgroundEffects,
    /// Text effects
    pub text: TextEffects,
    /// Border effects
    pub border: BorderEffects,
}

/// Background effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundEffects {
    /// Background type
    pub background_type: BackgroundType,
    /// Gradient configuration
    pub gradient: Option<GradientConfig>,
    /// Pattern configuration
    pub pattern: Option<PatternConfig>,
    /// Blur effect
    pub blur: Option<f64>,
}

/// Background type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundType {
    Solid,
    Gradient,
    Pattern,
    Image,
}

/// Gradient configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientConfig {
    pub gradient_type: GradientType,
    pub colors: Vec<String>,
    pub angle: f64,
    pub stops: Vec<f64>,
}

/// Pattern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    pub pattern_type: PatternType,
    pub color: String,
    pub opacity: f64,
    pub size: u32,
}

/// Pattern types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Dots,
    Lines,
    Grid,
    Waves,
    Geometric,
}

/// Text effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEffects {
    /// Text shadow
    pub shadow: Option<TextShadow>,
    /// Text outline
    pub outline: Option<TextOutline>,
    /// Text gradient
    pub gradient: Option<TextGradient>,
    /// Text glow
    pub glow: Option<GlowEffect>,
}

/// Glow effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlowEffect {
    pub color: String,
    pub intensity: f64,
    pub radius: f64,
}

/// Border effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderEffects {
    /// Border width
    pub width: f64,
    /// Border color
    pub color: String,
    /// Border radius
    pub radius: f64,
    /// Border style
    pub style: BorderStyle,
}

/// Border styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
}

impl Default for VisualEffects {
    fn default() -> Self {
        Self {
            background: BackgroundEffects {
                background_type: BackgroundType::Solid,
                gradient: None,
                pattern: None,
                blur: None,
            },
            text: TextEffects {
                shadow: None,
                outline: None,
                gradient: None,
                glow: None,
            },
            border: BorderEffects {
                width: 0.0,
                color: "#000000".to_string(),
                radius: 0.0,
                style: BorderStyle::Solid,
            },
        }
    }
}

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Unique theme identifier
    pub id: ThemeId,
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Theme version
    pub version: ThemeVersion,
    /// Theme category
    pub category: ThemeCategory,
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    pub typography: Typography,
    /// Layout configuration
    pub layout: Layout,
    /// Visual effects
    pub effects: VisualEffects,
    /// Custom layers for advanced themes
    pub custom_layers: Vec<OgImageLayer>,
    /// Theme metadata
    pub metadata: ThemeMetadata,
}

/// Theme metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// Theme author
    pub author: String,
    /// Creation date
    pub created_at: String,
    /// Last modified date
    pub modified_at: String,
    /// Tags for search and categorization
    pub tags: Vec<String>,
    /// License information
    pub license: String,
    /// Theme preview URL
    pub preview_url: Option<String>,
}

impl Default for ThemeMetadata {
    fn default() -> Self {
        Self {
            author: "leptos-next-metadata".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            modified_at: chrono::Utc::now().to_rfc3339(),
            tags: vec![],
            license: "MIT".to_string(),
            preview_url: None,
        }
    }
}

impl Theme {
    /// Create a new theme
    pub fn new(id: ThemeId, name: String, description: String, category: ThemeCategory) -> Self {
        Self {
            id,
            name,
            description,
            version: "1.0.0".to_string(),
            category,
            colors: ColorPalette::default(),
            typography: Typography::default(),
            layout: Layout::default(),
            effects: VisualEffects::default(),
            custom_layers: vec![],
            metadata: ThemeMetadata::default(),
        }
    }

    /// Apply theme to canvas parameters
    pub fn apply_to_canvas_params(&self, mut params: CanvasOgParams) -> CanvasOgParams {
        // Apply colors
        params.background_color = Some(self.colors.background.clone());
        params.text_color = Some(self.colors.text.clone());

        // Apply typography
        params.font_family = Some(self.typography.primary_font.clone());
        params.title_font_size = Some(self.typography.title_size);
        params.description_font_size = Some(self.typography.description_size);

        // Apply layout
        params.padding = Some(self.layout.padding);
        params.text_align = Some(match self.layout.text_alignment {
            TextAlignment::Left => crate::canvas_types::TextAlign::Left,
            TextAlignment::Center => crate::canvas_types::TextAlign::Center,
            TextAlignment::Right => crate::canvas_types::TextAlign::Right,
        });

        // Apply visual effects
        if let Some(gradient) = &self.effects.text.gradient {
            params.text_gradient = Some(gradient.clone());
        }
        if let Some(shadow) = &self.effects.text.shadow {
            params.text_shadow = Some(shadow.clone());
        }
        if let Some(outline) = &self.effects.text.outline {
            params.text_outline = Some(outline.clone());
        }

        // Add custom layers
        if !self.custom_layers.is_empty() {
            params.layers = Some(self.custom_layers.clone());
        }

        params
    }

    /// Generate theme preview
    pub fn generate_preview(&self) -> Result<String, MetadataError> {
        let _preview_params = CanvasOgParams {
            title: format!("{} Theme Preview", self.name),
            description: Some(self.description.clone()),
            width: Some(1200),
            height: Some(630),
            background_color: Some(self.colors.background.clone()),
            text_color: Some(self.colors.text.clone()),
            font_family: Some(self.typography.primary_font.clone()),
            title_font_size: Some(self.typography.title_size),
            description_font_size: Some(self.typography.description_size),
            logo_url: None,
            font_urls: None,
            default_font_family: None,
            layers: Some(self.custom_layers.clone()),
            background_image_url: None,
            background_image_opacity: None,
            text_gradient: self.effects.text.gradient.clone(),
            text_shadow: self.effects.text.shadow.clone(),
            text_outline: self.effects.text.outline.clone(),
            logo_position: Some(match self.layout.logo_position {
                LogoPosition::TopLeft => crate::canvas_types::LogoPosition::TopLeft,
                LogoPosition::TopRight => crate::canvas_types::LogoPosition::TopRight,
                LogoPosition::BottomLeft => crate::canvas_types::LogoPosition::BottomLeft,
                LogoPosition::BottomRight => crate::canvas_types::LogoPosition::BottomRight,
                LogoPosition::Center => crate::canvas_types::LogoPosition::Center,
                LogoPosition::None => crate::canvas_types::LogoPosition::TopLeft,
            }),
            text_align: Some(match self.layout.text_alignment {
                TextAlignment::Left => crate::canvas_types::TextAlign::Left,
                TextAlignment::Center => crate::canvas_types::TextAlign::Center,
                TextAlignment::Right => crate::canvas_types::TextAlign::Right,
            }),
            padding: Some(self.layout.padding),
        };

        // In a real implementation, this would generate an actual image
        // For now, we'll return a placeholder
        let svg_content = format!(
            r#"<svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
                <rect width="100%" height="100%" fill="{}"/>
                <text x="50%" y="50%" text-anchor="middle" dy=".3em" font-family="{}" font-size="{}" fill="{}">
                    {} Theme Preview
                </text>
            </svg>"#,
            self.colors.background,
            self.typography.primary_font,
            self.typography.title_size,
            self.colors.text,
            self.name
        );

        Ok(format!(
            "data:image/svg+xml;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, svg_content)
        ))
    }

    /// Validate theme configuration
    pub fn validate(&self) -> Result<(), MetadataError> {
        if self.id.is_empty() {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Theme ID cannot be empty".to_string(),
            ));
        }

        if self.name.is_empty() {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Theme name cannot be empty".to_string(),
            ));
        }

        // Validate colors
        if !self.is_valid_hex_color(&self.colors.primary) {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Invalid primary color format".to_string(),
            ));
        }

        if !self.is_valid_hex_color(&self.colors.background) {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Invalid background color format".to_string(),
            ));
        }

        // Validate typography
        if self.typography.title_size < 12 || self.typography.title_size > 120 {
            return Err(MetadataError::new(
                ErrorKind::Validation,
                "Title font size must be between 12 and 120".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if a string is a valid hex color
    fn is_valid_hex_color(&self, color: &str) -> bool {
        if color.starts_with('#') && color.len() == 7 {
            color[1..].chars().all(|c| c.is_ascii_hexdigit())
        } else {
            false
        }
    }

    /// Clone theme with new ID
    pub fn clone_with_id(&self, new_id: ThemeId, new_name: String) -> Self {
        let mut cloned = self.clone();
        cloned.id = new_id;
        cloned.name = new_name;
        cloned.metadata.modified_at = chrono::Utc::now().to_rfc3339();
        cloned
    }

    /// Update theme version
    pub fn update_version(&mut self, new_version: ThemeVersion) {
        self.version = new_version;
        self.metadata.modified_at = chrono::Utc::now().to_rfc3339();
    }
}

/// Theme manager for handling multiple themes
#[derive(Debug, Clone)]
pub struct ThemeManager {
    /// Available themes
    themes: HashMap<ThemeId, Theme>,
    /// Default theme ID
    default_theme_id: Option<ThemeId>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            default_theme_id: None,
        }
    }

    /// Add a theme to the manager
    pub fn add_theme(&mut self, theme: Theme) -> Result<(), MetadataError> {
        theme.validate()?;
        let theme_id = theme.id.clone();
        self.themes.insert(theme_id, theme);
        Ok(())
    }

    /// Get a theme by ID
    pub fn get_theme(&self, theme_id: &ThemeId) -> Option<&Theme> {
        self.themes.get(theme_id)
    }

    /// Get all themes
    pub fn get_all_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    /// Get themes by category
    pub fn get_themes_by_category(&self, category: &ThemeCategory) -> Vec<&Theme> {
        self.themes
            .values()
            .filter(|theme| &theme.category == category)
            .collect()
    }

    /// Set default theme
    pub fn set_default_theme(&mut self, theme_id: ThemeId) -> Result<(), MetadataError> {
        if self.themes.contains_key(&theme_id) {
            self.default_theme_id = Some(theme_id);
            Ok(())
        } else {
            Err(MetadataError::new(
                ErrorKind::Unknown,
                format!("Theme with ID '{}' not found", theme_id),
            ))
        }
    }

    /// Get default theme
    pub fn get_default_theme(&self) -> Option<&Theme> {
        if let Some(ref default_id) = self.default_theme_id {
            self.get_theme(default_id)
        } else {
            None
        }
    }

    /// Remove a theme
    pub fn remove_theme(&mut self, theme_id: &ThemeId) -> Option<Theme> {
        if let Some(ref default_id) = self.default_theme_id {
            if default_id == theme_id {
                self.default_theme_id = None;
            }
        }
        self.themes.remove(theme_id)
    }

    /// Search themes by name or tags
    pub fn search_themes(&self, query: &str) -> Vec<&Theme> {
        let query_lower = query.to_lowercase();
        self.themes
            .values()
            .filter(|theme| {
                theme.name.to_lowercase().contains(&query_lower)
                    || theme.description.to_lowercase().contains(&query_lower)
                    || theme
                        .metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get theme count
    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }

    /// Clear all themes
    pub fn clear(&mut self) {
        self.themes.clear();
        self.default_theme_id = None;
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export builder and predefined modules
pub use builder::{QuickThemes, ThemeBuilder, ThemeHelpers};
pub use predefined::create_predefined_themes;

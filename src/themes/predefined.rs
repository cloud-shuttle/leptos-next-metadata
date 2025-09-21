//! Predefined Themes
//!
//! A collection of professionally designed themes for OG images
//! covering various use cases and design styles.

use super::{
    BorderEffects, BorderStyle, ColorPalette, ContentAlignment, FontWeights, GradientConfig,
    GradientType, Layout, LogoPosition, PatternConfig, PatternType, TextAlignment, TextEffects,
    TextGradient, TextOutline, TextShadow, Theme, ThemeCategory, ThemeManager, Typography,
    VisualEffects,
};

/// Create all predefined themes
pub fn create_predefined_themes() -> ThemeManager {
    let mut manager = ThemeManager::new();

    // Add all predefined themes
    manager.add_theme(create_business_theme()).unwrap();
    manager.add_theme(create_tech_theme()).unwrap();
    manager.add_theme(create_creative_theme()).unwrap();
    manager.add_theme(create_minimalist_theme()).unwrap();
    manager.add_theme(create_bold_theme()).unwrap();
    manager.add_theme(create_elegant_theme()).unwrap();
    manager.add_theme(create_modern_theme()).unwrap();
    manager.add_theme(create_classic_theme()).unwrap();

    // Set default theme
    manager.set_default_theme("business".to_string()).unwrap();

    manager
}

/// Business theme - Professional and clean
pub fn create_business_theme() -> Theme {
    let mut theme = Theme::new(
        "business".to_string(),
        "Business Professional".to_string(),
        "Clean, professional theme perfect for corporate content and business communications"
            .to_string(),
        ThemeCategory::Business,
    );

    // Business color palette
    theme.colors = ColorPalette {
        primary: "#1e40af".to_string(),    // Blue
        secondary: "#374151".to_string(),  // Gray
        background: "#ffffff".to_string(), // White
        text: "#111827".to_string(),       // Dark gray
        accent: "#059669".to_string(),     // Green
        additional: vec![
            "#6b7280".to_string(), // Light gray
            "#f3f4f6".to_string(), // Very light gray
        ],
    };

    // Business typography
    theme.typography = Typography {
        primary_font: "Inter, sans-serif".to_string(),
        secondary_font: "Georgia, serif".to_string(),
        title_size: 52,
        description_size: 24,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        },
    };

    // Business layout
    theme.layout = Layout {
        padding: 80,
        spacing: 40,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::TopLeft,
        text_alignment: TextAlignment::Center,
    };

    // Business effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Solid,
            gradient: None,
            pattern: None,
            blur: None,
        },
        text: TextEffects {
            shadow: Some(TextShadow {
                color: "#000000".to_string(),
                blur: 2.0,
                offset_x: 0.0,
                offset_y: 1.0,
            }),
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
    };

    theme
}

/// Technology theme - Modern and tech-focused
pub fn create_tech_theme() -> Theme {
    let mut theme = Theme::new(
        "tech".to_string(),
        "Technology Modern".to_string(),
        "Sleek, modern theme with tech-inspired colors and gradients".to_string(),
        ThemeCategory::Technology,
    );

    // Tech color palette
    theme.colors = ColorPalette {
        primary: "#6366f1".to_string(),    // Indigo
        secondary: "#8b5cf6".to_string(),  // Purple
        background: "#0f172a".to_string(), // Dark blue
        text: "#f8fafc".to_string(),       // Light gray
        accent: "#06b6d4".to_string(),     // Cyan
        additional: vec![
            "#1e293b".to_string(), // Darker blue
            "#64748b".to_string(), // Slate
        ],
    };

    // Tech typography
    theme.typography = Typography {
        primary_font: "JetBrains Mono, monospace".to_string(),
        secondary_font: "Inter, sans-serif".to_string(),
        title_size: 48,
        description_size: 22,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        },
    };

    // Tech layout
    theme.layout = Layout {
        padding: 60,
        spacing: 30,
        alignment: ContentAlignment::Left,
        logo_position: LogoPosition::TopRight,
        text_alignment: TextAlignment::Left,
    };

    // Tech effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Gradient,
            gradient: Some(GradientConfig {
                gradient_type: GradientType::Linear,
                colors: vec!["#0f172a".to_string(), "#1e293b".to_string()],
                angle: 135.0,
                stops: vec![0.0, 1.0],
            }),
            pattern: None,
            blur: None,
        },
        text: TextEffects {
            shadow: None,
            outline: None,
            gradient: Some(TextGradient {
                gradient_type: GradientType::Linear,
                colors: vec!["#f8fafc".to_string(), "#06b6d4".to_string()],
                start_x: 0.0,
                start_y: 0.0,
                end_x: 100.0,
                end_y: 0.0,
            }),
            glow: Some(super::GlowEffect {
                color: "#06b6d4".to_string(),
                intensity: 0.5,
                radius: 10.0,
            }),
        },
        border: BorderEffects {
            width: 2.0,
            color: "#6366f1".to_string(),
            radius: 8.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

/// Creative theme - Vibrant and artistic
pub fn create_creative_theme() -> Theme {
    let mut theme = Theme::new(
        "creative".to_string(),
        "Creative Vibrant".to_string(),
        "Bold, colorful theme perfect for creative content and artistic projects".to_string(),
        ThemeCategory::Creative,
    );

    // Creative color palette
    theme.colors = ColorPalette {
        primary: "#ec4899".to_string(),    // Pink
        secondary: "#8b5cf6".to_string(),  // Purple
        background: "#fef3c7".to_string(), // Light yellow
        text: "#1f2937".to_string(),       // Dark gray
        accent: "#f59e0b".to_string(),     // Orange
        additional: vec![
            "#10b981".to_string(), // Green
            "#ef4444".to_string(), // Red
        ],
    };

    // Creative typography
    theme.typography = Typography {
        primary_font: "Poppins, sans-serif".to_string(),
        secondary_font: "Dancing Script, cursive".to_string(),
        title_size: 56,
        description_size: 26,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 600,
            bold: 800,
        },
    };

    // Creative layout
    theme.layout = Layout {
        padding: 50,
        spacing: 35,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::Center,
        text_alignment: TextAlignment::Center,
    };

    // Creative effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Pattern,
            gradient: None,
            pattern: Some(PatternConfig {
                pattern_type: PatternType::Dots,
                color: "#ec4899".to_string(),
                opacity: 0.1,
                size: 20,
            }),
            blur: None,
        },
        text: TextEffects {
            shadow: Some(TextShadow {
                color: "#ec4899".to_string(),
                blur: 4.0,
                offset_x: 2.0,
                offset_y: 2.0,
            }),
            outline: Some(TextOutline {
                color: "#ffffff".to_string(),
                width: 2.0,
            }),
            gradient: Some(TextGradient {
                gradient_type: GradientType::Linear,
                colors: vec!["#ec4899".to_string(), "#8b5cf6".to_string()],
                start_x: 0.0,
                start_y: 0.0,
                end_x: 100.0,
                end_y: 0.0,
            }),
            glow: None,
        },
        border: BorderEffects {
            width: 4.0,
            color: "#ec4899".to_string(),
            radius: 20.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

/// Minimalist theme - Clean and simple
pub fn create_minimalist_theme() -> Theme {
    let mut theme = Theme::new(
        "minimalist".to_string(),
        "Minimalist Clean".to_string(),
        "Ultra-clean, minimalist theme with maximum focus on content".to_string(),
        ThemeCategory::Minimalist,
    );

    // Minimalist color palette
    theme.colors = ColorPalette {
        primary: "#000000".to_string(),    // Black
        secondary: "#666666".to_string(),  // Gray
        background: "#ffffff".to_string(), // White
        text: "#000000".to_string(),       // Black
        accent: "#000000".to_string(),     // Black
        additional: vec![
            "#f5f5f5".to_string(), // Light gray
            "#e5e5e5".to_string(), // Very light gray
        ],
    };

    // Minimalist typography
    theme.typography = Typography {
        primary_font: "Helvetica, sans-serif".to_string(),
        secondary_font: "Helvetica, sans-serif".to_string(),
        title_size: 44,
        description_size: 20,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        },
    };

    // Minimalist layout
    theme.layout = Layout {
        padding: 100,
        spacing: 50,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::None,
        text_alignment: TextAlignment::Center,
    };

    // Minimalist effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Solid,
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
    };

    theme
}

/// Bold theme - High contrast and impactful
pub fn create_bold_theme() -> Theme {
    let mut theme = Theme::new(
        "bold".to_string(),
        "Bold Impact".to_string(),
        "High-contrast, bold theme for maximum visual impact".to_string(),
        ThemeCategory::Bold,
    );

    // Bold color palette
    theme.colors = ColorPalette {
        primary: "#ff0000".to_string(),    // Red
        secondary: "#000000".to_string(),  // Black
        background: "#ffff00".to_string(), // Yellow
        text: "#000000".to_string(),       // Black
        accent: "#ff0000".to_string(),     // Red
        additional: vec![
            "#ffffff".to_string(), // White
            "#0000ff".to_string(), // Blue
        ],
    };

    // Bold typography
    theme.typography = Typography {
        primary_font: "Impact, sans-serif".to_string(),
        secondary_font: "Arial Black, sans-serif".to_string(),
        title_size: 60,
        description_size: 28,
        weights: FontWeights {
            light: 400,
            normal: 700,
            medium: 800,
            bold: 900,
        },
    };

    // Bold layout
    theme.layout = Layout {
        padding: 40,
        spacing: 25,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::TopLeft,
        text_alignment: TextAlignment::Center,
    };

    // Bold effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Solid,
            gradient: None,
            pattern: None,
            blur: None,
        },
        text: TextEffects {
            shadow: Some(TextShadow {
                color: "#000000".to_string(),
                blur: 6.0,
                offset_x: 3.0,
                offset_y: 3.0,
            }),
            outline: Some(TextOutline {
                color: "#ffffff".to_string(),
                width: 3.0,
            }),
            gradient: None,
            glow: None,
        },
        border: BorderEffects {
            width: 8.0,
            color: "#000000".to_string(),
            radius: 0.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

/// Elegant theme - Sophisticated and refined
pub fn create_elegant_theme() -> Theme {
    let mut theme = Theme::new(
        "elegant".to_string(),
        "Elegant Sophisticated".to_string(),
        "Refined, elegant theme with sophisticated typography and subtle effects".to_string(),
        ThemeCategory::Elegant,
    );

    // Elegant color palette
    theme.colors = ColorPalette {
        primary: "#2d3748".to_string(),    // Dark gray
        secondary: "#4a5568".to_string(),  // Medium gray
        background: "#f7fafc".to_string(), // Very light gray
        text: "#2d3748".to_string(),       // Dark gray
        accent: "#d69e2e".to_string(),     // Gold
        additional: vec![
            "#e2e8f0".to_string(), // Light gray
            "#a0aec0".to_string(), // Medium light gray
        ],
    };

    // Elegant typography
    theme.typography = Typography {
        primary_font: "Playfair Display, serif".to_string(),
        secondary_font: "Source Sans Pro, sans-serif".to_string(),
        title_size: 50,
        description_size: 22,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        },
    };

    // Elegant layout
    theme.layout = Layout {
        padding: 80,
        spacing: 45,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::TopRight,
        text_alignment: TextAlignment::Center,
    };

    // Elegant effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Pattern,
            gradient: None,
            pattern: Some(PatternConfig {
                pattern_type: PatternType::Lines,
                color: "#d69e2e".to_string(),
                opacity: 0.05,
                size: 30,
            }),
            blur: None,
        },
        text: TextEffects {
            shadow: Some(TextShadow {
                color: "#000000".to_string(),
                blur: 1.0,
                offset_x: 0.0,
                offset_y: 1.0,
            }),
            outline: None,
            gradient: None,
            glow: None,
        },
        border: BorderEffects {
            width: 1.0,
            color: "#d69e2e".to_string(),
            radius: 4.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

/// Modern theme - Contemporary and sleek
pub fn create_modern_theme() -> Theme {
    let mut theme = Theme::new(
        "modern".to_string(),
        "Modern Contemporary".to_string(),
        "Contemporary, sleek theme with modern design principles".to_string(),
        ThemeCategory::Modern,
    );

    // Modern color palette
    theme.colors = ColorPalette {
        primary: "#3b82f6".to_string(),    // Blue
        secondary: "#1e40af".to_string(),  // Dark blue
        background: "#f8fafc".to_string(), // Light gray
        text: "#1e293b".to_string(),       // Dark slate
        accent: "#06b6d4".to_string(),     // Cyan
        additional: vec![
            "#64748b".to_string(), // Slate
            "#e2e8f0".to_string(), // Light slate
        ],
    };

    // Modern typography
    theme.typography = Typography {
        primary_font: "Inter, sans-serif".to_string(),
        secondary_font: "Inter, sans-serif".to_string(),
        title_size: 48,
        description_size: 24,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 600,
        },
    };

    // Modern layout
    theme.layout = Layout {
        padding: 60,
        spacing: 30,
        alignment: ContentAlignment::Left,
        logo_position: LogoPosition::TopLeft,
        text_alignment: TextAlignment::Left,
    };

    // Modern effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Gradient,
            gradient: Some(GradientConfig {
                gradient_type: GradientType::Linear,
                colors: vec!["#f8fafc".to_string(), "#e2e8f0".to_string()],
                angle: 45.0,
                stops: vec![0.0, 1.0],
            }),
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
            radius: 12.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

/// Classic theme - Traditional and timeless
pub fn create_classic_theme() -> Theme {
    let mut theme = Theme::new(
        "classic".to_string(),
        "Classic Traditional".to_string(),
        "Traditional, timeless theme with classic typography and colors".to_string(),
        ThemeCategory::Classic,
    );

    // Classic color palette
    theme.colors = ColorPalette {
        primary: "#1f2937".to_string(),    // Dark gray
        secondary: "#374151".to_string(),  // Medium gray
        background: "#ffffff".to_string(), // White
        text: "#1f2937".to_string(),       // Dark gray
        accent: "#dc2626".to_string(),     // Red
        additional: vec![
            "#6b7280".to_string(), // Light gray
            "#f3f4f6".to_string(), // Very light gray
        ],
    };

    // Classic typography
    theme.typography = Typography {
        primary_font: "Times New Roman, serif".to_string(),
        secondary_font: "Georgia, serif".to_string(),
        title_size: 46,
        description_size: 22,
        weights: FontWeights {
            light: 300,
            normal: 400,
            medium: 500,
            bold: 700,
        },
    };

    // Classic layout
    theme.layout = Layout {
        padding: 70,
        spacing: 35,
        alignment: ContentAlignment::Center,
        logo_position: LogoPosition::TopLeft,
        text_alignment: TextAlignment::Center,
    };

    // Classic effects
    theme.effects = VisualEffects {
        background: super::BackgroundEffects {
            background_type: super::BackgroundType::Solid,
            gradient: None,
            pattern: None,
            blur: None,
        },
        text: TextEffects {
            shadow: Some(TextShadow {
                color: "#000000".to_string(),
                blur: 1.0,
                offset_x: 0.0,
                offset_y: 1.0,
            }),
            outline: None,
            gradient: None,
            glow: None,
        },
        border: BorderEffects {
            width: 2.0,
            color: "#dc2626".to_string(),
            radius: 0.0,
            style: BorderStyle::Solid,
        },
    };

    theme
}

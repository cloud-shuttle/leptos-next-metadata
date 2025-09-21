//! Tests for the Theme Support system

use leptos_next_metadata::prelude::*;
use leptos_next_metadata::themes::{
    create_predefined_themes, BackgroundType, LogoPosition as ThemeLogoPosition, Typography,
};
use serde_json;

#[cfg(not(target_arch = "wasm32"))]
mod native_tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new(
            "test-theme".to_string(),
            "Test Theme".to_string(),
            "A test theme for unit testing".to_string(),
            ThemeCategory::Custom,
        );

        assert_eq!(theme.id, "test-theme");
        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.category, ThemeCategory::Custom);
        assert_eq!(theme.version, "1.0.0");
    }

    #[test]
    fn test_theme_validation() {
        let mut theme = Theme::new(
            "".to_string(), // Empty ID should fail
            "Test Theme".to_string(),
            "A test theme".to_string(),
            ThemeCategory::Custom,
        );

        assert!(theme.validate().is_err());

        theme.id = "valid-id".to_string();
        theme.name = "".to_string(); // Empty name should fail
        assert!(theme.validate().is_err());

        theme.name = "Valid Name".to_string();
        theme.colors.primary = "invalid-color".to_string(); // Invalid color should fail
        assert!(theme.validate().is_err());

        theme.colors.primary = "#ff0000".to_string();
        theme.typography.title_size = 5; // Too small should fail
        assert!(theme.validate().is_err());

        theme.typography.title_size = 50;
        assert!(theme.validate().is_ok());
    }

    #[test]
    fn test_theme_serialization() {
        let theme = Theme::new(
            "serialization-test".to_string(),
            "Serialization Test".to_string(),
            "Testing theme serialization".to_string(),
            ThemeCategory::Business,
        );

        let serialized = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, "serialization-test");
        assert_eq!(deserialized.name, "Serialization Test");
        assert_eq!(deserialized.category, ThemeCategory::Business);
    }

    #[test]
    fn test_theme_builder() {
        let theme = ThemeBuilder::new(
            "builder-test".to_string(),
            "Builder Test".to_string(),
            "Testing theme builder".to_string(),
            ThemeCategory::Creative,
        )
        .primary_color("#ff0000".to_string())
        .background_color("#ffffff".to_string())
        .text_color("#000000".to_string())
        .title_size(60)
        .description_size(30)
        .padding(80)
        .spacing(40)
        .build()
        .unwrap();

        assert_eq!(theme.id, "builder-test");
        assert_eq!(theme.colors.primary, "#ff0000");
        assert_eq!(theme.colors.background, "#ffffff");
        assert_eq!(theme.typography.title_size, 60);
        assert_eq!(theme.layout.padding, 80);
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();

        // Add themes
        let theme1 = Theme::new(
            "theme1".to_string(),
            "Theme 1".to_string(),
            "First theme".to_string(),
            ThemeCategory::Business,
        );
        let theme2 = Theme::new(
            "theme2".to_string(),
            "Theme 2".to_string(),
            "Second theme".to_string(),
            ThemeCategory::Creative,
        );

        manager.add_theme(theme1).unwrap();
        manager.add_theme(theme2).unwrap();

        assert_eq!(manager.theme_count(), 2);

        // Get theme
        let retrieved_theme = manager.get_theme(&"theme1".to_string());
        assert!(retrieved_theme.is_some());
        assert_eq!(retrieved_theme.unwrap().name, "Theme 1");

        // Set default theme
        manager.set_default_theme("theme1".to_string()).unwrap();
        let default_theme = manager.get_default_theme();
        assert!(default_theme.is_some());
        assert_eq!(default_theme.unwrap().id, "theme1");

        // Get themes by category
        let business_themes = manager.get_themes_by_category(&ThemeCategory::Business);
        assert_eq!(business_themes.len(), 1);
        assert_eq!(business_themes[0].id, "theme1");

        // Search themes
        let search_results = manager.search_themes("Theme 2");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "theme2");

        // Remove theme
        let removed_theme = manager.remove_theme(&"theme1".to_string());
        assert!(removed_theme.is_some());
        assert_eq!(manager.theme_count(), 1);
    }

    #[test]
    fn test_predefined_themes() {
        let manager = create_predefined_themes();

        // Check that we have predefined themes
        assert!(manager.theme_count() > 0);

        // Check specific themes exist
        assert!(manager.get_theme(&"business".to_string()).is_some());
        assert!(manager.get_theme(&"tech".to_string()).is_some());
        assert!(manager.get_theme(&"creative".to_string()).is_some());
        assert!(manager.get_theme(&"minimalist".to_string()).is_some());

        // Check theme categories
        let business_themes = manager.get_themes_by_category(&ThemeCategory::Business);
        assert!(!business_themes.is_empty());

        let tech_themes = manager.get_themes_by_category(&ThemeCategory::Technology);
        assert!(!tech_themes.is_empty());
    }

    #[test]
    fn test_theme_helpers() {
        // Test color validation
        assert!(ThemeHelpers::is_valid_hex_color("#ff0000"));
        assert!(ThemeHelpers::is_valid_hex_color("#000000"));
        assert!(ThemeHelpers::is_valid_hex_color("#ffffff"));
        assert!(!ThemeHelpers::is_valid_hex_color("ff0000")); // Missing #
        assert!(!ThemeHelpers::is_valid_hex_color("#ff00")); // Too short
        assert!(!ThemeHelpers::is_valid_hex_color("#gg0000")); // Invalid hex

        // Test RGB to hex conversion
        let hex = ThemeHelpers::rgb_to_hex(255, 0, 0);
        assert_eq!(hex, "#ff0000");

        let hex = ThemeHelpers::rgb_to_hex(0, 255, 0);
        assert_eq!(hex, "#00ff00");

        // Test hex to RGB conversion
        let rgb = ThemeHelpers::hex_to_rgb("#ff0000").unwrap();
        assert_eq!(rgb, (255, 0, 0));

        let rgb = ThemeHelpers::hex_to_rgb("#00ff00").unwrap();
        assert_eq!(rgb, (0, 255, 0));

        // Test complementary color
        let complementary = ThemeHelpers::get_complementary_color("#ff0000").unwrap();
        assert_eq!(complementary, "#00ffff");
    }

    #[test]
    fn test_quick_themes() {
        // Test dark theme
        let dark_theme =
            QuickThemes::dark_theme("dark-test".to_string(), "Dark Test".to_string()).unwrap();
        assert_eq!(dark_theme.colors.background, "#1a1a1a");
        assert_eq!(dark_theme.colors.text, "#ffffff");

        // Test light theme
        let light_theme =
            QuickThemes::light_theme("light-test".to_string(), "Light Test".to_string()).unwrap();
        assert_eq!(light_theme.colors.background, "#ffffff");
        assert_eq!(light_theme.colors.text, "#1f2937");

        // Test monochrome theme
        let mono_theme =
            QuickThemes::monochrome_theme("mono-test".to_string(), "Mono Test".to_string())
                .unwrap();
        assert_eq!(mono_theme.colors.background, "#ffffff");
        assert_eq!(mono_theme.colors.text, "#000000");
        assert_eq!(mono_theme.colors.primary, "#000000");

        // Test colorful theme
        let colorful_theme =
            QuickThemes::colorful_theme("colorful-test".to_string(), "Colorful Test".to_string())
                .unwrap();
        assert_eq!(colorful_theme.colors.background, "#fef3c7");
        assert_eq!(colorful_theme.colors.primary, "#ec4899");
        assert_eq!(colorful_theme.category, ThemeCategory::Creative);
    }

    #[test]
    fn test_theme_effects() {
        let mut theme = Theme::new(
            "effects-test".to_string(),
            "Effects Test".to_string(),
            "Testing theme effects".to_string(),
            ThemeCategory::Creative,
        );

        // Test text shadow
        let shadow = ThemeHelpers::simple_text_shadow("#000000".to_string(), 4.0, 2.0, 2.0);
        theme.effects.text.shadow = Some(shadow);

        // Test text outline
        let outline = ThemeHelpers::simple_text_outline("#ffffff".to_string(), 2.0);
        theme.effects.text.outline = Some(outline);

        // Test text gradient
        let gradient = ThemeHelpers::linear_text_gradient(
            vec!["#ff0000".to_string(), "#0000ff".to_string()],
            0.0,
            0.0,
            100.0,
            0.0,
        );
        theme.effects.text.gradient = Some(gradient);

        // Test background gradient
        let bg_gradient = ThemeHelpers::linear_background_gradient(
            vec!["#ffffff".to_string(), "#f0f0f0".to_string()],
            45.0,
            vec![0.0, 1.0],
        );
        theme.effects.background.gradient = Some(bg_gradient);
        theme.effects.background.background_type = BackgroundType::Gradient;

        // Test pattern
        let pattern = ThemeHelpers::dot_pattern("#cccccc".to_string(), 0.1, 20);
        theme.effects.background.pattern = Some(pattern);

        // Test border
        let border = ThemeHelpers::simple_border(2.0, "#000000".to_string(), 8.0);
        theme.effects.border = border;

        // Validate the theme
        assert!(theme.validate().is_ok());
    }

    #[test]
    fn test_theme_application_to_canvas() {
        let theme = Theme::new(
            "canvas-test".to_string(),
            "Canvas Test".to_string(),
            "Testing theme application to canvas".to_string(),
            ThemeCategory::Business,
        );

        let canvas_params = CanvasOgParams {
            title: "Test Title".to_string(),
            description: Some("Test Description".to_string()),
            width: Some(1200),
            height: Some(630),
            background_color: None,
            text_color: None,
            font_family: None,
            title_font_size: None,
            description_font_size: None,
            logo_url: None,
            font_urls: None,
            default_font_family: None,
            layers: None,
            background_image_url: None,
            background_image_opacity: None,
            text_gradient: None,
            text_shadow: None,
            text_outline: None,
            logo_position: None,
            text_align: None,
            padding: None,
        };

        let applied_params = theme.apply_to_canvas_params(canvas_params);

        // Check that theme values were applied
        assert_eq!(
            applied_params.background_color,
            Some(theme.colors.background)
        );
        assert_eq!(applied_params.text_color, Some(theme.colors.text));
        assert_eq!(
            applied_params.font_family,
            Some(theme.typography.primary_font)
        );
        assert_eq!(
            applied_params.title_font_size,
            Some(theme.typography.title_size)
        );
        assert_eq!(
            applied_params.description_font_size,
            Some(theme.typography.description_size)
        );
        assert_eq!(applied_params.padding, Some(theme.layout.padding));
    }

    #[test]
    fn test_theme_preview_generation() {
        let theme = Theme::new(
            "preview-test".to_string(),
            "Preview Test".to_string(),
            "Testing theme preview generation".to_string(),
            ThemeCategory::Modern,
        );

        let preview = theme.generate_preview().unwrap();
        assert!(preview.starts_with("data:image/svg+xml;base64,"));
        assert!(preview.contains("Preview Test Theme Preview"));
    }

    #[test]
    fn test_theme_cloning() {
        let original_theme = Theme::new(
            "original".to_string(),
            "Original Theme".to_string(),
            "Original theme for cloning test".to_string(),
            ThemeCategory::Business,
        );

        let cloned_theme =
            original_theme.clone_with_id("cloned".to_string(), "Cloned Theme".to_string());

        assert_eq!(cloned_theme.id, "cloned");
        assert_eq!(cloned_theme.name, "Cloned Theme");
        assert_eq!(cloned_theme.colors.primary, original_theme.colors.primary);
        assert_eq!(
            cloned_theme.typography.title_size,
            original_theme.typography.title_size
        );
        assert_ne!(
            cloned_theme.metadata.modified_at,
            original_theme.metadata.modified_at
        );
    }

    #[test]
    fn test_theme_version_update() {
        let mut theme = Theme::new(
            "version-test".to_string(),
            "Version Test".to_string(),
            "Testing theme version updates".to_string(),
            ThemeCategory::Custom,
        );

        let original_modified = theme.metadata.modified_at.clone();
        theme.update_version("2.0.0".to_string());

        assert_eq!(theme.version, "2.0.0");
        assert_ne!(theme.metadata.modified_at, original_modified);
    }

    #[test]
    fn test_theme_category_display() {
        assert_eq!(ThemeCategory::Business.to_string(), "Business");
        assert_eq!(ThemeCategory::Technology.to_string(), "Technology");
        assert_eq!(ThemeCategory::Creative.to_string(), "Creative");
        assert_eq!(ThemeCategory::Minimalist.to_string(), "Minimalist");
        assert_eq!(ThemeCategory::Bold.to_string(), "Bold");
        assert_eq!(ThemeCategory::Elegant.to_string(), "Elegant");
        assert_eq!(ThemeCategory::Modern.to_string(), "Modern");
        assert_eq!(ThemeCategory::Classic.to_string(), "Classic");
        assert_eq!(ThemeCategory::Custom.to_string(), "Custom");
    }

    #[test]
    fn test_color_palette_defaults() {
        let palette = ColorPalette::default();
        assert_eq!(palette.primary, "#4f46e5");
        assert_eq!(palette.secondary, "#7c3aed");
        assert_eq!(palette.background, "#ffffff");
        assert_eq!(palette.text, "#1f2937");
        assert_eq!(palette.accent, "#f59e0b");
        assert_eq!(palette.additional.len(), 2);
    }

    #[test]
    fn test_typography_defaults() {
        let typography = Typography::default();
        assert_eq!(typography.primary_font, "Inter, sans-serif");
        assert_eq!(typography.secondary_font, "Georgia, serif");
        assert_eq!(typography.title_size, 48);
        assert_eq!(typography.description_size, 24);
        assert_eq!(typography.weights.light, 300);
        assert_eq!(typography.weights.normal, 400);
        assert_eq!(typography.weights.medium, 500);
        assert_eq!(typography.weights.bold, 700);
    }

    #[test]
    fn test_layout_defaults() {
        let layout = Layout::default();
        assert_eq!(layout.padding, 60);
        assert_eq!(layout.spacing, 30);
        assert!(matches!(layout.alignment, ContentAlignment::Center));
        assert!(matches!(layout.logo_position, ThemeLogoPosition::None));
        assert!(matches!(layout.text_alignment, TextAlignment::Center));
    }

    #[test]
    fn test_visual_effects_defaults() {
        let effects = VisualEffects::default();
        assert!(matches!(
            effects.background.background_type,
            BackgroundType::Solid
        ));
        assert!(effects.background.gradient.is_none());
        assert!(effects.background.pattern.is_none());
        assert!(effects.background.blur.is_none());
        assert!(effects.text.shadow.is_none());
        assert!(effects.text.outline.is_none());
        assert!(effects.text.gradient.is_none());
        assert!(effects.text.glow.is_none());
        assert_eq!(effects.border.width, 0.0);
        assert_eq!(effects.border.color, "#000000");
        assert_eq!(effects.border.radius, 0.0);
        assert!(matches!(effects.border.style, BorderStyle::Solid));
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_theme_creation_wasm() {
        let theme = Theme::new(
            "wasm-theme".to_string(),
            "WASM Theme".to_string(),
            "A theme created in WASM".to_string(),
            ThemeCategory::Modern,
        );

        assert_eq!(theme.id, "wasm-theme");
        assert_eq!(theme.name, "WASM Theme");
        assert_eq!(theme.category, ThemeCategory::Modern);
    }

    #[wasm_bindgen_test]
    fn test_theme_manager_wasm() {
        let mut manager = ThemeManager::new();

        let theme = Theme::new(
            "wasm-manager-test".to_string(),
            "WASM Manager Test".to_string(),
            "Testing theme manager in WASM".to_string(),
            ThemeCategory::Technology,
        );

        manager.add_theme(theme).unwrap();
        assert_eq!(manager.theme_count(), 1);

        let retrieved_theme = manager.get_theme(&"wasm-manager-test".to_string());
        assert!(retrieved_theme.is_some());
        assert_eq!(retrieved_theme.unwrap().name, "WASM Manager Test");
    }

    #[wasm_bindgen_test]
    fn test_predefined_themes_wasm() {
        let manager = create_predefined_themes();
        assert!(manager.theme_count() > 0);

        let business_theme = manager.get_theme(&"business".to_string());
        assert!(business_theme.is_some());
        assert_eq!(business_theme.unwrap().category, ThemeCategory::Business);
    }

    #[wasm_bindgen_test]
    fn test_theme_builder_wasm() {
        let theme = ThemeBuilder::new(
            "wasm-builder".to_string(),
            "WASM Builder".to_string(),
            "Testing builder in WASM".to_string(),
            ThemeCategory::Creative,
        )
        .primary_color("#ff0000".to_string())
        .background_color("#ffffff".to_string())
        .title_size(50)
        .build()
        .unwrap();

        assert_eq!(theme.id, "wasm-builder");
        assert_eq!(theme.colors.primary, "#ff0000");
        assert_eq!(theme.colors.background, "#ffffff");
        assert_eq!(theme.typography.title_size, 50);
    }
}

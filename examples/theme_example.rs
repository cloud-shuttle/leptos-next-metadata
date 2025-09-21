//! Theme System Example
//!
//! Demonstrates how to use the comprehensive theme system for OG image generation
//! including predefined themes, custom theme creation, and theme management.

use leptos::*;
use leptos_next_metadata::prelude::*;
use leptos_next_metadata::themes::create_predefined_themes;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn run_theme_example() -> Result<JsValue, JsValue> {
    // Initialize a theme manager with predefined themes
    let mut theme_manager = create_predefined_themes();

    // Example 1: Use a predefined theme
    let business_theme = theme_manager
        .get_theme(&"business".to_string())
        .ok_or_else(|| JsValue::from_str("Business theme not found"))?;

    let canvas_params = CanvasOgParams {
        title: "My Business Blog Post".to_string(),
        description: Some("Learn about the latest trends in business technology".to_string()),
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

    let themed_params = business_theme.apply_to_canvas_params(canvas_params);

    // Example 2: Create a custom theme using the builder
    let custom_theme = ThemeBuilder::new(
        "my-custom-theme".to_string(),
        "My Custom Theme".to_string(),
        "A custom theme created with the builder".to_string(),
        ThemeCategory::Creative,
    )
    .primary_color("#ff6b6b".to_string())
    .secondary_color("#4ecdc4".to_string())
    .background_color("#f7f7f7".to_string())
    .text_color("#2c3e50".to_string())
    .accent_color("#f39c12".to_string())
    .primary_font("Poppins, sans-serif".to_string())
    .title_size(56)
    .description_size(28)
    .padding(60)
    .spacing(35)
    .content_alignment(ContentAlignment::Center)
    .logo_position(ThemeLogoPosition::TopRight)
    .text_alignment(TextAlignment::Center)
    .text_shadow(ThemeHelpers::simple_text_shadow(
        "#000000".to_string(),
        3.0,
        1.0,
        1.0,
    ))
    .text_outline(ThemeHelpers::simple_text_outline(
        "#ffffff".to_string(),
        1.0,
    ))
    .background_gradient(ThemeHelpers::linear_background_gradient(
        vec!["#f7f7f7".to_string(), "#e8e8e8".to_string()],
        135.0,
        vec![0.0, 1.0],
    ))
    .background_type(BackgroundType::Gradient)
    .border(ThemeHelpers::simple_border(
        2.0,
        "#ff6b6b".to_string(),
        12.0,
    ))
    .author("John Doe".to_string())
    .tags(vec![
        "custom".to_string(),
        "creative".to_string(),
        "modern".to_string(),
    ])
    .license("MIT".to_string())
    .build()
    .map_err(|e| JsValue::from_str(&format!("Failed to build custom theme: {}", e.message)))?;

    // Add the custom theme to the manager
    theme_manager
        .add_theme(custom_theme.clone())
        .map_err(|e| JsValue::from_str(&format!("Failed to add custom theme: {}", e.message)))?;

    // Example 3: Use quick theme creation
    let dark_theme =
        QuickThemes::dark_theme("my-dark-theme".to_string(), "My Dark Theme".to_string()).map_err(
            |e| JsValue::from_str(&format!("Failed to create dark theme: {}", e.message)),
        )?;

    theme_manager
        .add_theme(dark_theme.clone())
        .map_err(|e| JsValue::from_str(&format!("Failed to add dark theme: {}", e.message)))?;

    // Example 4: Search and filter themes
    let creative_themes = theme_manager.get_themes_by_category(&ThemeCategory::Creative);
    let search_results = theme_manager.search_themes("custom");

    // Example 5: Generate theme previews
    let business_preview = business_theme.generate_preview().map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to generate business theme preview: {}",
            e.message
        ))
    })?;

    let custom_preview = custom_theme.generate_preview().map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to generate custom theme preview: {}",
            e.message
        ))
    })?;

    // Example 6: Theme validation and cloning
    let cloned_theme = custom_theme.clone_with_id(
        "cloned-custom-theme".to_string(),
        "Cloned Custom Theme".to_string(),
    );

    let validation_result = cloned_theme.validate();

    // Example 7: Color manipulation with helpers
    let primary_color = "#ff6b6b";
    let complementary_color =
        ThemeHelpers::get_complementary_color(primary_color).map_err(|e| {
            JsValue::from_str(&format!("Failed to get complementary color: {}", e.message))
        })?;

    let analogous_colors = ThemeHelpers::get_analogous_colors(primary_color).map_err(|e| {
        JsValue::from_str(&format!("Failed to get analogous colors: {}", e.message))
    })?;

    // Example 8: Create a themed OG image
    let themed_og_params = CanvasOgParams {
        title: "Themed OG Image".to_string(),
        description: Some("This image was generated using our custom theme".to_string()),
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

    let final_themed_params = custom_theme.apply_to_canvas_params(themed_og_params);

    // Return comprehensive results
    let results = json!({
        "themeManager": {
            "totalThemes": theme_manager.theme_count(),
            "predefinedThemes": theme_manager.get_all_themes().len(),
        },
        "businessTheme": {
            "id": business_theme.id,
            "name": business_theme.name,
            "category": business_theme.category.to_string(),
            "colors": {
                "primary": business_theme.colors.primary,
                "background": business_theme.colors.background,
                "text": business_theme.colors.text,
            },
            "typography": {
                "primaryFont": business_theme.typography.primary_font,
                "titleSize": business_theme.typography.title_size,
            },
            "preview": business_preview,
        },
        "customTheme": {
            "id": custom_theme.id,
            "name": custom_theme.name,
            "category": custom_theme.category.to_string(),
            "colors": {
                "primary": custom_theme.colors.primary,
                "secondary": custom_theme.colors.secondary,
                "background": custom_theme.colors.background,
                "text": custom_theme.colors.text,
                "accent": custom_theme.colors.accent,
            },
            "typography": {
                "primaryFont": custom_theme.typography.primary_font,
                "titleSize": custom_theme.typography.title_size,
                "descriptionSize": custom_theme.typography.description_size,
            },
            "layout": {
                "padding": custom_theme.layout.padding,
                "spacing": custom_theme.layout.spacing,
                "alignment": format!("{:?}", custom_theme.layout.alignment),
            },
            "effects": {
                "hasTextShadow": custom_theme.effects.text.shadow.is_some(),
                "hasTextOutline": custom_theme.effects.text.outline.is_some(),
                "hasBackgroundGradient": custom_theme.effects.background.gradient.is_some(),
                "hasBorder": custom_theme.effects.border.width > 0.0,
            },
            "metadata": {
                "author": custom_theme.metadata.author,
                "tags": custom_theme.metadata.tags,
                "license": custom_theme.metadata.license,
            },
            "preview": custom_preview,
        },
        "darkTheme": {
            "id": dark_theme.id,
            "name": dark_theme.name,
            "category": dark_theme.category.to_string(),
            "colors": {
                "background": dark_theme.colors.background,
                "text": dark_theme.colors.text,
            },
        },
        "themeSearch": {
            "creativeThemesCount": creative_themes.len(),
            "searchResultsCount": search_results.len(),
        },
        "colorManipulation": {
            "primaryColor": primary_color,
            "complementaryColor": complementary_color,
            "analogousColors": analogous_colors,
        },
        "themedCanvasParams": {
            "title": final_themed_params.title,
            "description": final_themed_params.description,
            "backgroundColor": final_themed_params.background_color,
            "textColor": final_themed_params.text_color,
            "fontFamily": final_themed_params.font_family,
            "titleFontSize": final_themed_params.title_font_size,
            "padding": final_themed_params.padding,
        },
        "validation": {
            "clonedThemeValid": validation_result.is_ok(),
            "validationError": validation_result.err().map(|e| e.message),
        },
        "message": "Theme system example completed successfully!",
    });

    #[cfg(target_arch = "wasm32")]
    Ok(serde_wasm_bindgen::to_value(&results)?)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Theme system example is only available in WASM environments");
    println!("To use this example:");
    println!("1. Build the project with wasm-pack");
    println!("2. Load the generated WASM module in a browser");
    println!("3. Call the run_theme_example() function");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn initialize_theme_example() {
    web_sys::console::log_1(&"Theme system example loaded successfully!".into());
}

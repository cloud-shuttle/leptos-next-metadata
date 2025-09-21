//! Advanced Canvas Features Tests
//!
//! Tests for the new v1.5.0 advanced canvas features including:
//! - Layer-based composition
//! - Text effects (gradients, shadows, outlines)
//! - Shape drawing
//! - Template system
//! - Custom font loading

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use leptos_next_metadata::prelude::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_advanced_canvas_layers() {
        // Test layer-based composition
        let text_layer = TextLayer {
            content: "Test Title".to_string(),
            font_family: "Arial".to_string(),
            font_size: 48,
            color: "#ffffff".to_string(),
            x: 100.0,
            y: 100.0,
            max_width: 800.0,
            line_height: 60.0,
            text_align: TextAlign::Center,
            z_index: 1,
            gradient: None,
            shadow: None,
            outline: None,
        };

        let shape_layer = ShapeLayer {
            shape_type: ShapeType::Rectangle,
            x: 50.0,
            y: 50.0,
            width: 900.0,
            height: 200.0,
            fill_color: Some("#4f46e5".to_string()),
            stroke_color: None,
            stroke_width: None,
            z_index: 0,
        };

        let layers = vec![
            OgImageLayer::Shape(shape_layer),
            OgImageLayer::Text(text_layer),
        ];

        // Test that layers can be created and serialized
        let serialized = serde_json::to_string(&layers).unwrap();
        let deserialized: Vec<OgImageLayer> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 2);
    }

    #[wasm_bindgen_test]
    fn test_text_effects() {
        // Test text gradient
        let gradient = TextGradient {
            gradient_type: GradientType::Linear,
            colors: vec!["#ff0000".to_string(), "#0000ff".to_string()],
            start_x: 0.0,
            start_y: 0.0,
            end_x: 100.0,
            end_y: 0.0,
        };

        // Test text shadow
        let shadow = TextShadow {
            color: "#000000".to_string(),
            blur: 5.0,
            offset_x: 2.0,
            offset_y: 2.0,
        };

        // Test text outline
        let outline = TextOutline {
            color: "#ffffff".to_string(),
            width: 2.0,
        };

        let text_layer = TextLayer {
            content: "Styled Text".to_string(),
            font_family: "Arial".to_string(),
            font_size: 36,
            color: "#000000".to_string(),
            x: 200.0,
            y: 200.0,
            max_width: 600.0,
            line_height: 45.0,
            text_align: TextAlign::Center,
            z_index: 1,
            gradient: Some(gradient),
            shadow: Some(shadow),
            outline: Some(outline),
        };

        // Test serialization
        let serialized = serde_json::to_string(&text_layer).unwrap();
        let deserialized: TextLayer = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.content, "Styled Text");
        assert!(deserialized.gradient.is_some());
        assert!(deserialized.shadow.is_some());
        assert!(deserialized.outline.is_some());
    }

    #[wasm_bindgen_test]
    fn test_shape_types() {
        // Test rectangle
        let rectangle = ShapeLayer {
            shape_type: ShapeType::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
            fill_color: Some("#ff0000".to_string()),
            stroke_color: Some("#000000".to_string()),
            stroke_width: Some(2.0),
            z_index: 0,
        };

        // Test circle
        let circle = ShapeLayer {
            shape_type: ShapeType::Circle,
            x: 100.0,
            y: 100.0,
            width: 80.0,
            height: 80.0,
            fill_color: Some("#00ff00".to_string()),
            stroke_color: None,
            stroke_width: None,
            z_index: 1,
        };

        // Test line
        let line = ShapeLayer {
            shape_type: ShapeType::Line {
                x2: 200.0,
                y2: 200.0,
            },
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fill_color: None,
            stroke_color: Some("#0000ff".to_string()),
            stroke_width: Some(3.0),
            z_index: 2,
        };

        let layers = vec![
            OgImageLayer::Shape(rectangle),
            OgImageLayer::Shape(circle),
            OgImageLayer::Shape(line),
        ];

        // Test serialization
        let serialized = serde_json::to_string(&layers).unwrap();
        let deserialized: Vec<OgImageLayer> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 3);
    }

    #[wasm_bindgen_test]
    fn test_og_image_template() {
        let default_params = CanvasOgParams {
            title: "Default Title".to_string(),
            description: Some("Default Description".to_string()),
            width: Some(1200),
            height: Some(630),
            background_color: Some("#ffffff".to_string()),
            text_color: Some("#000000".to_string()),
            font_family: Some("Arial".to_string()),
            title_font_size: Some(48),
            description_font_size: Some(24),
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

        let template = OgImageTemplate {
            name: "Test Template".to_string(),
            description: "A test template for OG images".to_string(),
            default_params,
            layers: vec![],
            version: "1.0.0".to_string(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&template).unwrap();
        let deserialized: OgImageTemplate = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, "Test Template");
        assert_eq!(deserialized.version, "1.0.0");
    }

    #[wasm_bindgen_test]
    fn test_canvas_og_generator_advanced() {
        // Test that we can create a generator with advanced features
        let generator = CanvasOgGenerator::new();
        assert!(generator.is_ok());

        let mut generator = generator.unwrap();

        // Test setting dimensions
        let result = generator.set_dimensions(1200, 630);
        assert!(result.is_ok());

        let (width, height) = generator.get_dimensions();
        assert_eq!(width, 1200);
        assert_eq!(height, 630);

        // Test setting colors
        generator.set_colors("#4f46e5", "#ffffff");

        // Test setting font
        generator.set_font("Inter, sans-serif");
    }

    #[wasm_bindgen_test]
    fn test_canvas_og_utils() {
        // Test utility functions
        assert!(CanvasOgUtils::is_supported());

        let (width, height) = CanvasOgUtils::get_recommended_dimensions();
        assert_eq!(width, 1200);
        assert_eq!(height, 630);

        // Test parameter validation
        let valid_params = CanvasOgParams {
            title: "Valid Title".to_string(),
            description: None,
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

        let validation_result = CanvasOgUtils::validate_params(&valid_params);
        assert!(validation_result.is_ok());

        // Test invalid parameters
        let invalid_params = CanvasOgParams {
            title: "".to_string(), // Empty title should fail
            description: None,
            width: Some(100),  // Too small
            height: Some(100), // Too small
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

        let validation_result = CanvasOgUtils::validate_params(&invalid_params);
        assert!(validation_result.is_err());
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_tests {
    use leptos_next_metadata::prelude::*;

    #[test]
    fn test_advanced_canvas_types_native() {
        // Test that types can be created and serialized in native environment
        let text_layer = TextLayer {
            content: "Native Test".to_string(),
            font_family: "Arial".to_string(),
            font_size: 24,
            color: "#000000".to_string(),
            x: 0.0,
            y: 0.0,
            max_width: 400.0,
            line_height: 30.0,
            text_align: TextAlign::Left,
            z_index: 0,
            gradient: None,
            shadow: None,
            outline: None,
        };

        let serialized = serde_json::to_string(&text_layer).unwrap();
        let deserialized: TextLayer = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.content, "Native Test");
    }

    #[test]
    fn test_gradient_types() {
        let linear_gradient = TextGradient {
            gradient_type: GradientType::Linear,
            colors: vec!["#ff0000".to_string(), "#00ff00".to_string()],
            start_x: 0.0,
            start_y: 0.0,
            end_x: 100.0,
            end_y: 0.0,
        };

        let radial_gradient = TextGradient {
            gradient_type: GradientType::Radial,
            colors: vec!["#0000ff".to_string(), "#ffff00".to_string()],
            start_x: 50.0,
            start_y: 50.0,
            end_x: 50.0,
            end_y: 50.0,
        };

        // Test serialization
        let linear_serialized = serde_json::to_string(&linear_gradient).unwrap();
        let radial_serialized = serde_json::to_string(&radial_gradient).unwrap();

        let linear_deserialized: TextGradient = serde_json::from_str(&linear_serialized).unwrap();
        let radial_deserialized: TextGradient = serde_json::from_str(&radial_serialized).unwrap();

        assert!(matches!(
            linear_deserialized.gradient_type,
            GradientType::Linear
        ));
        assert!(matches!(
            radial_deserialized.gradient_type,
            GradientType::Radial
        ));
    }

    #[test]
    fn test_shape_types_native() {
        let shapes = vec![
            ShapeType::Rectangle,
            ShapeType::Circle,
            ShapeType::Line {
                x2: 100.0,
                y2: 100.0,
            },
        ];

        for shape in shapes {
            let serialized = serde_json::to_string(&shape).unwrap();
            let deserialized: ShapeType = serde_json::from_str(&serialized).unwrap();

            // Test that the shape type is preserved
            match (&shape, &deserialized) {
                (ShapeType::Rectangle, ShapeType::Rectangle) => {}
                (ShapeType::Circle, ShapeType::Circle) => {}
                (ShapeType::Line { x2: x1, y2: y1 }, ShapeType::Line { x2, y2 }) => {
                    assert_eq!(x1, x2);
                    assert_eq!(y1, y2);
                }
                _ => panic!("Shape type mismatch"),
            }
        }
    }
}

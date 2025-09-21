use leptos::*;
use leptos_next_metadata::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run_analytics_example() -> Result<JsValue, JsValue> {
    // Initialize analytics with custom configuration
    let analytics_config = AnalyticsConfig {
        enabled: true,
        batch_size: 5,
        flush_interval_seconds: 10,
        track_performance: true,
        track_errors: true,
        track_interactions: true,
        privacy: PrivacySettings {
            anonymize_ip: true,
            hash_identifiers: true,
            collect_user_agent: true,
            collect_page_urls: false,
            retention_days: 90,
        },
    };

    let integration_config = IntegrationConfig {
        auto_track: true,
        track_metadata: true,
        track_og_images: true,
        track_themes: true,
        track_validation: true,
        track_errors: true,
        include_performance: true,
        include_user_context: true,
    };

    let mut analytics = MetadataAnalyticsIntegration::new(analytics_config, integration_config);

    // Start a session
    analytics
        .start_session(
            "analytics_example_session".to_string(),
            Some("Analytics Example User Agent".to_string()),
        )
        .map_err(|e| {
            JsValue::from_str(&format!("Failed to start analytics session: {}", e.message))
        })?;

    // Example 1: Track metadata generation
    let mut metadata_properties = HashMap::new();
    metadata_properties.insert(
        "metadata_type".to_string(),
        serde_json::Value::String("blog_post".to_string()),
    );
    metadata_properties.insert(
        "content_length".to_string(),
        serde_json::Value::Number(serde_json::Number::from(1500)),
    );
    metadata_properties.insert("has_images".to_string(), serde_json::Value::Bool(true));

    analytics
        .track_metadata_generation("blog_post_metadata", true, metadata_properties)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to track metadata generation: {}",
                e.message
            ))
        })?;

    // Example 2: Track OG image generation
    let og_params = CanvasOgParams {
        title: "My Amazing Blog Post".to_string(),
        description: Some(
            "This is a description of my amazing blog post that will be used for the OG image."
                .to_string(),
        ),
        width: Some(1200),
        height: Some(630),
        background_color: Some("#1a202c".to_string()),
        text_color: Some("#ffffff".to_string()),
        font_family: Some("Inter, sans-serif".to_string()),
        title_font_size: Some(60),
        description_font_size: Some(30),
        logo_url: Some("https://example.com/logo.png".to_string()),
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

    analytics
        .track_og_image_generation(&og_params, true, 250, Some((1200, 630)))
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to track OG image generation: {}",
                e.message
            ))
        })?;

    // Example 3: Track theme application
    let theme = Theme::new(
        "modern_tech".to_string(),
        "Modern Technology".to_string(),
        "A modern theme with tech-focused colors and typography".to_string(),
        ThemeCategory::Technology,
    );

    analytics
        .track_theme_application(&theme, true, 75)
        .map_err(|e| {
            JsValue::from_str(&format!("Failed to track theme application: {}", e.message))
        })?;

    // Example 4: Track metadata validation
    let validation_errors = vec![
        "Title is too short".to_string(),
        "Description missing".to_string(),
    ];

    analytics
        .track_metadata_validation("blog_post_metadata", false, 50, validation_errors)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to track metadata validation: {}",
                e.message
            ))
        })?;

    // Example 5: Track custom events
    let mut custom_properties = HashMap::new();
    custom_properties.insert(
        "event_category".to_string(),
        serde_json::Value::String("user_engagement".to_string()),
    );
    custom_properties.insert(
        "engagement_score".to_string(),
        serde_json::Value::Number(serde_json::Number::from(85)),
    );
    custom_properties.insert(
        "time_on_page".to_string(),
        serde_json::Value::Number(serde_json::Number::from(120)),
    );

    analytics
        .track_custom_event("high_engagement", custom_properties, Some(100))
        .map_err(|e| JsValue::from_str(&format!("Failed to track custom event: {}", e.message)))?;

    // Example 6: Track performance metrics
    let mut performance_properties = HashMap::new();
    performance_properties.insert(
        "operation_type".to_string(),
        serde_json::Value::String("metadata_processing".to_string()),
    );
    performance_properties.insert(
        "memory_usage_mb".to_string(),
        serde_json::Value::Number(serde_json::Number::from(45)),
    );
    performance_properties.insert(
        "cpu_usage_percent".to_string(),
        serde_json::Value::Number(serde_json::Number::from(25)),
    );

    analytics
        .track_performance(
            "metadata_processing",
            150,
            true,
            Some(1024),
            performance_properties,
        )
        .map_err(|e| JsValue::from_str(&format!("Failed to track performance: {}", e.message)))?;

    // Example 7: Track error events
    let error = MetadataError::new(ErrorKind::Validation, "Invalid metadata format".to_string());
    let mut error_context = HashMap::new();
    error_context.insert(
        "error_source".to_string(),
        serde_json::Value::String("metadata_parser".to_string()),
    );
    error_context.insert(
        "line_number".to_string(),
        serde_json::Value::Number(serde_json::Number::from(42)),
    );
    error_context.insert(
        "file_name".to_string(),
        serde_json::Value::String("blog_post.md".to_string()),
    );

    analytics
        .track_error(error, error_context)
        .map_err(|e| JsValue::from_str(&format!("Failed to track error: {}", e.message)))?;

    // Example 8: Generate insights
    let insights = analytics
        .generate_insights()
        .map_err(|e| JsValue::from_str(&format!("Failed to generate insights: {}", e.message)))?;

    // Example 9: Get performance metrics
    let performance_metrics = analytics.get_performance_metrics();

    // Example 10: Use analytics wrapper for automatic tracking
    let test_data = "important_data".to_string();
    let wrapped_data = test_data.with_analytics(analytics.clone());

    // The wrapper automatically tracks when operations are performed
    let result = wrapped_data.inner();
    assert_eq!(result, "important_data");

    // Example 11: Use analytics context for operation tracking
    let mut context = AnalyticsContext::new("example_operation".to_string(), analytics.clone());

    // Start operation
    context
        .start_operation("example_operation".to_string())
        .map_err(|e| JsValue::from_str(&format!("Failed to start operation: {}", e.message)))?;

    // Simulate some work
    let promise = js_sys::Promise::resolve(&JsValue::undefined());
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

    // Complete operation
    let mut completion_properties = HashMap::new();
    completion_properties.insert(
        "result_size".to_string(),
        serde_json::Value::Number(serde_json::Number::from(2048)),
    );
    completion_properties.insert("success".to_string(), serde_json::Value::Bool(true));

    let duration = context
        .complete_operation("example_operation", true, completion_properties)
        .map_err(|e| JsValue::from_str(&format!("Failed to complete operation: {}", e.message)))?;

    // Example 12: Test error completion
    let error_context = AnalyticsContext::new("error_operation".to_string(), analytics.clone());
    let error = MetadataError::new(ErrorKind::Unknown, "Test error for completion".to_string());
    let mut error_properties = HashMap::new();
    error_properties.insert(
        "error_type".to_string(),
        serde_json::Value::String("test_error".to_string()),
    );

    let error_duration = error_context
        .complete_with_error("error_operation", error, error_properties)
        .map_err(|e| JsValue::from_str(&format!("Failed to complete with error: {}", e.message)))?;

    // Example 13: Test WASM-specific analytics (if available)
    #[cfg(target_arch = "wasm32")]
    {
        let wasm_context = WasmAnalyticsContext::new(analytics.get_config()).map_err(|e| {
            JsValue::from_str(&format!("Failed to create WASM analytics context: {:?}", e))
        })?;

        // Track page view
        wasm_context
            .track_page_view(
                "analytics_example_page".to_string(),
                Some("Analytics Example Page".to_string()),
            )
            .map_err(|e| JsValue::from_str(&format!("Failed to track page view: {:?}", e)))?;

        // Track user interaction
        let mut interaction_properties = HashMap::new();
        interaction_properties.insert(
            "interaction_type".to_string(),
            serde_json::Value::String("button_click".to_string()),
        );
        interaction_properties.insert(
            "button_id".to_string(),
            serde_json::Value::String("example_button".to_string()),
        );

        wasm_context
            .track_user_interaction(
                "click",
                Some("example_button".to_string()),
                interaction_properties,
            )
            .map_err(|e| {
                JsValue::from_str(&format!("Failed to track user interaction: {:?}", e))
            })?;

        // Get browser performance info
        let browser_performance = wasm_context.get_browser_performance_info().map_err(|e| {
            JsValue::from_str(&format!("Failed to get browser performance: {:?}", e))
        })?;

        // End WASM session
        wasm_context
            .end_session()
            .map_err(|e| JsValue::from_str(&format!("Failed to end WASM session: {:?}", e)))?;
    }

    // End the main session
    analytics.end_session().map_err(|e| {
        JsValue::from_str(&format!("Failed to end analytics session: {}", e.message))
    })?;

    // Return comprehensive results
    let results = json!({
        "insights": {
            "performance": {
                "avg_generation_time_ms": insights.performance.avg_generation_time_ms,
                "total_operations": insights.performance.total_operations,
                "error_rate": insights.performance.error_rate,
            },
            "usage": {
                "popular_features": insights.usage.popular_features,
                "feature_usage_count": insights.usage.feature_usage_count,
            },
            "recommendations": insights.recommendations,
        },
        "performance_metrics": {
            "total_operations": performance_metrics.total_operations,
            "error_count": performance_metrics.error_count,
            "success_rate": performance_metrics.success_rate,
            "avg_operation_time_ms": performance_metrics.avg_operation_time_ms,
        },
        "operation_duration": duration,
        "error_operation_duration": error_duration,
        "message": "Analytics integration example completed successfully!",
    });

    Ok(serde_wasm_bindgen::to_value(&results)?)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This is for the example to run in a browser.
    // In a real Leptos app, you'd typically have a main client entry point.
    // For this example, we'll just log that it's loaded.
    web_sys::console::log_1(&"Analytics integration example loaded successfully!".into());
    Ok(())
}

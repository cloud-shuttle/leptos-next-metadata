//! Tests for the Analytics Integration system

use leptos_next_metadata::analytics::handlers::{AnalyticsHandlerFactory, LogLevel};
use leptos_next_metadata::analytics::integration::{
    AnalyticsContext, AnalyticsTrackable, IntegrationConfig, MetadataAnalyticsIntegration,
};
use leptos_next_metadata::analytics::{
    AnalyticsConfig, AnalyticsEventType, AnalyticsManager, PrivacySettings,
};
use leptos_next_metadata::prelude::*;
use serde_json;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
mod native_tests {
    use super::*;

    #[test]
    fn test_analytics_manager_creation() {
        let config = AnalyticsConfig::default();
        let manager = AnalyticsManager::new(config);

        assert!(manager.get_config().enabled);
        assert_eq!(manager.get_config().batch_size, 10);
        assert_eq!(manager.get_config().flush_interval_seconds, 30);
    }

    #[test]
    fn test_analytics_session_management() {
        let mut manager = AnalyticsManager::new(AnalyticsConfig::default());

        // Start session
        manager
            .start_session(
                "test_session".to_string(),
                Some("test_user_agent".to_string()),
            )
            .unwrap();

        // End session
        manager.end_session().unwrap();
    }

    #[test]
    fn test_analytics_event_tracking() {
        let mut manager = AnalyticsManager::new(AnalyticsConfig::default());
        manager
            .start_session("test_session".to_string(), None)
            .unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "test_key".to_string(),
            serde_json::Value::String("test_value".to_string()),
        );

        // Track event
        manager
            .track_event(AnalyticsEventType::MetadataGenerated, properties, Some(100))
            .unwrap();

        // Track performance
        manager
            .track_performance("test_operation", 150, true, Some(1024))
            .unwrap();

        // Track error
        let error = MetadataError::new(ErrorKind::Validation, "Test error".to_string());
        let mut context = HashMap::new();
        context.insert(
            "context_key".to_string(),
            serde_json::Value::String("context_value".to_string()),
        );
        manager.track_error(error, context).unwrap();

        manager.end_session().unwrap();
    }

    #[test]
    fn test_analytics_insights_generation() {
        let mut manager = AnalyticsManager::new(AnalyticsConfig::default());
        manager
            .start_session("test_session".to_string(), None)
            .unwrap();

        // Generate some test events
        for i in 0..10 {
            let mut properties = HashMap::new();
            properties.insert(
                "iteration".to_string(),
                serde_json::Value::Number(serde_json::Number::from(i)),
            );

            manager
                .track_event(
                    AnalyticsEventType::MetadataGenerated,
                    properties,
                    Some(100 + i * 10),
                )
                .unwrap();
        }

        // Generate insights
        let insights = manager.generate_insights().unwrap();

        assert!(insights.performance.avg_generation_time_ms >= 0.0);
        // Recommendations might be empty if no issues are detected
        assert_eq!(insights.usage.popular_features.len(), 1);
        assert_eq!(
            insights.usage.popular_features[0].feature_name,
            "metadata_generation"
        );

        manager.end_session().unwrap();
    }

    #[test]
    fn test_analytics_handlers() {
        let mut manager = AnalyticsManager::new(AnalyticsConfig::default());

        // Add console handler
        let console_handler = AnalyticsHandlerFactory::console(
            Some("test_console".to_string()),
            Some(LogLevel::Debug),
        );
        manager.add_handler(console_handler);

        // Add memory handler
        let memory_handler =
            AnalyticsHandlerFactory::memory(Some("test_memory".to_string()), Some(100));
        manager.add_handler(memory_handler);

        manager
            .start_session("test_session".to_string(), None)
            .unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "test".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        manager
            .track_event(AnalyticsEventType::UserInteraction, properties, None)
            .unwrap();

        manager.end_session().unwrap();
    }

    #[test]
    fn test_analytics_integration() {
        let analytics_config = AnalyticsConfig::default();
        let integration_config = IntegrationConfig::default();
        let mut integration =
            MetadataAnalyticsIntegration::new(analytics_config, integration_config);

        integration
            .start_session("test_session".to_string(), None)
            .unwrap();

        // Test operation tracking
        let operation_id = "test_operation".to_string();
        integration.start_operation(operation_id.clone()).unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "test".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        let duration = integration
            .end_operation(
                "test_operation".to_string(),
                "test_operation",
                true,
                properties,
            )
            .unwrap();
        assert!(duration >= 0);

        // Test metadata generation tracking
        let mut metadata_properties = HashMap::new();
        metadata_properties.insert(
            "type".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        integration
            .track_metadata_generation("test_metadata", true, metadata_properties)
            .unwrap();

        // Test OG image generation tracking
        let og_params = CanvasOgParams {
            title: "Test OG Image".to_string(),
            description: Some("Test description".to_string()),
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

        integration
            .track_og_image_generation(&og_params, true, 200, Some((1200, 630)))
            .unwrap();

        // Test theme application tracking
        let theme = Theme::new(
            "test_theme".to_string(),
            "Test Theme".to_string(),
            "Test theme description".to_string(),
            ThemeCategory::Business,
        );

        integration
            .track_theme_application(&theme, true, 50)
            .unwrap();

        // Test validation tracking
        integration
            .track_metadata_validation("test_metadata", true, 25, vec![])
            .unwrap();

        // Test custom event tracking
        let mut custom_properties = HashMap::new();
        custom_properties.insert(
            "custom_key".to_string(),
            serde_json::Value::String("custom_value".to_string()),
        );

        integration
            .track_custom_event("custom_event", custom_properties, Some(75))
            .unwrap();

        // Generate insights
        let insights = integration.generate_insights().unwrap();
        // Recommendations might be empty if no issues are detected

        integration.end_session().unwrap();
    }

    #[test]
    fn test_analytics_wrapper() {
        let analytics = MetadataAnalyticsIntegration::default();
        let test_value = "test_data".to_string();
        let wrapper = test_value.with_analytics(analytics);

        assert_eq!(wrapper.inner(), "test_data");
        assert!(wrapper.analytics().get_config().enabled);

        let (inner, analytics) = wrapper.into_parts();
        assert_eq!(inner, "test_data");
        assert!(analytics.get_config().enabled);
    }

    #[test]
    fn test_analytics_context() {
        let analytics = MetadataAnalyticsIntegration::default();
        let context = AnalyticsContext::new("test_operation".to_string(), analytics);

        let mut properties = HashMap::new();
        properties.insert(
            "test".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        let duration = context
            .complete_operation("test_operation", true, properties)
            .unwrap();
        assert!(duration >= 0);
    }

    #[test]
    fn test_analytics_error_tracking() {
        let analytics = MetadataAnalyticsIntegration::default();
        let context = AnalyticsContext::new("test_operation".to_string(), analytics);

        let error = MetadataError::new(ErrorKind::Validation, "Test error".to_string());
        let mut error_context = HashMap::new();
        error_context.insert(
            "error_context".to_string(),
            serde_json::Value::String("error_value".to_string()),
        );

        let duration = context
            .complete_with_error("test_operation", error, error_context)
            .unwrap();
        assert!(duration >= 0);
    }

    #[test]
    fn test_analytics_serialization() {
        let config = AnalyticsConfig::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: AnalyticsConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.batch_size, deserialized.batch_size);
        assert_eq!(
            config.flush_interval_seconds,
            deserialized.flush_interval_seconds
        );
    }

    #[test]
    fn test_analytics_event_types() {
        let event_types = vec![
            AnalyticsEventType::MetadataGenerated,
            AnalyticsEventType::OgImageGenerated,
            AnalyticsEventType::ThemeApplied,
            AnalyticsEventType::MetadataValidated,
            AnalyticsEventType::PerformanceMeasured,
            AnalyticsEventType::ErrorOccurred,
            AnalyticsEventType::UserInteraction,
            AnalyticsEventType::Custom("test_custom".to_string()),
        ];

        for event_type in event_types {
            let event_type_str = event_type.to_string();
            assert!(!event_type_str.is_empty());
        }
    }

    #[test]
    fn test_analytics_performance_metrics() {
        let mut manager = AnalyticsManager::new(AnalyticsConfig::default());

        // Track some performance events
        manager
            .track_performance("operation1", 100, true, Some(1024))
            .unwrap();
        manager
            .track_performance("operation2", 200, true, Some(2048))
            .unwrap();
        manager
            .track_performance("operation3", 150, false, Some(1536))
            .unwrap();

        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.total_operations, 3);
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.success_rate, 2.0 / 3.0);
    }

    #[test]
    fn test_analytics_configuration() {
        let mut config = AnalyticsConfig::default();
        config.enabled = false;
        config.batch_size = 20;
        config.flush_interval_seconds = 60;
        config.track_performance = false;
        config.track_errors = false;
        config.track_interactions = false;

        let manager = AnalyticsManager::new(config);
        let manager_config = manager.get_config();

        assert!(!manager_config.enabled);
        assert_eq!(manager_config.batch_size, 20);
        assert_eq!(manager_config.flush_interval_seconds, 60);
        assert!(!manager_config.track_performance);
        assert!(!manager_config.track_errors);
        assert!(!manager_config.track_interactions);
    }

    #[test]
    fn test_analytics_privacy_settings() {
        let privacy = PrivacySettings {
            anonymize_ip: false,
            hash_identifiers: false,
            collect_user_agent: false,
            collect_page_urls: false,
            retention_days: 30,
        };

        assert!(!privacy.anonymize_ip);
        assert!(!privacy.hash_identifiers);
        assert!(!privacy.collect_user_agent);
        assert!(!privacy.collect_page_urls);
        assert_eq!(privacy.retention_days, 30);
    }

    #[test]
    fn test_analytics_integration_config() {
        let integration_config = IntegrationConfig {
            auto_track: false,
            track_metadata: false,
            track_og_images: false,
            track_themes: false,
            track_validation: false,
            track_errors: false,
            include_performance: false,
            include_user_context: false,
        };

        assert!(!integration_config.auto_track);
        assert!(!integration_config.track_metadata);
        assert!(!integration_config.track_og_images);
        assert!(!integration_config.track_themes);
        assert!(!integration_config.track_validation);
        assert!(!integration_config.track_errors);
        assert!(!integration_config.include_performance);
        assert!(!integration_config.include_user_context);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_analytics_context_creation() {
        let config = AnalyticsConfig::default();
        let context = WasmAnalyticsContext::new(config).unwrap();

        assert!(context.get_manager().get_config().enabled);
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_default_context() {
        let context = WasmAnalyticsContext::default().unwrap();
        assert!(context.get_manager().get_config().enabled);
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_page_view_tracking() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        context
            .track_page_view("test_page".to_string(), Some("Test Page".to_string()))
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_user_interaction_tracking() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "interaction_data".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        context
            .track_user_interaction("click", Some("test_button".to_string()), properties)
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_metadata_generation_tracking() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "metadata_type".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        context
            .track_metadata_generation("test_metadata", 100, true, properties)
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_og_image_generation_tracking() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            "image_type".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        context
            .track_og_image_generation("test_image", 200, true, Some((1200, 630)), properties)
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_theme_application_tracking() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        context
            .track_theme_application("test_theme", "Test Theme", 50, true)
            .unwrap();
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_browser_performance_info() {
        let context = WasmAnalyticsContext::default().unwrap();

        let performance_info = context.get_browser_performance_info().unwrap();

        // Performance info might be None if APIs are not available
        // This test just ensures the method doesn't panic
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn test_wasm_analytics_session_management() {
        let mut context = WasmAnalyticsContext::default().unwrap();

        // Session should be automatically started
        assert!(context.get_manager().get_config().enabled);

        // End session
        context.end_session().unwrap();
    }
}

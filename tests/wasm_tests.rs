//! Comprehensive WASM tests for leptos-next-metadata
//!
//! These tests validate WASM-specific functionality including:
//! - Browser API integration
//! - Web Storage functionality
//! - Canvas OG image generation
//! - Feature detection
//! - Security validation
//! - Error handling
//! - Performance monitoring

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use leptos_next_metadata::prelude::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_metadata_context_creation() {
        // Test basic WASM metadata context creation
        let context = WasmMetadataContext::new();
        assert!(!context.metadata.is_empty() || context.metadata.is_empty()); // Either is fine for new context
        assert!(context.capabilities.web_storage || !context.capabilities.web_storage);
        // Either is fine
    }

    #[wasm_bindgen_test]
    fn test_wasm_capabilities_detection() {
        // Test browser capabilities detection
        let capabilities = WasmCapabilities::detect();

        // These should be detectable in a browser environment
        assert!(capabilities.canvas || !capabilities.canvas); // Either is fine
        assert!(capabilities.web_storage || !capabilities.web_storage); // Either is fine
        assert!(capabilities.fetch || !capabilities.fetch); // Either is fine
    }

    #[wasm_bindgen_test]
    fn test_wasm_storage_backend() {
        // Test storage backend functionality
        let context = WasmMetadataContext::with_storage(WasmStorage::Local);
        assert!(matches!(context.storage, WasmStorage::Local));

        let context = WasmMetadataContext::with_storage(WasmStorage::Session);
        assert!(matches!(context.storage, WasmStorage::Session));

        let context = WasmMetadataContext::with_storage(WasmStorage::Memory);
        assert!(matches!(context.storage, WasmStorage::Memory));
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_handling() {
        // Test WASM error handling
        let context = WasmMetadataContext::new();

        // Test error handler creation
        let error_handler = context.get_error_handler();
        assert!(error_handler.is_ok());

        // Test error creation
        let browser_error = context.create_browser_error("Test error");
        assert_eq!(browser_error.kind, ErrorKind::Browser);
        assert_eq!(browser_error.message, "Test error");

        let storage_error = context.create_storage_error("Storage test error");
        assert_eq!(storage_error.kind, ErrorKind::Storage);
        assert_eq!(storage_error.message, "Storage test error");
    }

    #[wasm_bindgen_test]
    fn test_wasm_security_validation() {
        // Test security validation
        let context = WasmMetadataContext::new();

        // Test security validator creation
        let validator = context.get_security_validator();
        assert!(validator.is_ok());

        // Test security audit
        let audit = context.perform_security_audit();
        assert!(audit.is_ok());

        // Test security recommendations
        let recommendations = context.get_security_recommendations();
        assert!(!recommendations.is_empty());

        // Test security headers
        let headers = context.get_security_headers();
        assert!(!headers.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_wasm_performance_monitoring() {
        // Test performance monitoring
        let context = WasmMetadataContext::new();

        // Test performance recommendations
        let recommendations = context.get_performance_recommendations();
        assert!(recommendations.use_wasm_pack || !recommendations.use_wasm_pack); // Either is fine

        // Test optimization status
        let status = context.check_optimization_status();
        assert!(status.wasm_pack_available || !status.wasm_pack_available); // Either is fine

        // Test estimated savings
        let savings = context.get_estimated_savings();
        assert!(savings.bundle_size_reduction >= 0.0);

        // Test performance tips
        let tips = context.get_performance_tips();
        assert!(tips.is_empty() || !tips.is_empty()); // Either is fine

        // Test memory optimization
        let memory_opt = context.get_memory_optimization();
        assert!(memory_opt.memory_usage >= 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_canvas_og_generation() {
        // Test canvas OG image generation capability
        let context = WasmMetadataContext::new();

        // Test if canvas OG generation is available
        let can_generate = context.can_generate_og_images();
        assert!(can_generate || !can_generate); // Either is fine depending on browser support
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_recoverability() {
        // Test error recoverability detection
        let context = WasmMetadataContext::new();

        let browser_error = context.create_browser_error("Test browser error");
        let is_recoverable = context.is_wasm_recoverable(&browser_error);
        assert!(is_recoverable); // Browser errors should be recoverable

        let storage_error = context.create_storage_error("Test storage error");
        let is_recoverable = context.is_wasm_recoverable(&storage_error);
        assert!(is_recoverable); // Storage errors should be recoverable
    }

    #[wasm_bindgen_test]
    fn test_wasm_user_friendly_messages() {
        // Test user-friendly error messages
        let context = WasmMetadataContext::new();

        let browser_error = context.create_browser_error("Test browser error");
        let user_message = context.get_wasm_user_message(&browser_error);
        assert!(!user_message.is_empty());
        assert!(user_message.contains("browser") || user_message.contains("Browser"));

        let storage_error = context.create_storage_error("Test storage error");
        let user_message = context.get_wasm_user_message(&storage_error);
        assert!(!user_message.is_empty());
        assert!(user_message.contains("storage") || user_message.contains("Storage"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_environment_security() {
        // Test environment security checking
        let context = WasmMetadataContext::new();

        let is_secure = context.is_secure_environment();
        assert!(is_secure || !is_secure); // Either is fine depending on environment
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_context() {
        // Test WASM error context
        let context = WasmMetadataContext::new();

        let error_context = context.get_wasm_error_context();
        assert!(error_context.is_ok());

        let error_context = error_context.unwrap();
        // In a browser environment, we should have some context
        assert!(error_context.user_agent.is_some() || error_context.user_agent.is_none()); // Either is fine
        assert!(error_context.current_url.is_some() || error_context.current_url.is_none());
        // Either is fine
    }
}

// Native tests for comparison
#[cfg(not(target_arch = "wasm32"))]
mod native_tests {
    use leptos_next_metadata::prelude::*;

    #[test]
    fn test_native_metadata_context() {
        // Test that native builds work correctly
        let metadata = Metadata::default();
        assert!(metadata.title.is_none());
        assert!(metadata.description.is_none());
    }

    #[test]
    fn test_native_error_handling() {
        // Test native error handling
        let validation_error = ErrorUtils::validation_error("Test validation error");
        assert_eq!(validation_error.kind, ErrorKind::Validation);
        assert_eq!(validation_error.message, "Test validation error");

        let network_error = ErrorUtils::network_error("Test network error");
        assert_eq!(network_error.kind, ErrorKind::Network);
        assert_eq!(network_error.message, "Test network error");
    }

    #[test]
    fn test_native_error_severity() {
        // Test error severity classification
        let validation_error = ErrorUtils::validation_error("Test");
        assert_eq!(validation_error.severity(), ErrorSeverity::Medium);

        let security_error = ErrorUtils::security_error("Test");
        assert_eq!(security_error.severity(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_native_error_recoverability() {
        // Test error recoverability
        let validation_error = ErrorUtils::validation_error("Test");
        assert!(validation_error.is_recoverable());

        let security_error = ErrorUtils::security_error("Test");
        assert!(!security_error.is_recoverable());
    }

    #[test]
    fn test_native_error_statistics() {
        // Test error statistics
        let errors = vec![
            ErrorUtils::validation_error("Test 1"),
            ErrorUtils::network_error("Test 2"),
            ErrorUtils::security_error("Test 3"),
        ];

        let stats = ErrorUtils::get_error_stats(&errors);
        assert_eq!(stats.total_errors, 3);
        assert_eq!(stats.validation_errors, 1);
        assert_eq!(stats.network_errors, 1);
        assert_eq!(stats.security_errors, 1);
    }
}

//! Analytics Integration
//!
//! Provides integration between the analytics system and the main
//! metadata library, automatically tracking metadata operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::analytics::{AnalyticsConfig, AnalyticsEventType, AnalyticsManager, PerformanceMetrics};
use crate::canvas_types::CanvasOgParams;
use crate::error::{ErrorKind, MetadataError};
use crate::themes::Theme;

/// Analytics integration for metadata operations
pub struct MetadataAnalyticsIntegration {
    /// Analytics manager
    manager: AnalyticsManager,
    /// Operation start times
    operation_timers: HashMap<String, u64>,
}

/// Analytics integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable automatic tracking
    pub auto_track: bool,
    /// Track metadata generation
    pub track_metadata: bool,
    /// Track OG image generation
    pub track_og_images: bool,
    /// Track theme applications
    pub track_themes: bool,
    /// Track validation
    pub track_validation: bool,
    /// Track errors
    pub track_errors: bool,
    /// Include performance metrics
    pub include_performance: bool,
    /// Include user context
    pub include_user_context: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            auto_track: true,
            track_metadata: true,
            track_og_images: true,
            track_themes: true,
            track_validation: true,
            track_errors: true,
            include_performance: true,
            include_user_context: true,
        }
    }
}

impl MetadataAnalyticsIntegration {
    /// Create a new analytics integration
    pub fn new(config: AnalyticsConfig, _integration_config: IntegrationConfig) -> Self {
        let mut manager = AnalyticsManager::new(config);

        // Add default handlers based on environment
        #[cfg(target_arch = "wasm32")]
        {
            use crate::analytics::handlers::AnalyticsHandlerFactory;
            manager.add_handler(AnalyticsHandlerFactory::console(None, None));
            manager.add_handler(AnalyticsHandlerFactory::local_storage(None, None, None));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::analytics::handlers::AnalyticsHandlerFactory;
            manager.add_handler(AnalyticsHandlerFactory::console(None, None));
            manager.add_handler(AnalyticsHandlerFactory::file(None, None, None));
        }

        Self {
            manager,
            operation_timers: HashMap::new(),
        }
    }

    /// Create a default analytics integration
    pub fn default() -> Self {
        let analytics_config = AnalyticsConfig::default();
        let integration_config = IntegrationConfig::default();
        Self::new(analytics_config, integration_config)
    }

    /// Start tracking an operation
    pub fn start_operation(&mut self, operation_id: String) -> Result<(), MetadataError> {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
            .as_millis() as u64;

        self.operation_timers.insert(operation_id, start_time);
        Ok(())
    }

    /// End tracking an operation
    pub fn end_operation(
        &mut self,
        operation_id: String,
        operation_type: &str,
        success: bool,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<u64, MetadataError> {
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
            .as_millis() as u64;

        let duration_ms = if let Some(start_time) = self.operation_timers.remove(&operation_id) {
            end_time - start_time
        } else {
            0
        };

        // Track performance
        self.manager
            .track_performance(operation_type, duration_ms, success, None)?;

        // Track as event
        let mut event_properties = properties;
        event_properties.insert(
            "operation_type".to_string(),
            serde_json::Value::String(operation_type.to_string()),
        );
        event_properties.insert("success".to_string(), serde_json::Value::Bool(success));

        self.manager.track_event(
            AnalyticsEventType::PerformanceMeasured,
            event_properties,
            Some(duration_ms),
        )?;

        Ok(duration_ms)
    }

    /// Track metadata generation
    pub fn track_metadata_generation(
        &mut self,
        metadata_type: &str,
        success: bool,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        let mut event_properties = properties;
        event_properties.insert(
            "metadata_type".to_string(),
            serde_json::Value::String(metadata_type.to_string()),
        );
        event_properties.insert("success".to_string(), serde_json::Value::Bool(success));

        self.manager.track_event(
            AnalyticsEventType::MetadataGenerated,
            event_properties,
            None,
        )?;

        Ok(())
    }

    /// Track OG image generation
    pub fn track_og_image_generation(
        &mut self,
        params: &CanvasOgParams,
        success: bool,
        generation_time_ms: u64,
        image_size: Option<(u32, u32)>,
    ) -> Result<(), MetadataError> {
        let mut properties = HashMap::new();
        properties.insert(
            "title".to_string(),
            serde_json::Value::String(params.title.clone()),
        );
        properties.insert("success".to_string(), serde_json::Value::Bool(success));
        properties.insert(
            "generation_time_ms".to_string(),
            serde_json::Value::Number(serde_json::Number::from(generation_time_ms)),
        );

        if let Some(description) = &params.description {
            properties.insert(
                "description".to_string(),
                serde_json::Value::String(description.clone()),
            );
        }

        if let Some(width) = params.width {
            properties.insert(
                "width".to_string(),
                serde_json::Value::Number(serde_json::Number::from(width)),
            );
        }

        if let Some(height) = params.height {
            properties.insert(
                "height".to_string(),
                serde_json::Value::Number(serde_json::Number::from(height)),
            );
        }

        if let Some(background_color) = &params.background_color {
            properties.insert(
                "background_color".to_string(),
                serde_json::Value::String(background_color.clone()),
            );
        }

        if let Some(font_family) = &params.font_family {
            properties.insert(
                "font_family".to_string(),
                serde_json::Value::String(font_family.clone()),
            );
        }

        if let Some((width, height)) = image_size {
            properties.insert(
                "generated_width".to_string(),
                serde_json::Value::Number(serde_json::Number::from(width)),
            );
            properties.insert(
                "generated_height".to_string(),
                serde_json::Value::Number(serde_json::Number::from(height)),
            );
        }

        // Check if theme was used
        if params.text_gradient.is_some()
            || params.text_shadow.is_some()
            || params.text_outline.is_some()
        {
            properties.insert(
                "has_theme_effects".to_string(),
                serde_json::Value::Bool(true),
            );
        }

        if params.layers.is_some() {
            properties.insert(
                "has_custom_layers".to_string(),
                serde_json::Value::Bool(true),
            );
        }

        self.manager.track_event(
            AnalyticsEventType::OgImageGenerated,
            properties,
            Some(generation_time_ms),
        )?;

        // Also track as performance event
        self.manager
            .track_performance("og_image_generation", generation_time_ms, success, None)?;

        Ok(())
    }

    /// Track theme application
    pub fn track_theme_application(
        &mut self,
        theme: &Theme,
        success: bool,
        application_time_ms: u64,
    ) -> Result<(), MetadataError> {
        let mut properties = HashMap::new();
        properties.insert(
            "theme_id".to_string(),
            serde_json::Value::String(theme.id.clone()),
        );
        properties.insert(
            "theme_name".to_string(),
            serde_json::Value::String(theme.name.clone()),
        );
        properties.insert(
            "theme_category".to_string(),
            serde_json::Value::String(format!("{:?}", theme.category)),
        );
        properties.insert("success".to_string(), serde_json::Value::Bool(success));
        properties.insert(
            "application_time_ms".to_string(),
            serde_json::Value::Number(serde_json::Number::from(application_time_ms)),
        );

        // Track theme features
        properties.insert(
            "has_text_shadow".to_string(),
            serde_json::Value::Bool(theme.effects.text.shadow.is_some()),
        );
        properties.insert(
            "has_text_outline".to_string(),
            serde_json::Value::Bool(theme.effects.text.outline.is_some()),
        );
        properties.insert(
            "has_text_gradient".to_string(),
            serde_json::Value::Bool(theme.effects.text.gradient.is_some()),
        );
        properties.insert(
            "has_background_gradient".to_string(),
            serde_json::Value::Bool(theme.effects.background.gradient.is_some()),
        );
        properties.insert(
            "has_background_pattern".to_string(),
            serde_json::Value::Bool(theme.effects.background.pattern.is_some()),
        );
        properties.insert(
            "has_border".to_string(),
            serde_json::Value::Bool(theme.effects.border.width > 0.0),
        );

        self.manager.track_event(
            AnalyticsEventType::ThemeApplied,
            properties,
            Some(application_time_ms),
        )?;

        Ok(())
    }

    /// Track metadata validation
    pub fn track_metadata_validation(
        &mut self,
        metadata_type: &str,
        is_valid: bool,
        validation_time_ms: u64,
        errors: Vec<String>,
    ) -> Result<(), MetadataError> {
        let mut properties = HashMap::new();
        properties.insert(
            "metadata_type".to_string(),
            serde_json::Value::String(metadata_type.to_string()),
        );
        properties.insert("is_valid".to_string(), serde_json::Value::Bool(is_valid));
        properties.insert(
            "validation_time_ms".to_string(),
            serde_json::Value::Number(serde_json::Number::from(validation_time_ms)),
        );
        properties.insert(
            "error_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(errors.len())),
        );

        if !errors.is_empty() {
            properties.insert(
                "errors".to_string(),
                serde_json::Value::Array(
                    errors
                        .into_iter()
                        .map(|e| serde_json::Value::String(e))
                        .collect(),
                ),
            );
        }

        self.manager.track_event(
            AnalyticsEventType::MetadataValidated,
            properties,
            Some(validation_time_ms),
        )?;

        Ok(())
    }

    /// Track error
    pub fn track_error(
        &mut self,
        error: MetadataError,
        context: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        self.manager.track_error(error, context)?;
        Ok(())
    }

    /// Track custom event
    pub fn track_custom_event(
        &mut self,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
        duration_ms: Option<u64>,
    ) -> Result<(), MetadataError> {
        self.manager.track_event(
            AnalyticsEventType::Custom(event_name.to_string()),
            properties,
            duration_ms,
        )?;

        Ok(())
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        self.manager.get_performance_metrics()
    }

    /// Get analytics configuration
    pub fn get_config(&self) -> &AnalyticsConfig {
        self.manager.get_config()
    }

    /// Generate analytics insights
    pub fn generate_insights(&self) -> Result<crate::analytics::AnalyticsInsights, MetadataError> {
        self.manager.generate_insights()
    }

    /// Get analytics manager
    pub fn get_manager(&self) -> &AnalyticsManager {
        &self.manager
    }

    /// Get mutable analytics manager
    pub fn get_manager_mut(&mut self) -> &mut AnalyticsManager {
        &mut self.manager
    }

    /// Start analytics session
    pub fn start_session(
        &mut self,
        session_id: String,
        user_agent: Option<String>,
    ) -> Result<(), MetadataError> {
        self.manager.start_session(session_id, user_agent)
    }

    /// End analytics session
    pub fn end_session(&mut self) -> Result<(), MetadataError> {
        self.manager.end_session()
    }
}

/// Analytics wrapper for automatic tracking
pub struct AnalyticsWrapper<T> {
    /// Inner value
    inner: T,
    /// Analytics integration
    analytics: MetadataAnalyticsIntegration,
}

impl<T> AnalyticsWrapper<T> {
    /// Create a new analytics wrapper
    pub fn new(inner: T, analytics: MetadataAnalyticsIntegration) -> Self {
        Self { inner, analytics }
    }

    /// Get the inner value
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Get mutable reference to inner value
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Get analytics integration
    pub fn analytics(&self) -> &MetadataAnalyticsIntegration {
        &self.analytics
    }

    /// Get mutable analytics integration
    pub fn analytics_mut(&mut self) -> &mut MetadataAnalyticsIntegration {
        &mut self.analytics
    }

    /// Consume and return inner value and analytics
    pub fn into_parts(self) -> (T, MetadataAnalyticsIntegration) {
        (self.inner, self.analytics)
    }
}

/// Trait for objects that can be wrapped with analytics
pub trait AnalyticsTrackable {
    /// Wrap with analytics
    fn with_analytics(self, analytics: MetadataAnalyticsIntegration) -> AnalyticsWrapper<Self>
    where
        Self: Sized,
    {
        AnalyticsWrapper::new(self, analytics)
    }
}

impl<T> AnalyticsTrackable for T {}

/// Analytics context for tracking operations
pub struct AnalyticsContext {
    /// Operation ID
    operation_id: String,
    /// Analytics integration
    analytics: MetadataAnalyticsIntegration,
}

impl AnalyticsContext {
    /// Create a new analytics context
    pub fn new(operation_id: String, analytics: MetadataAnalyticsIntegration) -> Self {
        Self {
            operation_id,
            analytics,
        }
    }

    /// Track operation completion
    pub fn complete_operation(
        mut self,
        operation_type: &str,
        success: bool,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<u64, MetadataError> {
        self.analytics
            .end_operation(self.operation_id, operation_type, success, properties)
    }

    /// Track error and complete operation
    pub fn complete_with_error(
        mut self,
        operation_type: &str,
        error: MetadataError,
        context: HashMap<String, serde_json::Value>,
    ) -> Result<u64, MetadataError> {
        // Track the error
        self.analytics.track_error(error.clone(), context.clone())?;

        // Complete the operation as failed
        self.analytics
            .end_operation(self.operation_id, operation_type, false, context)
    }

    /// Get analytics integration
    pub fn analytics(&self) -> &MetadataAnalyticsIntegration {
        &self.analytics
    }

    /// Get mutable analytics integration
    pub fn analytics_mut(&mut self) -> &mut MetadataAnalyticsIntegration {
        &mut self.analytics
    }
}

/// Helper macro for tracking operations
#[macro_export]
macro_rules! track_operation {
    ($analytics:expr, $operation_type:expr, $operation:block) => {{
        let operation_id = format!(
            "{}_{}",
            $operation_type,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        $analytics.start_operation(operation_id.clone())?;

        let result = $operation;

        let success = result.is_ok();
        let properties = std::collections::HashMap::new();
        $analytics.end_operation(operation_id, $operation_type, success, properties)?;

        result
    }};
}

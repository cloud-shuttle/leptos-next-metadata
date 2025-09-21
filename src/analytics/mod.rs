//! Analytics Integration for Metadata Usage Tracking
//!
//! Provides comprehensive analytics and insights for metadata performance,
//! usage patterns, and optimization recommendations.

pub mod handlers;
pub mod integration;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{ErrorKind, MetadataError};

/// Analytics event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticsEventType {
    /// Metadata was generated
    MetadataGenerated,
    /// OG image was created
    OgImageGenerated,
    /// Theme was applied
    ThemeApplied,
    /// Metadata was validated
    MetadataValidated,
    /// Performance measurement
    PerformanceMeasured,
    /// Error occurred
    ErrorOccurred,
    /// User interaction
    UserInteraction,
    /// Custom event
    Custom(String),
}

impl std::fmt::Display for AnalyticsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyticsEventType::MetadataGenerated => write!(f, "metadata_generated"),
            AnalyticsEventType::OgImageGenerated => write!(f, "og_image_generated"),
            AnalyticsEventType::ThemeApplied => write!(f, "theme_applied"),
            AnalyticsEventType::MetadataValidated => write!(f, "metadata_validated"),
            AnalyticsEventType::PerformanceMeasured => write!(f, "performance_measured"),
            AnalyticsEventType::ErrorOccurred => write!(f, "error_occurred"),
            AnalyticsEventType::UserInteraction => write!(f, "user_interaction"),
            AnalyticsEventType::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

/// Analytics event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Event type
    pub event_type: AnalyticsEventType,
    /// Event timestamp (Unix timestamp)
    pub timestamp: u64,
    /// Event duration in milliseconds (for performance events)
    pub duration_ms: Option<u64>,
    /// Event properties/metadata
    pub properties: HashMap<String, serde_json::Value>,
    /// User/session identifier
    pub session_id: Option<String>,
    /// Page/route identifier
    pub page_id: Option<String>,
    /// Error details (if applicable)
    pub error: Option<ErrorDetails>,
}

/// Error details for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error message
    pub message: String,
    /// Error kind
    pub kind: String,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// Context information
    pub context: HashMap<String, serde_json::Value>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Generation time in milliseconds
    pub generation_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: Option<u64>,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: Option<f64>,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Error count
    pub error_count: u32,
    /// Total operations
    pub total_operations: u32,
}

/// Analytics session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSession {
    /// Session ID
    pub session_id: String,
    /// Session start time
    pub start_time: u64,
    /// Session end time (if ended)
    pub end_time: Option<u64>,
    /// User agent
    pub user_agent: Option<String>,
    /// Page views
    pub page_views: u32,
    /// Events in this session
    pub events: Vec<AnalyticsEvent>,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Enable analytics tracking
    pub enabled: bool,
    /// Batch size for sending events
    pub batch_size: usize,
    /// Flush interval in seconds
    pub flush_interval_seconds: u64,
    /// Maximum events to store locally
    pub max_local_events: usize,
    /// Enable performance tracking
    pub track_performance: bool,
    /// Enable error tracking
    pub track_errors: bool,
    /// Enable user interaction tracking
    pub track_interactions: bool,
    /// Custom event types to track
    pub custom_event_types: Vec<String>,
    /// Privacy settings
    pub privacy: PrivacySettings,
}

/// Privacy settings for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Anonymize IP addresses
    pub anonymize_ip: bool,
    /// Hash user identifiers
    pub hash_identifiers: bool,
    /// Collect user agent
    pub collect_user_agent: bool,
    /// Collect page URLs
    pub collect_page_urls: bool,
    /// Data retention period in days
    pub retention_days: u32,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            anonymize_ip: true,
            hash_identifiers: true,
            collect_user_agent: true,
            collect_page_urls: true,
            retention_days: 90,
        }
    }
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: 10,
            flush_interval_seconds: 30,
            max_local_events: 1000,
            track_performance: true,
            track_errors: true,
            track_interactions: true,
            custom_event_types: vec![],
            privacy: PrivacySettings::default(),
        }
    }
}

/// Analytics insights and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsInsights {
    /// Performance insights
    pub performance: PerformanceInsights,
    /// Usage patterns
    pub usage: UsageInsights,
    /// Error analysis
    pub errors: ErrorInsights,
    /// Recommendations
    pub recommendations: Vec<Recommendation>,
    /// Generated at timestamp
    pub generated_at: u64,
}

/// Performance insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsights {
    /// Average generation time
    pub avg_generation_time_ms: f64,
    /// Slowest operations
    pub slowest_operations: Vec<SlowOperation>,
    /// Performance trends
    pub trends: PerformanceTrends,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<String>,
}

/// Slow operation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowOperation {
    /// Operation type
    pub operation_type: String,
    /// Average duration
    pub avg_duration_ms: f64,
    /// Occurrence count
    pub count: u32,
    /// Last occurrence
    pub last_occurrence: u64,
}

/// Performance trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// Trend direction (improving, declining, stable)
    pub direction: TrendDirection,
    /// Change percentage
    pub change_percentage: f64,
    /// Time period
    pub time_period: String,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
}

/// Usage insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInsights {
    /// Most used features
    pub popular_features: Vec<FeatureUsage>,
    /// Usage patterns by time
    pub time_patterns: TimePatterns,
    /// User engagement metrics
    pub engagement: EngagementMetrics,
    /// Feature adoption
    pub adoption: FeatureAdoption,
}

/// Feature usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUsage {
    /// Feature name
    pub feature_name: String,
    /// Usage count
    pub usage_count: u32,
    /// Usage percentage
    pub usage_percentage: f64,
    /// Last used
    pub last_used: u64,
}

/// Time-based usage patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePatterns {
    /// Peak usage hours
    pub peak_hours: Vec<u8>,
    /// Usage by day of week
    pub day_of_week: HashMap<String, u32>,
    /// Usage by month
    pub monthly: HashMap<String, u32>,
}

/// User engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    /// Average session duration
    pub avg_session_duration_ms: f64,
    /// Bounce rate
    pub bounce_rate: f64,
    /// Return user rate
    pub return_user_rate: f64,
    /// Feature depth (how many features used per session)
    pub feature_depth: f64,
}

/// Feature adoption metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureAdoption {
    /// New features adopted
    pub new_features: Vec<String>,
    /// Adoption rate
    pub adoption_rate: f64,
    /// Time to adoption (days)
    pub time_to_adoption_days: f64,
}

/// Error insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInsights {
    /// Most common errors
    pub common_errors: Vec<CommonError>,
    /// Error trends
    pub error_trends: ErrorTrends,
    /// Error resolution
    pub resolution: ErrorResolution,
}

/// Common error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonError {
    /// Error type
    pub error_type: String,
    /// Error count
    pub count: u32,
    /// Error rate
    pub rate: f64,
    /// Last occurrence
    pub last_occurrence: u64,
    /// Common context
    pub common_context: HashMap<String, serde_json::Value>,
}

/// Error trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTrends {
    /// Overall error rate trend
    pub overall_trend: TrendDirection,
    /// Error rate change
    pub rate_change: f64,
    /// New error types
    pub new_error_types: Vec<String>,
}

/// Error resolution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResolution {
    /// Auto-resolved errors
    pub auto_resolved: u32,
    /// Manual resolution rate
    pub manual_resolution_rate: f64,
    /// Average resolution time
    pub avg_resolution_time_ms: f64,
}

/// Analytics recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: Priority,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Action items
    pub action_items: Vec<String>,
    /// Expected impact
    pub expected_impact: String,
    /// Implementation effort
    pub implementation_effort: EffortLevel,
}

/// Recommendation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    Performance,
    ErrorReduction,
    FeatureUsage,
    UserExperience,
    Security,
    CostOptimization,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Implementation effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// Analytics manager for tracking and insights
pub struct AnalyticsManager {
    /// Analytics configuration
    config: AnalyticsConfig,
    /// Current session
    current_session: Option<AnalyticsSession>,
    /// Event queue
    event_queue: Vec<AnalyticsEvent>,
    /// Performance metrics
    performance_metrics: PerformanceMetrics,
    /// Event handlers
    event_handlers: Vec<Box<dyn AnalyticsEventHandler>>,
}

/// Trait for analytics event handlers
pub trait AnalyticsEventHandler: Send + Sync {
    /// Handle an analytics event
    fn handle_event(&self, event: &AnalyticsEvent) -> Result<(), MetadataError>;

    /// Handle a batch of events
    fn handle_batch(&self, events: &[AnalyticsEvent]) -> Result<(), MetadataError>;

    /// Get handler name
    fn name(&self) -> &str;
}

impl AnalyticsManager {
    /// Create a new analytics manager
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            config,
            current_session: None,
            event_queue: Vec::new(),
            performance_metrics: PerformanceMetrics {
                generation_time_ms: 0,
                memory_usage_bytes: None,
                cache_hit_rate: None,
                success_rate: 1.0,
                error_count: 0,
                total_operations: 0,
            },
            event_handlers: Vec::new(),
        }
    }

    /// Start a new analytics session
    pub fn start_session(
        &mut self,
        session_id: String,
        user_agent: Option<String>,
    ) -> Result<(), MetadataError> {
        if !self.config.enabled {
            return Ok(());
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
            .as_secs();

        self.current_session = Some(AnalyticsSession {
            session_id,
            start_time: current_time,
            end_time: None,
            user_agent,
            page_views: 0,
            events: Vec::new(),
            performance: self.performance_metrics.clone(),
        });

        Ok(())
    }

    /// End the current session
    pub fn end_session(&mut self) -> Result<(), MetadataError> {
        if !self.config.enabled {
            return Ok(());
        }

        if let Some(session) = &mut self.current_session {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
                .as_secs();

            session.end_time = Some(current_time);

            // Flush remaining events
            self.flush_events()?;
        }

        self.current_session = None;
        Ok(())
    }

    /// Track an analytics event
    pub fn track_event(
        &mut self,
        event_type: AnalyticsEventType,
        properties: HashMap<String, serde_json::Value>,
        duration_ms: Option<u64>,
    ) -> Result<(), MetadataError> {
        if !self.config.enabled {
            return Ok(());
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
            .as_secs();

        let event = AnalyticsEvent {
            event_type,
            timestamp: current_time,
            duration_ms,
            properties,
            session_id: self.current_session.as_ref().map(|s| s.session_id.clone()),
            page_id: None,
            error: None,
        };

        self.add_event(event)?;
        Ok(())
    }

    /// Track a performance event
    pub fn track_performance(
        &mut self,
        operation_type: &str,
        duration_ms: u64,
        success: bool,
        memory_usage_bytes: Option<u64>,
    ) -> Result<(), MetadataError> {
        if !self.config.enabled || !self.config.track_performance {
            return Ok(());
        }

        let mut properties = HashMap::new();
        properties.insert(
            "operation_type".to_string(),
            serde_json::Value::String(operation_type.to_string()),
        );
        properties.insert("success".to_string(), serde_json::Value::Bool(success));

        if let Some(memory) = memory_usage_bytes {
            properties.insert(
                "memory_usage_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(memory)),
            );
        }

        // Update performance metrics
        self.performance_metrics.total_operations += 1;
        if !success {
            self.performance_metrics.error_count += 1;
        }

        self.performance_metrics.success_rate = (self.performance_metrics.total_operations
            - self.performance_metrics.error_count)
            as f64
            / self.performance_metrics.total_operations as f64;

        self.track_event(
            AnalyticsEventType::PerformanceMeasured,
            properties,
            Some(duration_ms),
        )?;
        Ok(())
    }

    /// Track an error event
    pub fn track_error(
        &mut self,
        error: MetadataError,
        context: HashMap<String, serde_json::Value>,
    ) -> Result<(), MetadataError> {
        if !self.config.enabled || !self.config.track_errors {
            return Ok(());
        }

        let error_details = ErrorDetails {
            message: error.message.clone(),
            kind: format!("{:?}", error.kind),
            stack_trace: None, // Would be populated in a real implementation
            context,
        };

        let mut properties = HashMap::new();
        properties.insert(
            "error_message".to_string(),
            serde_json::Value::String(error.message),
        );
        properties.insert(
            "error_kind".to_string(),
            serde_json::Value::String(format!("{:?}", error.kind)),
        );

        let event = AnalyticsEvent {
            event_type: AnalyticsEventType::ErrorOccurred,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
                .as_secs(),
            duration_ms: None,
            properties,
            session_id: self.current_session.as_ref().map(|s| s.session_id.clone()),
            page_id: None,
            error: Some(error_details),
        };

        self.add_event(event)?;
        Ok(())
    }

    /// Add an event to the queue
    fn add_event(&mut self, event: AnalyticsEvent) -> Result<(), MetadataError> {
        self.event_queue.push(event);

        // Add to current session if available
        if let Some(session) = &mut self.current_session {
            session
                .events
                .push(self.event_queue.last().unwrap().clone());
        }

        // Flush if batch size reached
        if self.event_queue.len() >= self.config.batch_size {
            self.flush_events()?;
        }

        Ok(())
    }

    /// Flush events to handlers
    fn flush_events(&mut self) -> Result<(), MetadataError> {
        if self.event_queue.is_empty() {
            return Ok(());
        }

        let events = self.event_queue.clone();
        self.event_queue.clear();

        // Send to all handlers
        for handler in &self.event_handlers {
            if let Err(e) = handler.handle_batch(&events) {
                eprintln!("Analytics handler {} failed: {}", handler.name(), e.message);
            }
        }

        Ok(())
    }

    /// Add an event handler
    pub fn add_handler(&mut self, handler: Box<dyn AnalyticsEventHandler>) {
        self.event_handlers.push(handler);
    }

    /// Generate analytics insights
    pub fn generate_insights(&self) -> Result<AnalyticsInsights, MetadataError> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataError::new(ErrorKind::Unknown, e.to_string()))?
            .as_secs();

        // Collect all events from current session
        let mut all_events = Vec::new();
        if let Some(session) = &self.current_session {
            all_events.extend(session.events.clone());
        }
        all_events.extend(self.event_queue.clone());

        let performance = self.analyze_performance(&all_events);
        let usage = self.analyze_usage(&all_events);
        let errors = self.analyze_errors(&all_events);
        let recommendations = self.generate_recommendations(&performance, &usage, &errors);

        Ok(AnalyticsInsights {
            performance,
            usage,
            errors,
            recommendations,
            generated_at: current_time,
        })
    }

    /// Analyze performance metrics
    fn analyze_performance(&self, events: &[AnalyticsEvent]) -> PerformanceInsights {
        let performance_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.event_type, AnalyticsEventType::PerformanceMeasured))
            .collect();

        let avg_generation_time = if !performance_events.is_empty() {
            let total_time: u64 = performance_events
                .iter()
                .filter_map(|e| e.duration_ms)
                .sum();
            total_time as f64 / performance_events.len() as f64
        } else {
            0.0
        };

        let slowest_operations = self.identify_slow_operations(&performance_events);
        let trends = self.calculate_performance_trends(&performance_events);
        let optimization_opportunities =
            self.identify_optimization_opportunities(&performance_events);

        PerformanceInsights {
            avg_generation_time_ms: avg_generation_time,
            slowest_operations,
            trends,
            optimization_opportunities,
        }
    }

    /// Analyze usage patterns
    fn analyze_usage(&self, events: &[AnalyticsEvent]) -> UsageInsights {
        let popular_features = self.identify_popular_features(events);
        let time_patterns = self.analyze_time_patterns(events);
        let engagement = self.calculate_engagement_metrics(events);
        let adoption = self.calculate_feature_adoption(events);

        UsageInsights {
            popular_features,
            time_patterns,
            engagement,
            adoption,
        }
    }

    /// Analyze error patterns
    fn analyze_errors(&self, events: &[AnalyticsEvent]) -> ErrorInsights {
        let error_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.event_type, AnalyticsEventType::ErrorOccurred))
            .collect();

        let common_errors = self.identify_common_errors(&error_events);
        let error_trends = self.calculate_error_trends(&error_events);
        let resolution = self.calculate_error_resolution(&error_events);

        ErrorInsights {
            common_errors,
            error_trends,
            resolution,
        }
    }

    /// Identify slow operations
    fn identify_slow_operations(&self, events: &[&AnalyticsEvent]) -> Vec<SlowOperation> {
        let mut operation_times: HashMap<String, Vec<u64>> = HashMap::new();

        for event in events {
            if let Some(duration) = event.duration_ms {
                if let Some(operation_type) = event.properties.get("operation_type") {
                    if let Some(op_type) = operation_type.as_str() {
                        operation_times
                            .entry(op_type.to_string())
                            .or_default()
                            .push(duration);
                    }
                }
            }
        }

        let mut slow_operations = Vec::new();
        for (operation_type, times) in operation_times {
            let avg_duration = times.iter().sum::<u64>() as f64 / times.len() as f64;
            let last_occurrence = events
                .iter()
                .filter(|e| {
                    e.properties.get("operation_type").and_then(|v| v.as_str())
                        == Some(&operation_type)
                })
                .map(|e| e.timestamp)
                .max()
                .unwrap_or(0);

            slow_operations.push(SlowOperation {
                operation_type,
                avg_duration_ms: avg_duration,
                count: times.len() as u32,
                last_occurrence,
            });
        }

        slow_operations.sort_by(|a, b| b.avg_duration_ms.partial_cmp(&a.avg_duration_ms).unwrap());
        slow_operations.truncate(5); // Top 5 slowest operations
        slow_operations
    }

    /// Calculate performance trends
    fn calculate_performance_trends(&self, events: &[&AnalyticsEvent]) -> PerformanceTrends {
        if events.len() < 2 {
            return PerformanceTrends {
                direction: TrendDirection::Stable,
                change_percentage: 0.0,
                time_period: "insufficient_data".to_string(),
            };
        }

        let mut sorted_events = events.to_vec();
        sorted_events.sort_by_key(|e| e.timestamp);

        let first_half = &sorted_events[..sorted_events.len() / 2];
        let second_half = &sorted_events[sorted_events.len() / 2..];

        let first_avg = first_half.iter().filter_map(|e| e.duration_ms).sum::<u64>() as f64
            / first_half.len() as f64;
        let second_avg = second_half
            .iter()
            .filter_map(|e| e.duration_ms)
            .sum::<u64>() as f64
            / second_half.len() as f64;

        let change_percentage = if first_avg > 0.0 {
            ((second_avg - first_avg) / first_avg) * 100.0
        } else {
            0.0
        };

        let direction = if change_percentage > 5.0 {
            TrendDirection::Declining
        } else if change_percentage < -5.0 {
            TrendDirection::Improving
        } else {
            TrendDirection::Stable
        };

        PerformanceTrends {
            direction,
            change_percentage,
            time_period: "recent".to_string(),
        }
    }

    /// Identify optimization opportunities
    fn identify_optimization_opportunities(&self, events: &[&AnalyticsEvent]) -> Vec<String> {
        let mut opportunities = Vec::new();

        // Check for slow operations
        let slow_operations = self.identify_slow_operations(events);
        for op in slow_operations {
            if op.avg_duration_ms > 1000.0 {
                opportunities.push(format!(
                    "Optimize {} - currently taking {:.0}ms on average",
                    op.operation_type, op.avg_duration_ms
                ));
            }
        }

        // Check for high error rates
        let total_events = events.len();
        let error_events = events
            .iter()
            .filter(|e| e.properties.get("success").and_then(|v| v.as_bool()) == Some(false))
            .count();

        if total_events > 0 {
            let error_rate = error_events as f64 / total_events as f64;
            if error_rate > 0.1 {
                opportunities.push(format!(
                    "High error rate detected: {:.1}% - investigate error causes",
                    error_rate * 100.0
                ));
            }
        }

        opportunities
    }

    /// Identify popular features
    fn identify_popular_features(&self, events: &[AnalyticsEvent]) -> Vec<FeatureUsage> {
        let mut feature_counts: HashMap<String, u32> = HashMap::new();
        let mut feature_last_used: HashMap<String, u64> = HashMap::new();

        for event in events {
            let feature_name = match &event.event_type {
                AnalyticsEventType::MetadataGenerated => "metadata_generation",
                AnalyticsEventType::OgImageGenerated => "og_image_generation",
                AnalyticsEventType::ThemeApplied => "theme_application",
                AnalyticsEventType::MetadataValidated => "metadata_validation",
                AnalyticsEventType::PerformanceMeasured => "performance_tracking",
                AnalyticsEventType::ErrorOccurred => "error_handling",
                AnalyticsEventType::UserInteraction => "user_interaction",
                AnalyticsEventType::Custom(name) => name,
            };

            *feature_counts.entry(feature_name.to_string()).or_insert(0) += 1;
            feature_last_used.insert(feature_name.to_string(), event.timestamp);
        }

        let total_events = events.len() as f64;
        let mut features: Vec<FeatureUsage> = feature_counts
            .into_iter()
            .map(|(name, count)| FeatureUsage {
                usage_percentage: (count as f64 / total_events) * 100.0,
                last_used: feature_last_used.get(&name).copied().unwrap_or(0),
                feature_name: name,
                usage_count: count,
            })
            .collect();

        features.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        features.truncate(10); // Top 10 features
        features
    }

    /// Analyze time patterns
    fn analyze_time_patterns(&self, events: &[AnalyticsEvent]) -> TimePatterns {
        let mut hourly_counts = [0u32; 24];
        let mut day_counts: HashMap<String, u32> = HashMap::new();
        let mut monthly_counts: HashMap<String, u32> = HashMap::new();

        for event in events {
            // Extract hour from timestamp (simplified)
            let hour = (event.timestamp / 3600) % 24;
            hourly_counts[hour as usize] += 1;

            // Extract day of week (simplified)
            let day_of_week = (event.timestamp / 86400) % 7;
            let day_name = match day_of_week {
                0 => "Sunday",
                1 => "Monday",
                2 => "Tuesday",
                3 => "Wednesday",
                4 => "Thursday",
                5 => "Friday",
                6 => "Saturday",
                _ => "Unknown",
            };
            *day_counts.entry(day_name.to_string()).or_insert(0) += 1;

            // Extract month (simplified)
            let month = (event.timestamp / 2629746) % 12; // Approximate seconds in a month
            let month_name = format!("Month_{}", month + 1);
            *monthly_counts.entry(month_name).or_insert(0) += 1;
        }

        // Find peak hours
        let mut peak_hours = Vec::new();
        let max_hourly_count = hourly_counts.iter().max().copied().unwrap_or(0);
        for (hour, &count) in hourly_counts.iter().enumerate() {
            if count >= max_hourly_count * 3 / 4 {
                peak_hours.push(hour as u8);
            }
        }

        TimePatterns {
            peak_hours,
            day_of_week: day_counts,
            monthly: monthly_counts,
        }
    }

    /// Calculate engagement metrics
    fn calculate_engagement_metrics(&self, events: &[AnalyticsEvent]) -> EngagementMetrics {
        let session_duration = if let Some(session) = &self.current_session {
            if let Some(end_time) = session.end_time {
                end_time - session.start_time
            } else {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    - session.start_time
            }
        } else {
            0
        };

        let unique_features = events
            .iter()
            .map(|e| format!("{:?}", e.event_type))
            .collect::<std::collections::HashSet<_>>()
            .len();

        EngagementMetrics {
            avg_session_duration_ms: session_duration as f64 * 1000.0,
            bounce_rate: if events.len() < 2 { 1.0 } else { 0.0 },
            return_user_rate: 0.0, // Would need session history to calculate
            feature_depth: unique_features as f64,
        }
    }

    /// Calculate feature adoption
    fn calculate_feature_adoption(&self, events: &[AnalyticsEvent]) -> FeatureAdoption {
        let new_features = events
            .iter()
            .filter_map(|e| match &e.event_type {
                AnalyticsEventType::Custom(name) => Some(name.clone()),
                _ => None,
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        FeatureAdoption {
            new_features,
            adoption_rate: 0.8,         // Placeholder
            time_to_adoption_days: 7.0, // Placeholder
        }
    }

    /// Identify common errors
    fn identify_common_errors(&self, events: &[&AnalyticsEvent]) -> Vec<CommonError> {
        let mut error_counts: HashMap<String, u32> = HashMap::new();
        let mut error_last_occurrence: HashMap<String, u64> = HashMap::new();
        let mut error_contexts: HashMap<String, HashMap<String, serde_json::Value>> =
            HashMap::new();

        for event in events {
            if let Some(error) = &event.error {
                let error_key = format!("{}:{}", error.kind, error.message);
                *error_counts.entry(error_key.clone()).or_insert(0) += 1;
                error_last_occurrence.insert(error_key.clone(), event.timestamp);
                error_contexts.insert(error_key.clone(), error.context.clone());
            }
        }

        let total_errors = events.len() as f64;
        let mut common_errors: Vec<CommonError> = error_counts
            .into_iter()
            .map(|(error_key, count)| {
                let parts: Vec<&str> = error_key.splitn(2, ':').collect();
                let error_type = parts.get(0).unwrap_or(&"Unknown").to_string();
                let _error_message = parts.get(1).unwrap_or(&"Unknown").to_string();

                CommonError {
                    error_type,
                    count,
                    rate: (count as f64 / total_errors) * 100.0,
                    last_occurrence: error_last_occurrence.get(&error_key).copied().unwrap_or(0),
                    common_context: error_contexts.get(&error_key).cloned().unwrap_or_default(),
                }
            })
            .collect();

        common_errors.sort_by(|a, b| b.count.cmp(&a.count));
        common_errors.truncate(5); // Top 5 common errors
        common_errors
    }

    /// Calculate error trends
    fn calculate_error_trends(&self, events: &[&AnalyticsEvent]) -> ErrorTrends {
        if events.len() < 2 {
            return ErrorTrends {
                overall_trend: TrendDirection::Stable,
                rate_change: 0.0,
                new_error_types: vec![],
            };
        }

        let mut sorted_events = events.to_vec();
        sorted_events.sort_by_key(|e| e.timestamp);

        let first_half = &sorted_events[..sorted_events.len() / 2];
        let second_half = &sorted_events[sorted_events.len() / 2..];

        let first_rate = first_half.len() as f64;
        let second_rate = second_half.len() as f64;

        let rate_change = if first_rate > 0.0 {
            ((second_rate - first_rate) / first_rate) * 100.0
        } else {
            0.0
        };

        let overall_trend = if rate_change > 10.0 {
            TrendDirection::Declining
        } else if rate_change < -10.0 {
            TrendDirection::Improving
        } else {
            TrendDirection::Stable
        };

        // Identify new error types (simplified)
        let mut new_error_types = Vec::new();
        let first_half_types: std::collections::HashSet<String> = first_half
            .iter()
            .filter_map(|e| e.error.as_ref().map(|err| err.kind.clone()))
            .collect();

        for event in second_half {
            if let Some(error) = &event.error {
                if !first_half_types.contains(&error.kind) {
                    new_error_types.push(error.kind.clone());
                }
            }
        }

        ErrorTrends {
            overall_trend,
            rate_change,
            new_error_types,
        }
    }

    /// Calculate error resolution metrics
    fn calculate_error_resolution(&self, events: &[&AnalyticsEvent]) -> ErrorResolution {
        let total_errors = events.len() as u32;
        let auto_resolved = events
            .iter()
            .filter(|e| e.properties.get("auto_resolved").and_then(|v| v.as_bool()) == Some(true))
            .count() as u32;

        ErrorResolution {
            auto_resolved,
            manual_resolution_rate: if total_errors > 0 {
                (total_errors - auto_resolved) as f64 / total_errors as f64
            } else {
                0.0
            },
            avg_resolution_time_ms: 1000.0, // Placeholder
        }
    }

    /// Generate recommendations
    fn generate_recommendations(
        &self,
        performance: &PerformanceInsights,
        usage: &UsageInsights,
        errors: &ErrorInsights,
    ) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // Performance recommendations
        if performance.avg_generation_time_ms > 500.0 {
            recommendations.push(Recommendation {
                recommendation_type: RecommendationType::Performance,
                priority: Priority::High,
                title: "Optimize Generation Performance".to_string(),
                description: "Metadata generation is taking longer than expected".to_string(),
                action_items: vec![
                    "Implement caching for frequently generated metadata".to_string(),
                    "Optimize image processing algorithms".to_string(),
                    "Consider using Web Workers for heavy operations".to_string(),
                ],
                expected_impact: "Reduce generation time by 30-50%".to_string(),
                implementation_effort: EffortLevel::Medium,
            });
        }

        // Error reduction recommendations
        if errors.common_errors.iter().any(|e| e.rate > 5.0) {
            recommendations.push(Recommendation {
                recommendation_type: RecommendationType::ErrorReduction,
                priority: Priority::High,
                title: "Address Common Errors".to_string(),
                description: "High error rates detected in metadata operations".to_string(),
                action_items: vec![
                    "Add better input validation".to_string(),
                    "Improve error handling and recovery".to_string(),
                    "Add retry mechanisms for transient failures".to_string(),
                ],
                expected_impact: "Reduce error rate by 50-80%".to_string(),
                implementation_effort: EffortLevel::Medium,
            });
        }

        // Feature usage recommendations
        if usage.adoption.adoption_rate < 0.5 {
            recommendations.push(Recommendation {
                recommendation_type: RecommendationType::FeatureUsage,
                priority: Priority::Medium,
                title: "Improve Feature Adoption".to_string(),
                description: "Low feature adoption rates detected".to_string(),
                action_items: vec![
                    "Add feature discovery mechanisms".to_string(),
                    "Improve documentation and examples".to_string(),
                    "Add progressive disclosure for advanced features".to_string(),
                ],
                expected_impact: "Increase feature adoption by 25-40%".to_string(),
                implementation_effort: EffortLevel::Low,
            });
        }

        recommendations
    }

    /// Get current performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// Get analytics configuration
    pub fn get_config(&self) -> &AnalyticsConfig {
        &self.config
    }

    /// Update analytics configuration
    pub fn update_config(&mut self, config: AnalyticsConfig) {
        self.config = config;
    }
}

impl Default for AnalyticsManager {
    fn default() -> Self {
        Self::new(AnalyticsConfig::default())
    }
}

//! Performance monitoring and metrics for OG image generation
//!
//! This module provides comprehensive performance tracking to ensure
//! generation times stay within acceptable limits and identify bottlenecks.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::Result;

/// Performance metrics for OG image generation
#[derive(Debug, Clone)]
pub struct GenerationMetrics {
    /// Total number of generations
    pub total_generations: u64,
    /// Total generation time
    pub total_time: Duration,
    /// Average generation time
    pub average_time: Duration,
    /// Minimum generation time
    pub min_time: Duration,
    /// Maximum generation time
    pub max_time: Duration,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of WebP fallbacks
    pub webp_fallbacks: u64,
    /// Number of errors
    pub errors: u64,
    /// Current memory usage (estimated)
    pub memory_usage_bytes: usize,
}

impl Default for GenerationMetrics {
    fn default() -> Self {
        Self {
            total_generations: 0,
            total_time: Duration::ZERO,
            average_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            cache_hits: 0,
            cache_misses: 0,
            cache_hit_rate: 0.0,
            webp_fallbacks: 0,
            errors: 0,
            memory_usage_bytes: 0,
        }
    }
}

/// Thread-safe metrics collector
pub struct MetricsCollector {
    /// Total generations counter
    total_generations: AtomicU64,
    /// Total time accumulator
    total_time: AtomicU64, // in nanoseconds
    /// Minimum time
    min_time: AtomicU64, // in nanoseconds
    /// Maximum time
    max_time: AtomicU64, // in nanoseconds
    /// Cache hits counter
    cache_hits: AtomicU64,
    /// Cache misses counter
    cache_misses: AtomicU64,
    /// WebP fallbacks counter
    webp_fallbacks: AtomicU64,
    /// Errors counter
    errors: AtomicU64,
    /// Memory usage estimate
    memory_usage: AtomicUsize,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            total_generations: AtomicU64::new(0),
            total_time: AtomicU64::new(0),
            min_time: AtomicU64::new(u64::MAX),
            max_time: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            webp_fallbacks: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            memory_usage: AtomicUsize::new(0),
        }
    }

    /// Record a generation with timing
    pub fn record_generation(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;

        self.total_generations.fetch_add(1, Ordering::Relaxed);
        self.total_time.fetch_add(nanos, Ordering::Relaxed);

        // Update min time
        loop {
            let current_min = self.min_time.load(Ordering::Relaxed);
            if nanos >= current_min {
                break;
            }
            if self
                .min_time
                .compare_exchange_weak(current_min, nanos, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }

        // Update max time
        loop {
            let current_max = self.max_time.load(Ordering::Relaxed);
            if nanos <= current_max {
                break;
            }
            if self
                .max_time
                .compare_exchange_weak(current_max, nanos, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }

    /// Record a cache hit
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache miss
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a WebP fallback
    pub fn record_webp_fallback(&self) {
        self.webp_fallbacks.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an error
    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Update memory usage estimate
    pub fn update_memory_usage(&self, bytes: usize) {
        self.memory_usage.store(bytes, Ordering::Relaxed);
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> GenerationMetrics {
        let total_gens = self.total_generations.load(Ordering::Relaxed);
        let total_time_nanos = self.total_time.load(Ordering::Relaxed);
        let min_time_nanos = self.min_time.load(Ordering::Relaxed);
        let max_time_nanos = self.max_time.load(Ordering::Relaxed);
        let cache_hits = self.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.cache_misses.load(Ordering::Relaxed);
        let webp_fallbacks = self.webp_fallbacks.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        let memory_usage = self.memory_usage.load(Ordering::Relaxed);

        let total_time = Duration::from_nanos(total_time_nanos);
        let min_time = if min_time_nanos == u64::MAX {
            Duration::ZERO
        } else {
            Duration::from_nanos(min_time_nanos)
        };
        let max_time = Duration::from_nanos(max_time_nanos);
        let average_time = if total_gens > 0 {
            Duration::from_nanos(total_time_nanos / total_gens)
        } else {
            Duration::ZERO
        };

        let total_cache_operations = cache_hits + cache_misses;
        let cache_hit_rate = if total_cache_operations > 0 {
            cache_hits as f64 / total_cache_operations as f64
        } else {
            0.0
        };

        GenerationMetrics {
            total_generations: total_gens,
            total_time,
            average_time,
            min_time,
            max_time,
            cache_hits,
            cache_misses,
            cache_hit_rate,
            webp_fallbacks,
            errors,
            memory_usage_bytes: memory_usage,
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_generations.store(0, Ordering::Relaxed);
        self.total_time.store(0, Ordering::Relaxed);
        self.min_time.store(u64::MAX, Ordering::Relaxed);
        self.max_time.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
        self.webp_fallbacks.store(0, Ordering::Relaxed);
        self.errors.store(0, Ordering::Relaxed);
        self.memory_usage.store(0, Ordering::Relaxed);
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance limits for OG image generation
#[derive(Debug, Clone)]
pub struct PerformanceLimits {
    /// Maximum allowed generation time
    pub max_generation_time: Duration,
    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,
    /// Maximum template size in bytes
    pub max_template_size: usize,
    /// Maximum cache entries
    pub max_cache_entries: usize,
    /// Warning threshold for generation time
    pub warning_generation_time: Duration,
}

impl Default for PerformanceLimits {
    fn default() -> Self {
        Self {
            max_generation_time: Duration::from_millis(500), // 500ms
            max_memory_usage: 50 * 1024 * 1024,              // 50MB
            max_template_size: 1024 * 1024,                  // 1MB
            max_cache_entries: 1000,
            warning_generation_time: Duration::from_millis(200), // 200ms
        }
    }
}

/// Performance monitor that tracks metrics and enforces limits
pub struct PerformanceMonitor {
    metrics: Arc<MetricsCollector>,
    limits: PerformanceLimits,
}

impl PerformanceMonitor {
    pub fn new(limits: PerformanceLimits) -> Self {
        Self {
            metrics: Arc::new(MetricsCollector::new()),
            limits,
        }
    }

    pub fn with_default_limits() -> Self {
        Self::new(PerformanceLimits::default())
    }

    /// Get the metrics collector
    pub fn metrics(&self) -> Arc<MetricsCollector> {
        self.metrics.clone()
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> GenerationMetrics {
        self.metrics.get_metrics()
    }

    /// Check if generation time is within limits
    pub fn check_generation_time(&self, duration: Duration) -> Result<()> {
        if duration > self.limits.max_generation_time {
            return Err(crate::Error::PerformanceLimitExceeded(format!(
                "Generation time {}ms exceeds limit {}ms",
                duration.as_millis(),
                self.limits.max_generation_time.as_millis()
            )));
        }

        if duration > self.limits.warning_generation_time {
            eprintln!(
                "WARNING: Generation time {}ms exceeds warning threshold {}ms",
                duration.as_millis(),
                self.limits.warning_generation_time.as_millis()
            );
        }

        Ok(())
    }

    /// Check if memory usage is within limits
    pub fn check_memory_usage(&self, usage: usize) -> Result<()> {
        if usage > self.limits.max_memory_usage {
            return Err(crate::Error::PerformanceLimitExceeded(format!(
                "Memory usage {}MB exceeds limit {}MB",
                usage / (1024 * 1024),
                self.limits.max_memory_usage / (1024 * 1024)
            )));
        }

        Ok(())
    }

    /// Check if template size is within limits
    pub fn check_template_size(&self, size: usize) -> Result<()> {
        if size > self.limits.max_template_size {
            return Err(crate::Error::PerformanceLimitExceeded(format!(
                "Template size {}KB exceeds limit {}KB",
                size / 1024,
                self.limits.max_template_size / 1024
            )));
        }

        Ok(())
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> PerformanceReport {
        let metrics = self.get_metrics();

        PerformanceReport {
            health_status: self.assess_health(&metrics),
            recommendations: self.generate_recommendations(&metrics),
            metrics,
            limits: self.limits.clone(),
        }
    }

    /// Assess overall health based on metrics
    fn assess_health(&self, metrics: &GenerationMetrics) -> HealthStatus {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check average generation time
        if metrics.average_time > self.limits.max_generation_time {
            issues.push("Average generation time exceeds limit".to_string());
        } else if metrics.average_time > self.limits.warning_generation_time {
            warnings.push("Average generation time approaching limit".to_string());
        }

        // Check cache hit rate
        if metrics.cache_hit_rate < 0.5 && metrics.total_generations > 10 {
            warnings.push("Low cache hit rate".to_string());
        }

        // Check error rate
        let error_rate = if metrics.total_generations > 0 {
            metrics.errors as f64 / metrics.total_generations as f64
        } else {
            0.0
        };

        if error_rate > 0.1 {
            issues.push("High error rate".to_string());
        } else if error_rate > 0.05 {
            warnings.push("Elevated error rate".to_string());
        }

        // Check WebP fallback rate
        let fallback_rate = if metrics.total_generations > 0 {
            metrics.webp_fallbacks as f64 / metrics.total_generations as f64
        } else {
            0.0
        };

        if fallback_rate > 0.2 {
            warnings.push("High WebP fallback rate".to_string());
        }

        if issues.is_empty() && warnings.is_empty() {
            HealthStatus::Healthy
        } else if issues.is_empty() {
            HealthStatus::Warning(warnings)
        } else {
            HealthStatus::Critical(issues, warnings)
        }
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, metrics: &GenerationMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Generation time recommendations
        if metrics.average_time > Duration::from_millis(300) {
            recommendations
                .push("Consider enabling caching to reduce generation times".to_string());
        }

        if metrics.max_time > Duration::from_millis(1000) {
            recommendations.push(
                "Investigate slow generation cases - consider template optimization".to_string(),
            );
        }

        // Cache recommendations
        if metrics.cache_hit_rate < 0.3 && metrics.total_generations > 20 {
            recommendations.push("Cache hit rate is low - review cache key generation".to_string());
        }

        // Memory recommendations
        if metrics.memory_usage_bytes > 10 * 1024 * 1024 {
            recommendations
                .push("High memory usage detected - consider reducing cache size".to_string());
        }

        // WebP recommendations
        if metrics.webp_fallbacks > 0 {
            recommendations.push(
                "WebP fallbacks detected - ensure webp-support feature is enabled".to_string(),
            );
        }

        recommendations
    }
}

/// Performance report with metrics and recommendations
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub metrics: GenerationMetrics,
    pub limits: PerformanceLimits,
    pub health_status: HealthStatus,
    pub recommendations: Vec<String>,
}

/// Health status of the OG image generation system
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Warning(Vec<String>),
    Critical(Vec<String>, Vec<String>),
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, HealthStatus::Warning(_))
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, HealthStatus::Critical(_, _))
    }
}

/// Timing guard for measuring generation time
pub struct TimingGuard {
    start: Instant,
    metrics: Arc<MetricsCollector>,
}

impl TimingGuard {
    pub fn new(metrics: Arc<MetricsCollector>) -> Self {
        Self {
            start: Instant::now(),
            metrics,
        }
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_generation(duration);
    }
}

/// Helper macro for timing operations
#[macro_export]
macro_rules! time_operation {
    ($metrics:expr, $operation:expr) => {{
        let _guard = $crate::og_image::metrics::TimingGuard::new($metrics);
        $operation
    }};
}

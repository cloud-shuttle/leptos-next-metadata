//! Performance optimization utilities for WASM
//!
//! Provides tools for optimizing bundle size, runtime performance, and memory usage

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Performance;

/// Performance metrics for WASM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operation name
    pub operation: String,
    /// Start time (high resolution timestamp)
    pub start_time: f64,
    /// End time (high resolution timestamp)
    pub end_time: f64,
    /// Duration in milliseconds
    pub duration: f64,
    /// Memory usage (approximate)
    pub memory_usage: Option<usize>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Performance profiler for WASM operations
#[derive(Debug)]
pub struct WasmProfiler {
    /// Current measurements
    measurements: HashMap<String, PerformanceMetrics>,
    /// Performance API reference
    performance: Option<Performance>,
    /// Memory tracking enabled
    memory_tracking: bool,
}

/// Bundle size optimization utilities
pub struct BundleOptimizer;

/// Runtime performance utilities
pub struct RuntimeOptimizer;

impl Default for WasmProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmProfiler {
    /// Create a new WASM profiler
    pub fn new() -> Self {
        let performance = web_sys::window().and_then(|w| w.performance());

        Self {
            measurements: HashMap::new(),
            performance,
            memory_tracking: false,
        }
    }

    /// Create with memory tracking enabled
    pub fn with_memory_tracking() -> Self {
        let mut profiler = Self::new();
        profiler.memory_tracking = true;
        profiler
    }

    /// Start timing an operation
    pub fn start_timing(&mut self, operation: &str) -> Result<(), JsValue> {
        let start_time = self.get_current_time()?;

        let mut metadata = HashMap::new();
        if self.memory_tracking {
            if let Some(memory) = self.get_memory_usage() {
                metadata.insert("start_memory".to_string(), memory.to_string());
            }
        }

        let metrics = PerformanceMetrics {
            operation: operation.to_string(),
            start_time,
            end_time: 0.0,
            duration: 0.0,
            memory_usage: self.get_memory_usage(),
            metadata,
        };

        self.measurements.insert(operation.to_string(), metrics);
        Ok(())
    }

    /// End timing an operation
    pub fn end_timing(&mut self, operation: &str) -> Result<PerformanceMetrics, JsValue> {
        let end_time = self.get_current_time()?;
        let memory_usage = if self.memory_tracking {
            self.get_memory_usage()
        } else {
            None
        };

        if let Some(metrics) = self.measurements.get_mut(operation) {
            metrics.end_time = end_time;
            metrics.duration = end_time - metrics.start_time;

            if self.memory_tracking {
                if let Some(memory) = memory_usage {
                    metrics
                        .metadata
                        .insert("end_memory".to_string(), memory.to_string());
                    if let Some(start_memory) = metrics.memory_usage {
                        let memory_delta = memory as i64 - start_memory as i64;
                        metrics
                            .metadata
                            .insert("memory_delta".to_string(), memory_delta.to_string());
                    }
                }
            }

            Ok(metrics.clone())
        } else {
            Err(JsValue::from_str(&format!(
                "Operation '{}' not found",
                operation
            )))
        }
    }

    /// Get current high-resolution timestamp
    fn get_current_time(&self) -> Result<f64, JsValue> {
        if let Some(perf) = &self.performance {
            Ok(perf.now())
        } else {
            // Fallback to Date.now() if Performance API is not available
            Ok(js_sys::Date::now())
        }
    }

    /// Get approximate memory usage
    fn get_memory_usage(&self) -> Option<usize> {
        if let Some(_window) = web_sys::window() {
            // Try to get memory info from performance.memory (Chrome/Edge)
            // This is a non-standard API, so we'll use a different approach
            // For now, return None as memory tracking is not universally available
            None
        } else {
            None
        }
    }

    /// Get all measurements
    pub fn get_measurements(&self) -> &HashMap<String, PerformanceMetrics> {
        &self.measurements
    }

    /// Get measurement for specific operation
    pub fn get_measurement(&self, operation: &str) -> Option<&PerformanceMetrics> {
        self.measurements.get(operation)
    }

    /// Clear all measurements
    pub fn clear_measurements(&mut self) {
        self.measurements.clear();
    }

    /// Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        let total_operations = self.measurements.len();
        let total_duration: f64 = self.measurements.values().map(|m| m.duration).sum();

        let avg_duration = if total_operations > 0 {
            total_duration / total_operations as f64
        } else {
            0.0
        };

        let slowest_operation = self.measurements.values().max_by(|a, b| {
            a.duration
                .partial_cmp(&b.duration)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        PerformanceSummary {
            total_operations,
            total_duration,
            average_duration: avg_duration,
            slowest_operation: slowest_operation.map(|m| m.operation.clone()),
            memory_tracking_enabled: self.memory_tracking,
        }
    }
}

/// Performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Total number of operations measured
    pub total_operations: usize,
    /// Total duration of all operations (ms)
    pub total_duration: f64,
    /// Average duration per operation (ms)
    pub average_duration: f64,
    /// Slowest operation name
    pub slowest_operation: Option<String>,
    /// Whether memory tracking is enabled
    pub memory_tracking_enabled: bool,
}

impl BundleOptimizer {
    /// Get bundle size recommendations
    pub fn get_recommendations() -> BundleRecommendations {
        BundleRecommendations {
            use_wasm_pack: true,
            enable_wasm_opt: true,
            use_wasm_bindgen_optimization: true,
            enable_tree_shaking: true,
            use_compression: true,
            lazy_load_modules: true,
            code_splitting: true,
        }
    }

    /// Check if optimizations are available
    pub fn check_optimizations() -> OptimizationStatus {
        let mut status = OptimizationStatus {
            wasm_pack_available: false,
            wasm_opt_available: false,
            compression_available: false,
            tree_shaking_available: false,
            recommendations: Vec::new(),
        };

        // Check for WASM optimizations
        if Self::is_wasm_pack_available() {
            status.wasm_pack_available = true;
        } else {
            status
                .recommendations
                .push("Install wasm-pack for optimal WASM builds".to_string());
        }

        if Self::is_wasm_opt_available() {
            status.wasm_opt_available = true;
        } else {
            status
                .recommendations
                .push("Install wasm-opt for additional WASM optimizations".to_string());
        }

        // Check for compression support
        if Self::is_compression_available() {
            status.compression_available = true;
        } else {
            status
                .recommendations
                .push("Enable gzip/brotli compression for smaller bundles".to_string());
        }

        // Tree shaking is typically available in modern bundlers
        status.tree_shaking_available = true;

        status
    }

    /// Check if wasm-pack is available
    fn is_wasm_pack_available() -> bool {
        // This would typically check if wasm-pack is installed
        // For now, we'll assume it's available
        true
    }

    /// Check if wasm-opt is available
    fn is_wasm_opt_available() -> bool {
        // This would typically check if wasm-opt is installed
        // For now, we'll assume it's available
        true
    }

    /// Check if compression is available
    fn is_compression_available() -> bool {
        // Check if the server supports compression
        if let Some(_window) = web_sys::window() {
            // This is a simplified check - in reality, you'd check server headers
            true
        } else {
            false
        }
    }

    /// Get estimated bundle size savings
    pub fn get_estimated_savings() -> BundleSavings {
        BundleSavings {
            wasm_opt_savings: "15-30%".to_string(),
            compression_savings: "60-80%".to_string(),
            tree_shaking_savings: "10-40%".to_string(),
            total_estimated_savings: "70-90%".to_string(),
        }
    }
}

/// Bundle optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleRecommendations {
    /// Use wasm-pack for building
    pub use_wasm_pack: bool,
    /// Enable wasm-opt optimization
    pub enable_wasm_opt: bool,
    /// Use wasm-bindgen optimization
    pub use_wasm_bindgen_optimization: bool,
    /// Enable tree shaking
    pub enable_tree_shaking: bool,
    /// Use compression
    pub use_compression: bool,
    /// Lazy load modules
    pub lazy_load_modules: bool,
    /// Use code splitting
    pub code_splitting: bool,
}

/// Optimization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStatus {
    /// Whether wasm-pack is available
    pub wasm_pack_available: bool,
    /// Whether wasm-opt is available
    pub wasm_opt_available: bool,
    /// Whether compression is available
    pub compression_available: bool,
    /// Whether tree shaking is available
    pub tree_shaking_available: bool,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

/// Bundle size savings estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSavings {
    /// Savings from wasm-opt
    pub wasm_opt_savings: String,
    /// Savings from compression
    pub compression_savings: String,
    /// Savings from tree shaking
    pub tree_shaking_savings: String,
    /// Total estimated savings
    pub total_estimated_savings: String,
}

impl RuntimeOptimizer {
    /// Optimize memory usage
    pub fn optimize_memory() -> MemoryOptimization {
        MemoryOptimization {
            gc_recommended: Self::should_run_gc(),
            memory_pressure: Self::get_memory_pressure(),
            optimization_suggestions: Self::get_memory_suggestions(),
        }
    }

    /// Check if garbage collection should be run
    fn should_run_gc() -> bool {
        // In a real implementation, you'd check memory usage
        // For now, return false as we can't reliably detect this
        false
    }

    /// Get current memory pressure level
    fn get_memory_pressure() -> MemoryPressure {
        // This would typically check actual memory usage
        // For now, return a default value
        MemoryPressure::Low
    }

    /// Get memory optimization suggestions
    fn get_memory_suggestions() -> Vec<String> {
        vec![
            "Use WeakMap for temporary object references".to_string(),
            "Avoid creating large objects in tight loops".to_string(),
            "Use object pooling for frequently created objects".to_string(),
            "Consider using Web Workers for heavy computations".to_string(),
        ]
    }

    /// Get performance tips
    pub fn get_performance_tips() -> Vec<PerformanceTip> {
        vec![
            PerformanceTip {
                category: "Memory".to_string(),
                tip: "Use TypedArrays for large data sets".to_string(),
                impact: "High".to_string(),
            },
            PerformanceTip {
                category: "DOM".to_string(),
                tip: "Batch DOM operations to reduce reflows".to_string(),
                impact: "High".to_string(),
            },
            PerformanceTip {
                category: "Canvas".to_string(),
                tip: "Use requestAnimationFrame for smooth animations".to_string(),
                impact: "Medium".to_string(),
            },
            PerformanceTip {
                category: "Storage".to_string(),
                tip: "Use IndexedDB for large data storage".to_string(),
                impact: "Medium".to_string(),
            },
        ]
    }
}

/// Memory optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimization {
    /// Whether garbage collection is recommended
    pub gc_recommended: bool,
    /// Current memory pressure level
    pub memory_pressure: MemoryPressure,
    /// Optimization suggestions
    pub optimization_suggestions: Vec<String>,
}

/// Memory pressure levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryPressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance tip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTip {
    /// Category of the tip
    pub category: String,
    /// The tip itself
    pub tip: String,
    /// Impact level
    pub impact: String,
}

/// Performance monitoring utilities
pub struct PerformanceMonitor;

impl PerformanceMonitor {
    /// Start monitoring performance
    pub fn start_monitoring() -> Result<WasmProfiler, JsValue> {
        Ok(WasmProfiler::with_memory_tracking())
    }

    /// Get current performance metrics
    pub fn get_current_metrics() -> Result<PerformanceSummary, JsValue> {
        let profiler = WasmProfiler::new();
        Ok(profiler.get_summary())
    }

    /// Check if performance monitoring is supported
    pub fn is_supported() -> bool {
        web_sys::window().is_some() && web_sys::window().unwrap().performance().is_some()
    }

    /// Get browser performance info
    pub fn get_browser_info() -> BrowserPerformanceInfo {
        let window = web_sys::window().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        BrowserPerformanceInfo {
            performance_api_available: window.performance().is_some(),
            high_resolution_timing: Self::supports_high_resolution_timing(),
            memory_api_available: Self::supports_memory_api(),
            user_agent: window
                .navigator()
                .user_agent()
                .unwrap_or_else(|_| "Unknown".to_string()),
        }
    }

    /// Check if high resolution timing is supported
    fn supports_high_resolution_timing() -> bool {
        if let Some(window) = web_sys::window() {
            if let Some(perf) = window.performance() {
                // Check if performance.now() returns high resolution timestamps
                let start = perf.now();
                let end = perf.now();
                end - start < 1.0 // Should be very small for high resolution timing
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if memory API is supported
    fn supports_memory_api() -> bool {
        // Memory API is non-standard and only available in some browsers
        false
    }
}

/// Browser performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserPerformanceInfo {
    /// Whether Performance API is available
    pub performance_api_available: bool,
    /// Whether high resolution timing is supported
    pub high_resolution_timing: bool,
    /// Whether memory API is available
    pub memory_api_available: bool,
    /// User agent string
    pub user_agent: String,
}

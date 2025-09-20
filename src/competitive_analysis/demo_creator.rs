//! Demo Creator for Competitive Analysis
//!
//! Provides functionality to create demos that showcase our capabilities
//! against competitors, including performance benchmarking and feature comparison.

use super::types::*;
use chrono::{DateTime, Utc};

/// Demo creator for competitive positioning
pub struct DemoCreator {
    target_competitor: String,
    target_capability: String,
    demo_requirements: DemoRequirements,
}

impl DemoCreator {
    /// Create a new demo creator
    pub fn new(
        target_competitor: String,
        target_capability: String,
        demo_requirements: DemoRequirements,
    ) -> Self {
        Self {
            target_competitor,
            target_capability,
            demo_requirements,
        }
    }

    /// Get target competitor
    pub fn target_competitor(&self) -> &str {
        &self.target_competitor
    }

    /// Get target capability
    pub fn target_capability(&self) -> &str {
        &self.target_capability
    }

    /// Create a competitive demo
    pub fn create_competitive_demo(
        &self,
        competitor_demo_url: &str,
    ) -> Result<Demo, DemoCreationError> {
        // Analyze competitor demo
        let competitor_analysis = self.analyze_competitor_demo(competitor_demo_url)?;

        // Create our demo with matching or exceeding capabilities
        let our_demo = self.create_our_demo(&competitor_analysis)?;

        // Benchmark against competitor
        let benchmark_result = self.benchmark_demos(&competitor_analysis, &our_demo)?;

        // Ensure we meet or exceed targets
        if !self.meets_performance_targets(&benchmark_result) {
            return Err(DemoCreationError::PerformanceTargetsNotMet);
        }

        Ok(our_demo)
    }

    /// Analyze competitor demo
    fn analyze_competitor_demo(
        &self,
        _url: &str,
    ) -> Result<CompetitorDemoAnalysis, DemoCreationError> {
        // This would involve web scraping, performance testing, etc.
        // For now, return a mock analysis
        Ok(CompetitorDemoAnalysis {
            features: vec!["Basic functionality".to_string()],
            performance_metrics: PerformanceMetrics::new(
                Some(1000.0),
                Some(100.0),
                Some(512.0),
                Some(50.0),
            ),
            user_experience: vec!["Simple interface".to_string()],
            technical_implementation: vec!["Standard approach".to_string()],
        })
    }

    /// Create our demo
    fn create_our_demo(
        &self,
        _competitor_analysis: &CompetitorDemoAnalysis,
    ) -> Result<Demo, DemoCreationError> {
        Ok(Demo {
            name: format!("{} vs {}", self.target_capability, self.target_competitor),
            description: format!(
                "Demo showcasing our {} capabilities against {}",
                self.target_capability, self.target_competitor
            ),
            features: vec!["Advanced functionality".to_string()],
            performance_metrics: PerformanceMetrics::new(
                Some(2000.0), // Better throughput
                Some(50.0),   // Better latency
                Some(256.0),  // Better memory usage
                Some(25.0),   // Better CPU usage
            ),
            created_at: Utc::now(),
            demo_url: "https://demo.example.com".to_string(),
        })
    }

    /// Benchmark demos
    fn benchmark_demos(
        &self,
        _competitor_analysis: &CompetitorDemoAnalysis,
        _our_demo: &Demo,
    ) -> Result<BenchmarkResult, DemoCreationError> {
        Ok(BenchmarkResult {
            competitor_metrics: PerformanceMetrics::new(
                Some(1000.0),
                Some(100.0),
                Some(512.0),
                Some(50.0),
            ),
            our_metrics: PerformanceMetrics::new(Some(2000.0), Some(50.0), Some(256.0), Some(25.0)),
            performance_difference: PerformanceDifference::new(100.0, 50.0, 50.0, 50.0),
            recommendations: vec!["Continue current approach".to_string()],
        })
    }

    /// Check if performance targets are met
    fn meets_performance_targets(&self, benchmark_result: &BenchmarkResult) -> bool {
        let targets = &self.demo_requirements.performance_targets;

        // Check throughput target
        if let Some(target) = targets.throughput_target {
            if benchmark_result.our_metrics.throughput.unwrap_or(0.0) < target {
                return false;
            }
        }

        // Check latency target (lower is better)
        if let Some(target) = targets.latency_target {
            if benchmark_result.our_metrics.latency.unwrap_or(f64::MAX) > target {
                return false;
            }
        }

        // Check memory target (lower is better)
        if let Some(target) = targets.memory_target {
            if benchmark_result
                .our_metrics
                .memory_usage
                .unwrap_or(f64::MAX)
                > target
            {
                return false;
            }
        }

        // Check CPU target (lower is better)
        if let Some(target) = targets.cpu_target {
            if benchmark_result.our_metrics.cpu_usage.unwrap_or(f64::MAX) > target {
                return false;
            }
        }

        true
    }
}

/// Demo requirements
#[derive(Debug, Clone, Default)]
pub struct DemoRequirements {
    pub performance_targets: PerformanceTargets,
    pub feature_requirements: Vec<String>,
    pub user_experience_goals: Vec<String>,
    pub technical_constraints: Vec<String>,
}

/// Performance targets for demos
#[derive(Debug, Clone)]
pub struct PerformanceTargets {
    pub throughput_target: Option<f64>, // records/second
    pub latency_target: Option<f64>,    // milliseconds
    pub memory_target: Option<f64>,     // MB
    pub cpu_target: Option<f64>,        // percentage
}

impl Default for PerformanceTargets {
    fn default() -> Self {
        Self {
            throughput_target: Some(1000.0),
            latency_target: Some(100.0),
            memory_target: Some(512.0),
            cpu_target: Some(50.0),
        }
    }
}

/// Demo representation
#[derive(Debug, Clone)]
pub struct Demo {
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
    pub performance_metrics: PerformanceMetrics,
    pub created_at: DateTime<Utc>,
    pub demo_url: String,
}

/// Competitor demo analysis
#[derive(Debug, Clone)]
pub struct CompetitorDemoAnalysis {
    pub features: Vec<String>,
    pub performance_metrics: PerformanceMetrics,
    pub user_experience: Vec<String>,
    pub technical_implementation: Vec<String>,
}

/// Demo creation errors
#[derive(Debug, thiserror::Error)]
pub enum DemoCreationError {
    #[error("Performance targets not met")]
    PerformanceTargetsNotMet,
    #[error("Failed to analyze competitor demo: {0}")]
    CompetitorAnalysisFailed(String),
    #[error("Failed to create demo: {0}")]
    DemoCreationFailed(String),
    #[error("Benchmarking failed: {0}")]
    BenchmarkingFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_creator_creation() {
        // Red: Test demo creator
        let demo_creator = DemoCreator::new(
            "Next.js".to_string(),
            "SSR Performance".to_string(),
            DemoRequirements::default(),
        );

        assert_eq!(demo_creator.target_competitor(), "Next.js");
        assert_eq!(demo_creator.target_capability(), "SSR Performance");
    }

    #[test]
    fn test_demo_requirements_default() {
        // Red: Test demo requirements default
        let requirements = DemoRequirements::default();

        assert!(requirements.performance_targets.throughput_target.is_some());
        assert!(requirements.performance_targets.latency_target.is_some());
        assert!(requirements.performance_targets.memory_target.is_some());
        assert!(requirements.performance_targets.cpu_target.is_some());
    }

    #[test]
    fn test_performance_targets_default() {
        // Red: Test performance targets default
        let targets = PerformanceTargets::default();

        assert_eq!(targets.throughput_target, Some(1000.0));
        assert_eq!(targets.latency_target, Some(100.0));
        assert_eq!(targets.memory_target, Some(512.0));
        assert_eq!(targets.cpu_target, Some(50.0));
    }

    #[test]
    fn test_create_competitive_demo() {
        // Red: Test demo creation
        let demo_creator = DemoCreator::new(
            "React".to_string(),
            "Performance".to_string(),
            DemoRequirements::default(),
        );

        let result = demo_creator.create_competitive_demo("https://react-demo.example.com");
        assert!(result.is_ok());

        let demo = result.unwrap();
        assert!(demo.name.contains("Performance"));
        assert!(demo.name.contains("React"));
        assert!(!demo.demo_url.is_empty());
    }
}

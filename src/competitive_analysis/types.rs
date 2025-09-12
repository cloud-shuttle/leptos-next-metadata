//! Competitive Analysis Types
//!
//! Core data structures for competitive analysis including competitors,
//! capabilities, performance metrics, and client value assessments.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a competitor in the market
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Competitor {
    pub name: String,
    pub category: CompetitorCategory,
    pub capabilities: Vec<Capability>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub market_share: f64,
    pub last_updated: DateTime<Utc>,
}

impl Competitor {
    /// Create a new competitor
    pub fn new(
        name: String,
        category: CompetitorCategory,
        capabilities: Vec<Capability>,
        strengths: Vec<String>,
        weaknesses: Vec<String>,
        market_share: f64,
    ) -> Self {
        Self {
            name,
            category,
            capabilities,
            strengths,
            weaknesses,
            market_share,
            last_updated: Utc::now(),
        }
    }
}

/// Categories of competitors
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CompetitorCategory {
    WebFramework,
    DataEngineering,
    Analytics,
    MachineLearning,
    RealTimeProcessing,
    DataVisualization,
    CloudPlatform,
    Database,
    MessageQueue,
    Monitoring,
}

/// Represents a specific capability of a competitor or our solution
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub implementation: ImplementationType,
    pub performance_metrics: PerformanceMetrics,
    pub client_value: ClientValue,
}

impl Capability {
    /// Create a new capability
    pub fn new(
        name: String,
        description: String,
        implementation: ImplementationType,
        performance_metrics: PerformanceMetrics,
        client_value: ClientValue,
    ) -> Self {
        Self {
            name,
            description,
            implementation,
            performance_metrics,
            client_value,
        }
    }
}

/// Types of implementation technologies
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ImplementationType {
    Rust,
    Python,
    Java,
    Scala,
    Go,
    JavaScript,
    TypeScript,
    Cpp,
    CSharp,
    Other(String),
}

/// Performance metrics for capabilities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    pub throughput: Option<f64>, // records/second
    pub latency: Option<f64>,    // milliseconds
    pub memory_usage: Option<f64>, // MB
    pub cpu_usage: Option<f64>,  // percentage
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new(
        throughput: Option<f64>,
        latency: Option<f64>,
        memory_usage: Option<f64>,
        cpu_usage: Option<f64>,
    ) -> Self {
        Self {
            throughput,
            latency,
            memory_usage,
            cpu_usage,
        }
    }
}

/// Client value assessment for capabilities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientValue {
    pub cost_effectiveness: f64, // 1-10 scale
    pub ease_of_use: f64,        // 1-10 scale
    pub reliability: f64,        // 1-10 scale
    pub scalability: f64,        // 1-10 scale
    pub innovation: f64,         // 1-10 scale
}

impl ClientValue {
    /// Create new client value assessment
    pub fn new(
        cost_effectiveness: f64,
        ease_of_use: f64,
        reliability: f64,
        scalability: f64,
        innovation: f64,
    ) -> Self {
        Self {
            cost_effectiveness,
            ease_of_use,
            reliability,
            scalability,
            innovation,
        }
    }

    /// Calculate overall client value score
    pub fn overall_score(&self) -> f64 {
        (self.cost_effectiveness + self.ease_of_use + self.reliability +
         self.scalability + self.innovation) / 5.0
    }
}

/// Result of competitor analysis
#[derive(Debug, Clone)]
pub struct CompetitorAnalysis {
    pub competitor: Competitor,
    pub gap_analysis: Vec<String>,
    pub recommendations: Vec<String>,
    pub competitive_advantage: Vec<String>,
}

/// Result of performance benchmarking
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub competitor_metrics: PerformanceMetrics,
    pub our_metrics: PerformanceMetrics,
    pub performance_difference: PerformanceDifference,
    pub recommendations: Vec<String>,
}

/// Performance difference between our solution and competitor
#[derive(Debug, Clone)]
pub struct PerformanceDifference {
    pub throughput_improvement: f64, // percentage improvement
    pub latency_improvement: f64,    // percentage improvement (lower is better)
    pub memory_improvement: f64,     // percentage improvement (lower is better)
    pub cpu_improvement: f64,        // percentage improvement (lower is better)
}

impl PerformanceDifference {
    /// Create new performance difference
    pub fn new(
        throughput_improvement: f64,
        latency_improvement: f64,
        memory_improvement: f64,
        cpu_improvement: f64,
    ) -> Self {
        Self {
            throughput_improvement,
            latency_improvement,
            memory_improvement,
            cpu_improvement,
        }
    }

    /// Calculate overall performance advantage
    pub fn overall_advantage(&self) -> f64 {
        (self.throughput_improvement + self.latency_improvement +
         self.memory_improvement + self.cpu_improvement) / 4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_competitor_creation() {
        // Red: Write a failing test first
        let competitor = Competitor::new(
            "Next.js".to_string(),
            CompetitorCategory::WebFramework,
            vec![],
            vec!["Fast development".to_string()],
            vec!["Complex setup".to_string()],
            25.5,
        );

        assert_eq!(competitor.name, "Next.js");
        assert_eq!(competitor.category, CompetitorCategory::WebFramework);
        assert_eq!(competitor.market_share, 25.5);
        assert!(competitor.strengths.contains(&"Fast development".to_string()));
        assert!(competitor.weaknesses.contains(&"Complex setup".to_string()));
    }

    #[test]
    fn test_capability_creation() {
        // Red: Test capability creation
        let capability = Capability::new(
            "SSR Support".to_string(),
            "Server-side rendering capabilities".to_string(),
            ImplementationType::Rust,
            PerformanceMetrics::new(Some(1000.0), Some(50.0), Some(128.0), Some(30.0)),
            ClientValue::new(9.0, 8.0, 9.5, 8.5, 9.0),
        );

        assert_eq!(capability.name, "SSR Support");
        assert_eq!(capability.description, "Server-side rendering capabilities");
        assert_eq!(capability.implementation, ImplementationType::Rust);
        assert_eq!(capability.performance_metrics.throughput, Some(1000.0));
        assert_eq!(capability.client_value.reliability, 9.5);
    }

    #[test]
    fn test_client_value_overall_score() {
        // Red: Test client value scoring
        let client_value = ClientValue::new(8.0, 7.0, 9.0, 8.5, 7.5);
        let overall_score = client_value.overall_score();

        // Should be average of all scores
        assert_eq!(overall_score, 8.0);
    }

    #[test]
    fn test_performance_difference_overall_advantage() {
        // Red: Test performance difference calculation
        let performance_diff = PerformanceDifference::new(50.0, 25.0, 30.0, 20.0);
        let overall_advantage = performance_diff.overall_advantage();

        // Should be average of all improvements
        assert_eq!(overall_advantage, 31.25);
    }
}

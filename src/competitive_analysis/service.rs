//! Competitive Analysis Service
//!
//! Core service for managing competitive analysis including competitor data,
//! capability mapping, and benchmarking functionality.

use super::types::*;

/// Main service for competitive analysis
pub struct CompetitiveAnalysisService {
    competitors: Vec<Competitor>,
    our_capabilities: Vec<Capability>,
}

impl CompetitiveAnalysisService {
    /// Create a new competitive analysis service
    pub fn new() -> Self {
        Self {
            competitors: Vec::new(),
            our_capabilities: Vec::new(),
        }
    }

    /// Get all competitors
    pub fn competitors(&self) -> &[Competitor] {
        &self.competitors
    }

    /// Get our capabilities
    pub fn our_capabilities(&self) -> &[Capability] {
        &self.our_capabilities
    }

    /// Add a competitor
    pub fn add_competitor(&mut self, competitor: Competitor) {
        self.competitors.push(competitor);
    }

    /// Add our capability
    pub fn add_our_capability(&mut self, capability: Capability) {
        self.our_capabilities.push(capability);
    }

    /// Analyze a specific competitor
    pub fn analyze_competitor(&self, competitor_name: &str) -> Option<CompetitorAnalysis> {
        let competitor = self
            .competitors
            .iter()
            .find(|c| c.name == competitor_name)?;

        Some(CompetitorAnalysis {
            competitor: competitor.clone(),
            gap_analysis: self.perform_gap_analysis(competitor),
            recommendations: self.generate_recommendations(competitor),
            competitive_advantage: self.identify_competitive_advantage(competitor),
        })
    }

    /// Benchmark against a specific competitor capability
    pub fn benchmark_against_competitor(
        &self,
        competitor_name: &str,
        capability_name: &str,
    ) -> Option<BenchmarkResult> {
        let competitor = self
            .competitors
            .iter()
            .find(|c| c.name == competitor_name)?;

        let competitor_capability = competitor
            .capabilities
            .iter()
            .find(|c| c.name == capability_name)?;

        let our_capability = self
            .our_capabilities
            .iter()
            .find(|c| c.name == capability_name)?;

        Some(BenchmarkResult {
            competitor_metrics: competitor_capability.performance_metrics.clone(),
            our_metrics: our_capability.performance_metrics.clone(),
            performance_difference: self.calculate_performance_difference(
                &competitor_capability.performance_metrics,
                &our_capability.performance_metrics,
            ),
            recommendations: self.generate_performance_recommendations(
                &competitor_capability.performance_metrics,
                &our_capability.performance_metrics,
            ),
        })
    }

    /// Perform gap analysis for a competitor
    fn perform_gap_analysis(&self, competitor: &Competitor) -> Vec<String> {
        let mut gaps = Vec::new();

        // Check for capabilities we have that competitor doesn't
        for our_capability in &self.our_capabilities {
            let competitor_has_capability = competitor
                .capabilities
                .iter()
                .any(|c| c.name == our_capability.name);

            if !competitor_has_capability {
                gaps.push(format!(
                    "We have '{}' capability that {} lacks",
                    our_capability.name, competitor.name
                ));
            }
        }

        // Check for capabilities competitor has that we don't
        for competitor_capability in &competitor.capabilities {
            let we_have_capability = self
                .our_capabilities
                .iter()
                .any(|c| c.name == competitor_capability.name);

            if !we_have_capability {
                gaps.push(format!(
                    "{} has '{}' capability that we lack",
                    competitor.name, competitor_capability.name
                ));
            }
        }

        gaps
    }

    /// Generate recommendations based on competitor analysis
    fn generate_recommendations(&self, competitor: &Competitor) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analyze market share
        if competitor.market_share > 30.0 {
            recommendations.push(format!(
                "{} has significant market share ({}%). Consider strategic positioning.",
                competitor.name, competitor.market_share
            ));
        }

        // Analyze strengths
        for strength in &competitor.strengths {
            recommendations.push(format!(
                "Learn from {}'s strength: {}",
                competitor.name, strength
            ));
        }

        // Analyze weaknesses
        for weakness in &competitor.weaknesses {
            recommendations.push(format!(
                "Exploit {}'s weakness: {}",
                competitor.name, weakness
            ));
        }

        recommendations
    }

    /// Identify our competitive advantages
    fn identify_competitive_advantage(&self, competitor: &Competitor) -> Vec<String> {
        let mut advantages = Vec::new();

        // Compare capabilities
        for our_capability in &self.our_capabilities {
            if let Some(competitor_capability) = competitor
                .capabilities
                .iter()
                .find(|c| c.name == our_capability.name)
            {
                let our_score = our_capability.client_value.overall_score();
                let competitor_score = competitor_capability.client_value.overall_score();

                if our_score > competitor_score {
                    advantages.push(format!(
                        "We outperform {} in '{}' ({} vs {})",
                        competitor.name, our_capability.name, our_score, competitor_score
                    ));
                }
            }
        }

        advantages
    }

    /// Calculate performance difference between our metrics and competitor metrics
    fn calculate_performance_difference(
        &self,
        competitor_metrics: &PerformanceMetrics,
        our_metrics: &PerformanceMetrics,
    ) -> PerformanceDifference {
        let throughput_improvement = if let (Some(ours), Some(theirs)) =
            (our_metrics.throughput, competitor_metrics.throughput)
        {
            ((ours - theirs) / theirs) * 100.0
        } else {
            0.0
        };

        let latency_improvement =
            if let (Some(ours), Some(theirs)) = (our_metrics.latency, competitor_metrics.latency) {
                ((theirs - ours) / theirs) * 100.0 // Lower latency is better
            } else {
                0.0
            };

        let memory_improvement = if let (Some(ours), Some(theirs)) =
            (our_metrics.memory_usage, competitor_metrics.memory_usage)
        {
            ((theirs - ours) / theirs) * 100.0 // Lower memory usage is better
        } else {
            0.0
        };

        let cpu_improvement = if let (Some(ours), Some(theirs)) =
            (our_metrics.cpu_usage, competitor_metrics.cpu_usage)
        {
            ((theirs - ours) / theirs) * 100.0 // Lower CPU usage is better
        } else {
            0.0
        };

        PerformanceDifference::new(
            throughput_improvement,
            latency_improvement,
            memory_improvement,
            cpu_improvement,
        )
    }

    /// Generate performance recommendations
    fn generate_performance_recommendations(
        &self,
        competitor_metrics: &PerformanceMetrics,
        our_metrics: &PerformanceMetrics,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Throughput recommendations
        if let (Some(ours), Some(theirs)) = (our_metrics.throughput, competitor_metrics.throughput)
        {
            if ours > theirs {
                recommendations.push(format!(
                    "We have {}% better throughput",
                    ((ours - theirs) / theirs) * 100.0
                ));
            } else {
                recommendations.push(format!(
                    "We need to improve throughput by {}% to match competitor",
                    ((theirs - ours) / theirs) * 100.0
                ));
            }
        }

        // Latency recommendations
        if let (Some(ours), Some(theirs)) = (our_metrics.latency, competitor_metrics.latency) {
            if ours < theirs {
                recommendations.push(format!(
                    "We have {}% better latency",
                    ((theirs - ours) / theirs) * 100.0
                ));
            } else {
                recommendations.push(format!(
                    "We need to improve latency by {}% to match competitor",
                    ((ours - theirs) / theirs) * 100.0
                ));
            }
        }

        recommendations
    }
}

impl Default for CompetitiveAnalysisService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_competitive_analysis_service_creation() {
        // Red: Test service creation
        let service = CompetitiveAnalysisService::new();

        // Should initialize with empty competitors and capabilities
        assert!(service.competitors().is_empty());
        assert!(service.our_capabilities().is_empty());
    }

    #[test]
    fn test_add_competitor() {
        // Red: Test adding competitors
        let mut service = CompetitiveAnalysisService::new();
        let competitor = Competitor::new(
            "React".to_string(),
            CompetitorCategory::WebFramework,
            vec![],
            vec!["Large ecosystem".to_string()],
            vec!["Bundle size".to_string()],
            40.0,
        );

        service.add_competitor(competitor.clone());

        assert_eq!(service.competitors().len(), 1);
        assert_eq!(service.competitors()[0].name, "React");
    }

    #[test]
    fn test_analyze_competitor() {
        // Red: Test competitor analysis
        let mut service = CompetitiveAnalysisService::new();
        let competitor = Competitor::new(
            "Vue.js".to_string(),
            CompetitorCategory::WebFramework,
            vec![],
            vec!["Easy learning curve".to_string()],
            vec!["Smaller ecosystem".to_string()],
            15.0,
        );

        service.add_competitor(competitor);

        // Add our capability to generate gap analysis
        let our_capability = Capability::new(
            "SSR Support".to_string(),
            "Server-side rendering".to_string(),
            ImplementationType::Rust,
            PerformanceMetrics::new(Some(1000.0), Some(50.0), Some(128.0), Some(30.0)),
            ClientValue::new(9.0, 8.5, 9.5, 9.0, 9.5),
        );
        service.add_our_capability(our_capability);

        let analysis = service.analyze_competitor("Vue.js");
        assert!(analysis.is_some());

        let analysis = analysis.unwrap();
        assert_eq!(analysis.competitor.name, "Vue.js");
        assert!(!analysis.gap_analysis.is_empty());
        assert!(!analysis.recommendations.is_empty());
    }

    #[test]
    fn test_benchmark_against_competitor() {
        // Red: Test benchmarking
        let mut service = CompetitiveAnalysisService::new();

        // Add competitor with capability
        let capability = Capability::new(
            "Performance".to_string(),
            "Runtime performance".to_string(),
            ImplementationType::JavaScript,
            PerformanceMetrics::new(Some(500.0), Some(100.0), Some(256.0), Some(60.0)),
            ClientValue::new(7.0, 8.0, 7.5, 7.0, 6.5),
        );

        let competitor = Competitor::new(
            "Angular".to_string(),
            CompetitorCategory::WebFramework,
            vec![capability],
            vec!["Enterprise features".to_string()],
            vec!["Steep learning curve".to_string()],
            20.0,
        );

        service.add_competitor(competitor);

        // Add our capability
        let our_capability = Capability::new(
            "Performance".to_string(),
            "Runtime performance".to_string(),
            ImplementationType::Rust,
            PerformanceMetrics::new(Some(1000.0), Some(25.0), Some(128.0), Some(30.0)),
            ClientValue::new(9.0, 8.5, 9.5, 9.0, 9.5),
        );

        service.add_our_capability(our_capability);

        let benchmark = service.benchmark_against_competitor("Angular", "Performance");
        assert!(benchmark.is_some());

        let benchmark = benchmark.unwrap();
        assert!(benchmark.performance_difference.throughput_improvement > 0.0);
        assert!(benchmark.performance_difference.latency_improvement > 0.0);
    }
}

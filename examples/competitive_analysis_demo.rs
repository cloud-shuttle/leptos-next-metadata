//! Competitive Analysis Demo
//!
//! This example demonstrates the competitive analysis capabilities of leptos-next-metadata,
//! showing how to analyze competitors, benchmark performance, and create competitive demos.

use leptos_next_metadata::competitive_analysis::*;
use leptos::prelude::*;

fn main() {
    // Initialize Leptos
    leptos::mount_to_body(|| view! {
        <CompetitiveAnalysisDemo />
    });
}

#[component]
fn CompetitiveAnalysisDemo() -> impl IntoView {
    let (analysis_service, set_analysis_service) = create_signal(CompetitiveAnalysisService::new());

    // Initialize with some sample data
    let initialize_data = move || {
        let mut service = analysis_service.get();

        // Add our capabilities
        let our_ssr_capability = Capability::new(
            "SSR Performance".to_string(),
            "Server-side rendering with Rust performance".to_string(),
            ImplementationType::Rust,
            PerformanceMetrics::new(Some(2000.0), Some(25.0), Some(128.0), Some(30.0)),
            ClientValue::new(9.0, 8.5, 9.5, 9.0, 9.5),
        );

        let our_metadata_capability = Capability::new(
            "Metadata Management".to_string(),
            "Type-safe metadata with compile-time validation".to_string(),
            ImplementationType::Rust,
            PerformanceMetrics::new(Some(5000.0), Some(10.0), Some(64.0), Some(15.0)),
            ClientValue::new(9.5, 9.0, 9.8, 8.5, 9.2),
        );

        service.add_our_capability(our_ssr_capability);
        service.add_our_capability(our_metadata_capability);

        // Add competitors
        let nextjs_capability = Capability::new(
            "SSR Performance".to_string(),
            "Server-side rendering with Node.js".to_string(),
            ImplementationType::JavaScript,
            PerformanceMetrics::new(Some(1000.0), Some(50.0), Some(256.0), Some(60.0)),
            ClientValue::new(8.0, 9.0, 8.5, 7.5, 8.0),
        );

        let nextjs_competitor = Competitor::new(
            "Next.js".to_string(),
            CompetitorCategory::WebFramework,
            vec![nextjs_capability],
            vec!["Large ecosystem".to_string(), "Great developer experience".to_string()],
            vec!["JavaScript performance".to_string(), "Bundle size".to_string()],
            25.5,
        );

        let react_capability = Capability::new(
            "SSR Performance".to_string(),
            "Server-side rendering with React".to_string(),
            ImplementationType::JavaScript,
            PerformanceMetrics::new(Some(800.0), Some(75.0), Some(320.0), Some(70.0)),
            ClientValue::new(7.5, 8.5, 8.0, 7.0, 7.8),
        );

        let react_competitor = Competitor::new(
            "React".to_string(),
            CompetitorCategory::WebFramework,
            vec![react_capability],
            vec!["Huge ecosystem".to_string(), "Mature library".to_string()],
            vec!["Performance overhead".to_string(), "Complex setup".to_string()],
            40.0,
        );

        service.add_competitor(nextjs_competitor);
        service.add_competitor(react_competitor);

        set_analysis_service.set(service);
    };

    // Initialize data on mount
    create_effect(move |_| {
        initialize_data();
    });

    view! {
        <div class="competitive-analysis-demo">
            <h1>"Competitive Analysis Demo"</h1>
            <p>"This demo showcases our competitive analysis capabilities against major web frameworks."</p>

            <div class="analysis-section">
                <h2>"Our Capabilities"</h2>
                <div class="capabilities-grid">
                    {move || {
                        analysis_service.get().our_capabilities().iter().map(|capability| {
                            view! {
                                <div class="capability-card">
                                    <h3>{&capability.name}</h3>
                                    <p>{&capability.description}</p>
                                    <div class="performance-metrics">
                                        <div class="metric">
                                            <span class="metric-label">"Throughput:"</span>
                                            <span class="metric-value">
                                                {capability.performance_metrics.throughput
                                                    .map(|t| format!("{:.0} req/s", t))
                                                    .unwrap_or_else(|| "N/A".to_string())}
                                            </span>
                                        </div>
                                        <div class="metric">
                                            <span class="metric-label">"Latency:"</span>
                                            <span class="metric-value">
                                                {capability.performance_metrics.latency
                                                    .map(|l| format!("{:.0}ms", l))
                                                    .unwrap_or_else(|| "N/A".to_string())}
                                            </span>
                                        </div>
                                        <div class="metric">
                                            <span class="metric-label">"Memory:"</span>
                                            <span class="metric-value">
                                                {capability.performance_metrics.memory_usage
                                                    .map(|m| format!("{:.0}MB", m))
                                                    .unwrap_or_else(|| "N/A".to_string())}
                                            </span>
                                        </div>
                                    </div>
                                    <div class="client-value">
                                        <span class="value-label">"Overall Score:"</span>
                                        <span class="value-score">
                                            {format!("{:.1}/10", capability.client_value.overall_score())}
                                        </span>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            <div class="analysis-section">
                <h2>"Competitor Analysis"</h2>
                <div class="competitors-grid">
                    {move || {
                        analysis_service.get().competitors().iter().map(|competitor| {
                            let analysis = analysis_service.get().analyze_competitor(&competitor.name);
                            view! {
                                <div class="competitor-card">
                                    <h3>{&competitor.name}</h3>
                                    <div class="competitor-info">
                                        <div class="market-share">
                                            <span class="label">"Market Share:"</span>
                                            <span class="value">{format!("{:.1}%", competitor.market_share)}</span>
                                        </div>
                                        <div class="strengths">
                                            <h4>"Strengths"</h4>
                                            <ul>
                                                {competitor.strengths.iter().map(|strength| {
                                                    view! { <li>{strength}</li> }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                        <div class="weaknesses">
                                            <h4>"Weaknesses"</h4>
                                            <ul>
                                                {competitor.weaknesses.iter().map(|weakness| {
                                                    view! { <li>{weakness}</li> }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                    </div>

                                    {if let Some(analysis) = analysis {
                                        view! {
                                            <div class="analysis-results">
                                                <h4>"Gap Analysis"</h4>
                                                <ul>
                                                    {analysis.gap_analysis.iter().map(|gap| {
                                                        view! { <li class="gap-item">{gap}</li> }
                                                    }).collect::<Vec<_>>()}
                                                </ul>

                                                <h4>"Recommendations"</h4>
                                                <ul>
                                                    {analysis.recommendations.iter().map(|rec| {
                                                        view! { <li class="recommendation">{rec}</li> }
                                                    }).collect::<Vec<_>>()}
                                                </ul>

                                                <h4>"Our Advantages"</h4>
                                                <ul>
                                                    {analysis.competitive_advantage.iter().map(|adv| {
                                                        view! { <li class="advantage">{adv}</li> }
                                                    }).collect::<Vec<_>>()}
                                                </ul>
                                            </div>
                                        }
                                    } else {
                                        view! { <p>"No analysis available"</p> }
                                    }}
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            <div class="analysis-section">
                <h2>"Performance Benchmarking"</h2>
                <div class="benchmark-results">
                    {move || {
                        let service = analysis_service.get();
                        let benchmarks = ["Next.js", "React"].iter()
                            .filter_map(|competitor_name| {
                                service.benchmark_against_competitor(competitor_name, "SSR Performance")
                            })
                            .collect::<Vec<_>>();

                        if benchmarks.is_empty() {
                            view! { <p>"No benchmark data available"</p> }
                        } else {
                            view! {
                                <div class="benchmark-grid">
                                    {benchmarks.iter().map(|benchmark| {
                                        view! {
                                            <div class="benchmark-card">
                                                <h3>"SSR Performance Comparison"</h3>
                                                <div class="performance-comparison">
                                                    <div class="our-performance">
                                                        <h4>"Our Performance"</h4>
                                                        <div class="metrics">
                                                            <div class="metric">
                                                                <span class="label">"Throughput:"</span>
                                                                <span class="value">
                                                                    {benchmark.our_metrics.throughput
                                                                        .map(|t| format!("{:.0} req/s", t))
                                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                                </span>
                                                            </div>
                                                            <div class="metric">
                                                                <span class="label">"Latency:"</span>
                                                                <span class="value">
                                                                    {benchmark.our_metrics.latency
                                                                        .map(|l| format!("{:.0}ms", l))
                                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                                </span>
                                                            </div>
                                                        </div>
                                                    </div>

                                                    <div class="competitor-performance">
                                                        <h4>"Competitor Performance"</h4>
                                                        <div class="metrics">
                                                            <div class="metric">
                                                                <span class="label">"Throughput:"</span>
                                                                <span class="value">
                                                                    {benchmark.competitor_metrics.throughput
                                                                        .map(|t| format!("{:.0} req/s", t))
                                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                                </span>
                                                            </div>
                                                            <div class="metric">
                                                                <span class="label">"Latency:"</span>
                                                                <span class="value">
                                                                    {benchmark.competitor_metrics.latency
                                                                        .map(|l| format!("{:.0}ms", l))
                                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                                </span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>

                                                <div class="performance-improvement">
                                                    <h4>"Performance Improvement"</h4>
                                                    <div class="improvement-metrics">
                                                        <div class="improvement">
                                                            <span class="label">"Throughput:"</span>
                                                            <span class="value positive">
                                                                {format!("+{:.1}%", benchmark.performance_difference.throughput_improvement)}
                                                            </span>
                                                        </div>
                                                        <div class="improvement">
                                                            <span class="label">"Latency:"</span>
                                                            <span class="value positive">
                                                                {format!("+{:.1}%", benchmark.performance_difference.latency_improvement)}
                                                            </span>
                                                        </div>
                                                        <div class="improvement">
                                                            <span class="label">"Memory:"</span>
                                                            <span class="value positive">
                                                                {format!("+{:.1}%", benchmark.performance_difference.memory_improvement)}
                                                            </span>
                                                        </div>
                                                        <div class="improvement">
                                                            <span class="label">"CPU:"</span>
                                                            <span class="value positive">
                                                                {format!("+{:.1}%", benchmark.performance_difference.cpu_improvement)}
                                                            </span>
                                                        </div>
                                                    </div>
                                                </div>

                                                <div class="recommendations">
                                                    <h4>"Recommendations"</h4>
                                                    <ul>
                                                        {benchmark.recommendations.iter().map(|rec| {
                                                            view! { <li>{rec}</li> }
                                                        }).collect::<Vec<_>>()}
                                                    </ul>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

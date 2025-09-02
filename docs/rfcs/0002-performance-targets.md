# RFC 0002: Performance Targets and Benchmarks

- **Start Date**: 2024-03-20
- **RFC PR**: [#3](https://github.com/yourusername/leptos-next-metadata/pull/3)
- **Implementation PR**: [#4](https://github.com/yourusername/leptos-next-metadata/pull/4)

## Summary

This RFC establishes performance targets and benchmarking methodology for leptos-next-metadata to ensure it delivers on the promise of being significantly faster than existing solutions while maintaining excellent developer experience.

## Motivation

Performance is a key differentiator for leptos-next-metadata. Without clear targets and measurement methodology, we risk:

1. **Performance Regression**: Gradual slowdown without visibility
2. **False Claims**: Marketing faster performance without proof
3. **User Disappointment**: Failing to meet expectations set by claims
4. **Competitive Disadvantage**: Losing ground to alternatives

Clear performance targets enable:
- **Objective Measurement**: Data-driven performance decisions
- **Continuous Monitoring**: Catch regressions early
- **Marketing Confidence**: Back up performance claims with data
- **User Trust**: Deliver on performance promises

## Detailed Design

### Performance Categories

#### 1. OG Image Generation

**Primary Target**: **100ms average generation time**

| Scenario | Target | Baseline (Puppeteer) | Improvement |
|----------|--------|---------------------|-------------|
| Simple template (text only) | 50ms | 600ms | 12x faster |
| Complex template (text + images) | 150ms | 1200ms | 8x faster |
| Custom fonts | 80ms | 800ms | 10x faster |
| Batch generation (10 images) | 800ms | 8000ms | 10x faster |

**Measurement Conditions**:
- Hardware: Modern laptop (M1 MacBook Pro equivalent)
- Template: Standard blog post template
- Font: Single web font (Inter)
- Image size: 1200x630 PNG
- Concurrent requests: 1

#### 2. Metadata Resolution

**Primary Target**: **Sub-millisecond static, sub-10ms dynamic**

| Scenario | Target | Measurement |
|----------|--------|-------------|
| Static metadata (compile-time) | 0ms | Macro expansion only |
| Static metadata (runtime) | <1ms | Component render time |
| Dynamic metadata (cached) | <5ms | Resource resolution |
| Dynamic metadata (uncached) | <10ms | Including data fetch overhead |
| Template resolution | <2ms | String interpolation |

#### 3. Build Performance

**Primary Target**: **<5% build time increase**

| Project Size | Baseline | With leptos-next-metadata | Increase |
|-------------|----------|--------------------------|----------|
| Small (10 components) | 20s | 21s | 5% |
| Medium (50 components) | 60s | 62s | 3.3% |
| Large (200 components) | 180s | 189s | 5% |

#### 4. Runtime Performance

**Primary Target**: **Minimal memory overhead**

| Metric | Target | Measurement |
|--------|--------|-------------|
| Bundle size (minimal) | <50KB | Tree-shaken build |
| Bundle size (full features) | <200KB | Complete feature set |
| Memory usage (static) | <100KB | Runtime allocation |
| Memory usage (dynamic) | <500KB | Including caches |
| Cold start overhead | <5ms | First component render |

### Benchmarking Methodology

#### Benchmark Suite Structure

```rust
// benches/og_image_generation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_next_metadata::og_image::*;

fn benchmark_og_simple(c: &mut Criterion) {
    let generator = OgImageGenerator::new();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("og_simple_text", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let params = OgImageParams {
                    template: "simple".to_string(),
                    data: liquid::object!({
                        "title" => black_box("Test Title"),
                        "description" => black_box("Test Description"),
                    }),
                };
                generator.generate(params).await.unwrap()
            })
        })
    });
}

fn benchmark_og_complex(c: &mut Criterion) {
    let generator = OgImageGenerator::new();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("og_complex_template", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let params = OgImageParams {
                    template: "blog".to_string(),
                    data: liquid::object!({
                        "title" => black_box("Complex Blog Post Title"),
                        "author" => black_box("Author Name"),
                        "date" => black_box("March 20, 2024"),
                        "category" => black_box("Technology"),
                        "reading_time" => black_box("5 min read"),
                        "image" => black_box("/author-avatar.jpg"),
                    }),
                };
                generator.generate(params).await.unwrap()
            })
        })
    });
}

criterion_group!(og_benches, benchmark_og_simple, benchmark_og_complex);
criterion_main!(og_benches);
```

#### Comparative Benchmarks

```rust
// benches/comparison.rs  
use criterion::*;

fn compare_with_puppeteer(c: &mut Criterion) {
    let mut group = c.benchmark_group("og_generation_comparison");
    
    // leptos-next-metadata implementation
    group.bench_function("leptos_next_metadata", |b| {
        b.iter(|| generate_with_leptos_next_metadata(test_data()))
    });
    
    // Puppeteer baseline (for comparison only)
    #[cfg(feature = "puppeteer-comparison")]
    group.bench_function("puppeteer_baseline", |b| {
        b.iter(|| generate_with_puppeteer(test_data()))
    });
    
    group.finish();
}
```

#### Memory Profiling

```rust
// benches/memory_usage.rs
use criterion::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Custom allocator to track memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_static_metadata", |b| {
        b.iter_custom(|iters| {
            let start_memory = ALLOCATED.load(Ordering::SeqCst);
            let start = std::time::Instant::now();
            
            for _i in 0..iters {
                black_box(generate_static_metadata());
            }
            
            let elapsed = start.elapsed();
            let end_memory = ALLOCATED.load(Ordering::SeqCst);
            let memory_used = end_memory.saturating_sub(start_memory);
            
            println!("Memory used: {} bytes", memory_used);
            elapsed
        });
    });
}
```

### Performance Testing Infrastructure

#### Continuous Integration

```yaml
# .github/workflows/performance.yml
name: Performance Testing

on:
  pull_request:
    branches: [main]
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * *'  # Daily performance runs

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libfontconfig1-dev
      
      - name: Run benchmarks
        run: |
          cargo bench --all-features -- --output-format json > benchmark_results.json
      
      - name: Compare with baseline
        uses: benchmark-action/github-action-benchmark@v1
        with:
          name: leptos-next-metadata Benchmark
          tool: 'cargo'
          output-file-path: benchmark_results.json
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
          # Alert if performance degrades by 5%
          alert-threshold: '105%'
          comment-on-alert: true
```

#### Performance Dashboard

```rust
// tools/performance-dashboard/src/main.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct BenchmarkResult {
    name: String,
    value: f64,
    unit: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)] 
struct PerformanceReport {
    og_image_generation: BenchmarkResult,
    metadata_resolution: BenchmarkResult,
    build_time_impact: BenchmarkResult,
    bundle_size: BenchmarkResult,
}

fn generate_performance_report() -> PerformanceReport {
    // Collect benchmark results
    let og_result = run_og_benchmarks();
    let metadata_result = run_metadata_benchmarks();
    let build_result = measure_build_impact();
    let bundle_result = measure_bundle_size();
    
    PerformanceReport {
        og_image_generation: og_result,
        metadata_resolution: metadata_result, 
        build_time_impact: build_result,
        bundle_size: bundle_result,
    }
}

fn main() {
    let report = generate_performance_report();
    
    // Generate HTML dashboard
    let html = generate_dashboard_html(&report);
    std::fs::write("performance_report.html", html).unwrap();
    
    // Check if targets are met
    check_performance_targets(&report);
}
```

### Target Validation

#### Automated Target Checking

```rust
// tests/performance_targets.rs
#[test]
fn test_og_generation_targets() {
    let generator = create_test_generator();
    let params = create_simple_test_params();
    
    let start = std::time::Instant::now();
    let _result = generator.generate(params).unwrap();
    let duration = start.elapsed();
    
    // Target: 100ms average, allow 150ms for CI variance
    assert!(
        duration.as_millis() < 150, 
        "OG generation took {}ms, target is <100ms", 
        duration.as_millis()
    );
}

#[test]
fn test_static_metadata_performance() {
    let start = std::time::Instant::now();
    
    // Generate static metadata 1000 times
    for _ in 0..1000 {
        black_box(generate_static_metadata());
    }
    
    let duration = start.elapsed();
    let per_call = duration.as_nanos() / 1000;
    
    // Target: <1ms per call = 1,000,000 nanoseconds
    assert!(
        per_call < 1_000_000,
        "Static metadata took {}ns per call, target is <1,000,000ns",
        per_call
    );
}

#[test] 
fn test_bundle_size_targets() {
    let metadata = std::fs::metadata("target/wasm32-unknown-unknown/release/leptos_next_metadata.wasm").unwrap();
    let size_kb = metadata.len() / 1024;
    
    // Target: <200KB for full build
    assert!(
        size_kb < 200,
        "Bundle size is {}KB, target is <200KB",
        size_kb
    );
}
```

### Performance Optimization Strategy

#### Hot Path Optimization

1. **OG Image Generation**:
   - Cache parsed SVG templates
   - Reuse font databases across requests
   - Optimize PNG compression settings
   - Implement streaming generation for large batches

2. **Metadata Resolution**:
   - Compile-time macro expansion for static content
   - Efficient string interning for repeated values
   - Lazy evaluation for conditional metadata
   - Template caching with content-based keys

3. **Build Time**:
   - Incremental macro compilation
   - Parallel file convention scanning
   - Cached template compilation
   - Minimal runtime dependencies

#### Memory Optimization

```rust
// Use string interning for repeated metadata values
lazy_static! {
    static ref STRING_INTERNER: Interner = Interner::new();
}

pub struct OptimizedMetadata {
    title: InternedString,
    description: InternedString,
    // Use Cow for conditionally owned strings
    dynamic_fields: HashMap<&'static str, Cow<'static, str>>,
}
```

#### Caching Strategy

```rust
// Multi-level caching for OG image generation
pub struct CachingOgGenerator {
    // L1: In-memory LRU cache
    memory_cache: Arc<Mutex<LruCache<String, Arc<Vec<u8>>>>>,
    
    // L2: Disk cache for persistence
    disk_cache: Option<DiskCache>,
    
    // L3: CDN cache via headers
    cache_headers: CacheHeaderConfig,
}
```

### Monitoring and Alerting

#### Performance Regression Detection

```rust
// tools/regression-detector/src/lib.rs
pub struct RegressionDetector {
    baseline: PerformanceBenchmarks,
    threshold: f64,  // 5% regression threshold
}

impl RegressionDetector {
    pub fn check_regression(&self, current: &PerformanceBenchmarks) -> Vec<RegressionAlert> {
        let mut alerts = Vec::new();
        
        // Check each performance metric
        if current.og_generation_time > self.baseline.og_generation_time * (1.0 + self.threshold) {
            alerts.push(RegressionAlert {
                metric: "og_generation_time".to_string(),
                baseline: self.baseline.og_generation_time,
                current: current.og_generation_time,
                regression_percent: ((current.og_generation_time / self.baseline.og_generation_time) - 1.0) * 100.0,
            });
        }
        
        alerts
    }
}
```

#### Performance Monitoring Dashboard

- **Real-time Metrics**: Track performance in production
- **Historical Trends**: Visualize performance over time  
- **Comparative Analysis**: Compare against competitors
- **Alert System**: Notify on performance regressions

## Rationale and Alternatives

### Alternative 1: No Explicit Targets

**Pros**: No pressure to meet arbitrary numbers
**Cons**: No way to measure success, potential for performance regressions

### Alternative 2: Relative Targets Only

**Pros**: Always better than competition
**Cons**: Moving baseline, no absolute quality bar

### Chosen Approach: Absolute + Relative Targets

Combines the benefits of both:
- **Absolute targets** ensure consistent quality
- **Relative comparisons** demonstrate competitive advantage
- **Continuous monitoring** catches regressions early

## Implementation Plan

### Phase 1: Baseline Establishment (Week 1)
- Implement core benchmarking suite
- Establish baseline measurements
- Set up CI performance testing

### Phase 2: Optimization (Weeks 2-4)
- Profile and optimize hot paths
- Implement caching strategies
- Meet initial performance targets

### Phase 3: Monitoring (Week 5-6)
- Deploy performance monitoring
- Set up regression alerts
- Create performance dashboard

## Success Metrics

- ✅ OG image generation: <100ms average
- ✅ Static metadata: <1ms resolution
- ✅ Dynamic metadata: <10ms resolution  
- ✅ Build time: <5% increase
- ✅ Bundle size: <200KB full, <50KB minimal
- ✅ Zero performance regressions in CI
- ✅ Performance dashboard operational

---

**Implementation Timeline**: 6 weeks
**Impact**: High - Core value proposition depends on performance claims
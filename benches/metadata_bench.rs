use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leptos_next_metadata::metadata::*;

fn benchmark_metadata_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata_operations");

    // Benchmark merge operations with different sizes
    for size in [1, 10, 50, 100].iter() {
        let parent = create_metadata_with_fields(*size);
        let child = create_metadata_with_fields(*size / 2);

        group.bench_with_input(BenchmarkId::new("merge", size), size, |b, _| {
            b.iter(|| black_box(child.clone()).merge(black_box(&parent.clone())))
        });
    }

    // Benchmark title creation
    group.bench_function("title_static", |b| {
        b.iter(|| Title::Static(black_box("Test Page Title".to_string())))
    });

    group.bench_function("title_template", |b| {
        b.iter(|| Title::Template {
            template: black_box("%s | My Amazing Site".to_string()),
            default: black_box("My Amazing Site".to_string()),
        })
    });

    // Benchmark metadata validation
    group.bench_function("validation_simple", |b| {
        let metadata = create_simple_metadata();
        b.iter(|| {
            // Simple validation - check if title exists
            metadata.title.is_some()
        })
    });

    group.finish();
}

fn benchmark_metadata_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata_builder");

    group.bench_function("builder_chain", |b| {
        b.iter(|| {
            Metadata::default()
                .title(Title::Static(black_box("Test Title".to_string())))
                .description(black_box("Test Description".to_string()))
                .canonical(black_box("https://example.com".to_string()))
        })
    });

    group.finish();
}

// Helper functions
fn create_metadata_with_fields(count: usize) -> Metadata {
    let mut metadata = Metadata::default();

    for i in 0..count {
        metadata = metadata.title(Title::Static(format!("Title {}", i)));
        metadata = metadata.description(format!("Description {}", i));
    }

    metadata
}

fn create_simple_metadata() -> Metadata {
    Metadata::default()
        .title(Title::Static("Simple Title".to_string()))
        .description("Simple Description".to_string())
        .canonical("https://example.com".to_string())
}

criterion_group!(
    benches,
    benchmark_metadata_operations,
    benchmark_metadata_builder
);
criterion_main!(benches);

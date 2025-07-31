use browserslist::{Opts, resolve};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// Benchmark suite focused on the performance improvements made
pub fn bench_performance_improvements(c: &mut Criterion) {
    // Test complex queries that use multiple string joins
    c.bench_function("resolve complex multi-query", |b| {
        b.iter(|| {
            resolve(
                black_box(&[
                    "> 0.5%",
                    "last 2 versions", 
                    "Firefox ESR",
                    "not dead",
                    "not ie <= 10"
                ]), 
                &black_box(Opts::default())
            )
        });
    });

    // Test negated queries that benefit from HashSet optimization
    c.bench_function("resolve with negated queries", |b| {
        b.iter(|| {
            resolve(
                black_box(&[
                    "last 5 versions",
                    "not dead", 
                    "not ie <= 11",
                    "not op_mini all"
                ]), 
                &black_box(Opts::default())
            )
        });
    });

    // Test AND queries that benefit from HashSet optimization
    c.bench_function("resolve with AND queries", |b| {
        b.iter(|| {
            resolve(
                black_box(&[
                    "last 3 versions and > 1%"
                ]), 
                &black_box(Opts::default())
            )
        });
    });

    // Test query parsing with many comma-separated queries
    c.bench_function("resolve many comma separated queries", |b| {
        b.iter(|| {
            resolve(
                black_box(&[
                    "chrome >= 90, firefox >= 90, safari >= 14, edge >= 90, opera >= 76"
                ]), 
                &black_box(Opts::default())
            )
        });
    });
}

criterion_group!(performance_improvements, bench_performance_improvements);
criterion_main!(performance_improvements);
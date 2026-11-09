use criterion::{BenchmarkId, Criterion};
use rust_skills_perf_validation::measurement::{checksum, checksum_rounds, fixture, pipeline};
use std::hint::black_box;

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-black-box-bench");
    let data = [1, 2, 3, 4];
    group.bench_function("discarded_constant_input", |b| {
        b.iter(|| {
            // Intentionally invalid measurement: demonstrate elimination risk.
            let _ = checksum(&data);
        });
    });
    group.bench_function("opaque_input_returned_output", |b| {
        b.iter(|| checksum(black_box(&data)));
    });
    group.bench_function("opaque_input_and_output", |b| {
        b.iter(|| black_box(checksum(black_box(&data))));
    });
    group.finish();

    let mut group = c.benchmark_group("perf-profile-first");
    for size in [32, 4_096] {
        let data = fixture(size);
        // Distinct scopes, not an A/B speedup: compare preparation vs kernel.
        group.bench_function(BenchmarkId::new("whole_pipeline", size), |b| {
            b.iter(|| pipeline(black_box(size), black_box(16)));
        });
        group.bench_function(BenchmarkId::new("kernel_only", size), |b| {
            b.iter(|| checksum_rounds(black_box(&data), black_box(16)));
        });
    }
    group.finish();

    let data = fixture(4_096);
    c.bench_function("perf-release-profile/checksum", |b| {
        b.iter(|| checksum_rounds(black_box(&data), black_box(16)));
    });
}

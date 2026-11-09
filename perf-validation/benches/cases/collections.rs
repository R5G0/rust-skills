use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use rust_skills_perf_validation::collections::*;
use std::hint::black_box;
use std::num::NonZeroUsize;

pub fn register(c: &mut Criterion) {
    entry(c);
    drain(c);
    extend(c);
    collect_into(c);
}

fn entry(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-entry-api");
    for size in [32usize, 4096] {
        for (distribution, distinct) in [("repeated", 8), ("unique", size)] {
            let keys: Vec<String> = (0..size)
                .map(|index| format!("key-{}", index % distinct))
                .collect();
            assert_eq!(count_keys_reference(&keys), count_keys_entry(&keys));
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(
                BenchmarkId::new(format!("reference-{distribution}"), size),
                &keys,
                |b, keys| {
                    b.iter(|| black_box(count_keys_reference(black_box(keys))));
                },
            );
            group.bench_with_input(
                BenchmarkId::new(format!("entry-{distribution}"), size),
                &keys,
                |b, keys| {
                    b.iter(|| black_box(count_keys_entry(black_box(keys))));
                },
            );
        }
    }
    group.finish();
}

fn drain(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-drain-reuse");
    for size in [32usize, 4096] {
        let data: Vec<i32> = (0..size as i32).collect();
        let chunk_size = NonZeroUsize::new(16).unwrap();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(BenchmarkId::new("fresh", size), |b| {
            b.iter_batched(
                || data.clone(),
                |input| {
                    let mut checksum = 0i64;
                    drain_batches_fresh(black_box(input), chunk_size, |batch| {
                        checksum += black_box(batch)
                            .iter()
                            .map(|value| i64::from(*value))
                            .sum::<i64>();
                    });
                    black_box(checksum)
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("reused", size), |b| {
            b.iter_batched(
                || data.clone(),
                |input| {
                    let mut checksum = 0i64;
                    drain_batches_reused(black_box(input), chunk_size, |batch| {
                        checksum += black_box(batch)
                            .iter()
                            .map(|value| i64::from(*value))
                            .sum::<i64>();
                    });
                    black_box(checksum)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn extend(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-extend-batch");
    for size in [32usize, 4096] {
        let data: Vec<i32> = (0..size as i32).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("push", size), &data, |b, input| {
            b.iter(|| {
                let mut destination = Vec::new();
                append_push(&mut destination, black_box(input));
                black_box(destination)
            });
        });
        group.bench_with_input(BenchmarkId::new("extend", size), &data, |b, input| {
            b.iter(|| {
                let mut destination = Vec::new();
                append_extend(&mut destination, black_box(input));
                black_box(destination)
            });
        });
    }
    group.finish();
}

fn collect_into(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-collect-into");
    for size in [32usize, 4096] {
        let batches: Vec<Vec<i32>> = (0..16)
            .map(|batch| {
                (0..size)
                    .map(|index| ((index + batch) % 13) as i32 - 6)
                    .collect()
            })
            .collect();
        group.throughput(Throughput::Elements((size * batches.len()) as u64));
        group.bench_with_input(BenchmarkId::new("fresh", size), &batches, |b, input| {
            b.iter(|| {
                let mut checksum = 0i64;
                visit_filtered_fresh(black_box(input), |batch| {
                    checksum += black_box(batch)
                        .iter()
                        .map(|value| i64::from(*value))
                        .sum::<i64>();
                });
                black_box(checksum)
            });
        });
        group.bench_with_input(
            BenchmarkId::new("stable-extend-reused", size),
            &batches,
            |b, input| {
                b.iter(|| {
                    let mut checksum = 0i64;
                    visit_filtered_reused(black_box(input), |batch| {
                        checksum += black_box(batch)
                            .iter()
                            .map(|value| i64::from(*value))
                            .sum::<i64>();
                    });
                    black_box(checksum)
                });
            },
        );
    }
    group.finish();
}

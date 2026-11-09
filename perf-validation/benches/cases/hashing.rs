use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput};
use rust_skills_perf_validation::hashing::*;

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-ahash");
    for size in [32usize, 4096] {
        group.throughput(Throughput::Elements(size as u64));
        for (distribution, distinct) in [("unique", size), ("repeated", (size / 8).max(1))] {
            for width in [8usize, 128] {
                let names: Vec<_> = (0..size)
                    .map(|index| format!("{:0width$}", index % distinct))
                    .collect();
                let case = format!("strings/{distribution}/width{width}/{size}");
                group.bench_with_input(BenchmarkId::new("std", &case), &names, |b, names| {
                    b.iter(|| count_names_std(black_box(names)));
                });
                group.bench_with_input(BenchmarkId::new("ahash", &case), &names, |b, names| {
                    b.iter(|| count_names_ahash(black_box(names)));
                });
                group.bench_with_input(BenchmarkId::new("fx", &case), &names, |b, names| {
                    b.iter(|| count_names_fx(black_box(names)));
                });
            }
            let nodes: Vec<_> = (0..size)
                .map(|index| ((index % distinct) as u32, format!("node-{index}")))
                .collect();
            let case = format!("integers/{distribution}/{size}");
            group.bench_with_input(BenchmarkId::new("std", &case), &nodes, |b, nodes| {
                b.iter(|| build_nodes_std(black_box(nodes)));
            });
            group.bench_with_input(BenchmarkId::new("ahash", &case), &nodes, |b, nodes| {
                b.iter(|| build_nodes_ahash(black_box(nodes)));
            });
            group.bench_with_input(BenchmarkId::new("fx", &case), &nodes, |b, nodes| {
                b.iter(|| build_nodes_fx(black_box(nodes)));
            });
        }
    }
    group.finish();
}

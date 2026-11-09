use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use rust_skills_perf_validation::iterators::*;

pub fn register(criterion: &mut Criterion) {
    bench_iteration(criterion);
    bench_laziness(criterion);
    bench_collection(criterion);
    bench_chaining(criterion);
}

fn bench_iteration(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("perf-iter-over-index");
    for size in [32_usize, 4096] {
        let input: Vec<_> = (0..size).map(|value| (value % 101) as i32 - 50).collect();
        assert_eq!(sum_squares_indexed(&input), sum_squares_iterated(&input));
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("indexed", size), &input, |b, data| {
            b.iter(|| black_box(sum_squares_indexed(black_box(data))))
        });
        group.bench_with_input(BenchmarkId::new("iterator", size), &input, |b, data| {
            b.iter(|| black_box(sum_squares_iterated(black_box(data))))
        });
    }
    group.finish();
}

fn bench_laziness(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("perf-iter-lazy");
    for size in [32_usize, 4096] {
        for distribution in ["early", "late", "absent"] {
            let mut input = vec![-1_i32; size];
            match distribution {
                "early" => input[0] = 1,
                "late" => input[size - 1] = 1,
                _ => {}
            }
            let label = format!("{distribution}-{size}");
            assert_eq!(any_eager(&input, |x| x > 0), any_lazy(&input, |x| x > 0));
            group.bench_with_input(BenchmarkId::new("eager", &label), &input, |b, data| {
                b.iter(|| black_box(any_eager(black_box(data), |x| x > 0)))
            });
            group.bench_with_input(BenchmarkId::new("lazy", &label), &input, |b, data| {
                b.iter(|| black_box(any_lazy(black_box(data), |x| x > 0)))
            });
        }
    }
    group.finish();
}

fn bench_collection(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("perf-collect-once");
    for size in [32_usize, 4096] {
        for selectivity in ["mixed", "none"] {
            let users: Vec<_> = (0..size)
                .map(|index| User {
                    name: format!("user-{index}"),
                    is_active: index % 2 == 0,
                    is_verified: selectivity == "mixed" && index % 3 == 0,
                })
                .collect();
            assert_eq!(
                process_users_staged(users.clone()),
                process_users_once(users.clone())
            );
            let label = format!("{selectivity}-{size}");
            group.throughput(Throughput::Elements(size as u64));
            group.bench_function(BenchmarkId::new("staged", &label), |b| {
                b.iter_batched(
                    || users.clone(),
                    |input| black_box(process_users_staged(black_box(input))),
                    BatchSize::SmallInput,
                )
            });
            group.bench_function(BenchmarkId::new("once", &label), |b| {
                b.iter_batched(
                    || users.clone(),
                    |input| black_box(process_users_once(black_box(input))),
                    BatchSize::SmallInput,
                )
            });
        }
    }
    group.finish();
}

fn bench_chaining(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("perf-chain-avoid");
    for size in [32_usize, 4096] {
        let left: Vec<_> = (0..size).map(|value| (value % 101) as i32 - 50).collect();
        let right: Vec<_> = left.iter().copied().rev().collect();
        assert_eq!(sum_chained(&left, &right), sum_separate(&left, &right));
        group.throughput(Throughput::Elements((size * 2) as u64));
        group.bench_function(BenchmarkId::new("chain", size), |b| {
            b.iter(|| black_box(sum_chained(black_box(&left), black_box(&right))))
        });
        group.bench_function(BenchmarkId::new("separate", size), |b| {
            b.iter(|| black_box(sum_separate(black_box(&left), black_box(&right))))
        });
    }
    group.finish();
}

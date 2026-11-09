# test-criterion-bench

> Use `criterion` for benchmarking

## Why It Matters

Criterion provides statistically rigorous benchmarking with warmup, multiple iterations, outlier detection, and comparison between runs. It's far more reliable than simple timing with `Instant::now()`.

## Setup

```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

## Basic Benchmark

```rust
// benches/my_benchmark.rs
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

## black_box is Critical

```rust
// BAD: Constant input and an explicitly discarded result
b.iter(|| { let _ = fibonacci(20); });

// GOOD: Hide the input; Criterion consumes the returned result
b.iter(|| fibonacci(black_box(20)));

// An explicit output barrier is also possible
b.iter(|| black_box(fibonacci(black_box(20))));
```

Returning a value from `b.iter` already lets Criterion consume it. An explicit
discard inside the closure is different. `std::hint::black_box` is a best-effort
optimization barrier; it does not verify the result or guarantee a valid workload.

## Reset Inputs and Define the Timed Work

Validate expected results outside the timed loop, and benchmark the same logical
operation for each implementation. For consumed or mutated inputs, reset the
fixture before every iteration; repeatedly sorting an already sorted vector or
draining an empty buffer changes the workload.

<!-- verify: compile -->
```rust
use std::hint::black_box;
use criterion::{BatchSize, Criterion};

fn bench_sort(c: &mut Criterion) {
    let input: Vec<i32> = (0..1000).rev().collect();
    c.bench_function("sort-unsorted", |b| {
        b.iter_batched(
            || input.clone(), // setup: excluded from the measured sorting operation
            |mut values| {
                values.sort_unstable();
                black_box(values)
            },
            BatchSize::SmallInput,
        );
    });
}
```

Use `iter_batched_ref` when retaining ownership of the prepared fixture is useful.
Include allocation, preparation, cleanup, or I/O in timing only when those costs
are part of the question being measured, and give both implementations the same
boundary. Consume lazy results and verify the intended success or error path.

## Comparing Implementations

```rust
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("String concat");
    
    let data = "hello";
    
    group.bench_function("format!", |b| {
        b.iter(|| format!("{}{}", black_box(data), " world"))
    });
    
    group.bench_function("push_str", |b| {
        b.iter(|| {
            let mut s = String::from(black_box(data));
            s.push_str(" world");
            s
        })
    });
    
    group.bench_function("concat", |b| {
        b.iter(|| [black_box(data), " world"].concat())
    });
    
    group.finish();
}
```

## Parameterized Benchmarks

```rust
fn bench_vec_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec::push");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v = Vec::new();
                    for i in 0..size {
                        v.push(black_box(i));
                    }
                    v
                });
            },
        );
    }
    
    group.finish();
}
```

## Throughput Measurement

```rust
use criterion::Throughput;

fn bench_parse(c: &mut Criterion) {
    let input = "a]ong string to parse...";
    
    let mut group = c.benchmark_group("Parser");
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("parse", |b| {
        b.iter(|| parse(black_box(input)))
    });
    
    group.finish();
}
```

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench my_benchmark -- fib

# On the original implementation, save a named baseline
cargo bench --bench my_benchmark -- --save-baseline before

# After the change, compare with that data in the same target directory
cargo bench --bench my_benchmark -- --baseline before
```

Keep the saved baseline artifacts between runs and record the source revision,
toolchain, CPU, profile, features, and workload. Do not overwrite `before` with
candidate results. Compare repeated samples and uncertainty estimates; ordinary
unit tests should not assert wall-clock speed ratios on shared runners.

## Evidence from tokio

```rust
// https://github.com/tokio-rs/tokio/blob/master/benches/sync_mpsc.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn send_data<T: Default, const SIZE: usize>(
    g: &mut BenchmarkGroup<WallTime>, 
    prefix: &str
) {
    let rt = rt();
    g.bench_function(format!("{prefix}_{SIZE}"), |b| {
        b.iter(|| {
            let (tx, mut rx) = mpsc::channel::<T>(SIZE);
            rt.block_on(tx.send(T::default())).unwrap();
            rt.block_on(rx.recv()).unwrap();
        })
    });
}
```

## See Also

- [perf-profile-first](perf-profile-first.md) - Profile before optimizing
- [perf-black-box-bench](perf-black-box-bench.md) - Use black_box in benchmarks

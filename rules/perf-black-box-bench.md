# perf-black-box-bench

> Use black_box in benchmarks

## Why It Matters

The compiler may eliminate unused computations or precompute constant inputs. `std::hint::black_box()` discourages those transformations when they would make a benchmark unrealistic. This is a best-effort hint, not a correctness, constant-time, or inlining guarantee.

## Bad

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benchmark_bad(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            let result = expensive_computation(42);
            // Result unused - compiler may eliminate the call!
        });
    });
}

fn benchmark_also_bad(c: &mut Criterion) {
    let input = 42;  // Constant - compiler may precompute
    
    c.bench_function("compute", |b| {
        b.iter(|| {
            let _ = expensive_computation(input);
            // Constant input and discarded output permit elimination.
        });
    });
}
```

## Good

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benchmark_good(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            // black_box on input prevents constant folding
            let result = expensive_computation(black_box(42));
            // black_box on output prevents dead code elimination
            black_box(result)
        });
    });
}

// Or simpler with Criterion's built-in support
fn benchmark_simpler(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| expensive_computation(black_box(42)))
    });
}
```

## What black_box Does

| Without black_box | With black_box |
|-------------------|----------------|
| Input may be constant-folded | Input treated as unknown |
| Result may be eliminated | Discourages eliminating the observed computation |
| Loops may be optimized away | Discourages unrealistic elimination; best effort |
| Functions may be inlined | Inlining remains allowed |

## Standard Library Usage

```rust
use std::hint::black_box;

fn main() {
    // In std since Rust 1.66
    let result = black_box(compute_something(black_box(input)));
}
```

## Criterion's black_box

Prefer `std::hint::black_box` on the supported Rust version. Older Criterion versions also provide a helper:

```rust
use criterion::black_box;

// Prefer std::hint::black_box for the standard library contract.
```

## Pattern: Benchmark with Setup

```rust
fn benchmark_with_setup(c: &mut Criterion) {
    c.bench_function("process_data", |b| {
        // Setup outside iter - not measured
        let data = generate_test_data(1000);
        
        b.iter(|| {
            // black_box the input reference
            let result = process(black_box(&data));
            black_box(result)
        });
    });
}
```

## Pattern: Benchmark Multiple Inputs

```rust
fn benchmark_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    
    for size in [100, 1000, 10000] {
        let data = generate_data(size);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &data,
            |b, data| {
                b.iter(|| process(black_box(data)))
            },
        );
    }
    group.finish();
}
```

## Common Mistakes

```rust
// WRONG: black_box inside loop does nothing useful
for _ in 0..1000 {
    black_box(());  // Doesn't help
    compute();
}

// RIGHT: black_box the computation result
for _ in 0..1000 {
    black_box(compute());
}

// WRONG: Only blocking output, not input
let x = 42;  // Constant, may be optimized
black_box(expensive(x));

// RIGHT: Block both
black_box(expensive(black_box(42)));
```

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

## See Also

Criterion consumes the value returned by `b.iter`, so a returned result is already protected from simply being discarded by the harness. Benchmark assertions belong outside the timed region. The [executable comparison](../perf-validation/benches/cases/measurement.rs) keeps the input identical across discarded-output and opaque-input/output cases; timings alone do not prove which compiler transformation occurred. See the [standard library contract](https://doc.rust-lang.org/std/hint/fn.black_box.html).

- [test-criterion-bench](./test-criterion-bench.md) - Using Criterion
- [perf-profile-first](./perf-profile-first.md) - Profile before optimize
- [perf-release-profile](./perf-release-profile.md) - Release settings

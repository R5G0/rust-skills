# perf-profile-first

> Profile before optimizing

## Why It Matters

Intuition about performance is often wrong. The code you think is slow frequently isn't, while actual bottlenecks hide in unexpected places. Profiling shows you exactly where time is spent, preventing wasted effort on optimizations that don't matter.

## Bad

```rust
// Optimizing without measuring
fn process(data: &[Item]) -> Vec<Output> {
    // "I bet this clone is slow..."
    let cloned: Vec<_> = data.iter().cloned().collect();
    
    // Actually, 99% of time is spent here:
    cloned.iter().map(|x| expensive_computation(x)).collect()
}

// Over-engineering rarely-called code
#[inline(always)]
fn rarely_called() {
    // This runs once at startup...
}
```

## Good

```rust
// 1. Profile first
// cargo flamegraph --bin myapp
// cargo instruments -t time --bin myapp (macOS)

// 2. Find the actual bottleneck
// Flamegraph shows expensive_computation takes 95% of time

// 3. Optimize the hot spot
fn process(data: &[Item]) -> Vec<Output> {
    // Clone is fine - only 1% of time
    let cloned: Vec<_> = data.iter().cloned().collect();
    
    // Focus optimization HERE
    cloned.par_iter()  // Parallelize the expensive part
        .map(|x| expensive_computation(x))
        .collect()
}
```

## Profiling Tools

### Flamegraphs (Recommended Start)

```bash
# Install
cargo install flamegraph

# Profile
cargo flamegraph --bin myapp -- <args>

# Opens flamegraph.svg showing call stacks by time
```

### perf (Linux)

```bash
# Record
perf record -g cargo run --release

# Report
perf report

# Or generate flamegraph
perf script | inferno-collapse-perf | inferno-flamegraph > flamegraph.svg
```

### Instruments (macOS)

```bash
# Install cargo-instruments
cargo install cargo-instruments

# Time profiler
cargo instruments -t time --release

# Allocations profiler
cargo instruments -t alloc --release
```

### DHAT (Heap Profiling)

```bash
# In your code
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    // ... your code
}

# Run and get allocation report
cargo run --release
```

### criterion (Micro-benchmarks)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_my_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| my_function(black_box(input)))
    });
}

criterion_group!(benches, bench_my_function);
criterion_main!(benches);
```

## What to Look For

```
Flamegraph Reading:
├── Width = time spent
├── Height = call stack depth
└── Look for:
    ├── Wide bars (time hogs)
    ├── malloc/free (allocation heavy)
    ├── memcpy (copying data)
    └── Unexpected functions taking time
```

## Common Findings

```rust
// Finding: HashMap operations are slow
// Candidate: measure another hasher only after checking who controls the keys.

// Finding: String allocation in hot loop
// Fix: Pre-allocate with capacity, use &str

// Finding: Clone in hot path
// Fix: Use references or Cow

// Finding: Bounds checks visible in profile
// Fix: Use iterators instead of indexing

// Finding: Lock contention
// Fix: Reduce critical section, use RwLock, or partition data
```

## Optimization Workflow

```
1. Define correct behavior and cover the changed path with regression tests
2. Write benchmarks for hot paths
3. Profile under realistic load
4. Identify actual bottlenecks
5. Save a baseline, then optimize ONE thing
6. Rerun correctness tests and compare measurements and uncertainty
7. Check the affected application workload; retain only justified changes
```

## Evidence: Rust Performance Book

> "The biggest performance improvements often come from changes to algorithms or data structures, rather than low-level optimizations."

> "It is worth understanding which Rust data structures and operations cause allocations, because avoiding them can greatly improve performance."

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

## See Also

The [validation project](../perf-validation/README.md) demonstrates a prepared kernel and a whole pipeline with checked equal results. Their timing scopes differ: this illustrates where costs occur, not an A/B speedup or a substitute for sampling a real application's call stacks. Record the workload, CPU, toolchain, features, profile, and compiler flags with any performance claim.

- [opt-lto-release](opt-lto-release.md) - Enable LTO for release builds
- [test-criterion-bench](test-criterion-bench.md) - Use criterion for benchmarking
- [anti-premature-optimize](anti-premature-optimize.md) - Don't optimize without data

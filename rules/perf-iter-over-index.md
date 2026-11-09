# perf-iter-over-index

> Prefer iterators over manual indexing

## Why It Matters

Iterators express traversal without manual index bookkeeping and can help the optimizer remove bounds checks or vectorize loops. Indexed loops can also be optimized well. Neither loop syntax guarantees a particular number of checks, vector instructions, or a speedup; compare optimized builds on the actual workload.

## Bad

```rust
// Manual indexing - bounds checked every iteration
fn sum_squares(data: &[i32]) -> i64 {
    let mut sum = 0i64;
    for i in 0..data.len() {
        sum += (data[i] as i64) * (data[i] as i64);
    }
    sum
}

// Index-based iteration with multiple collections
fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len().min(b.len()) {
        sum += a[i] * b[i];
    }
    sum
}

// Mutating with indices
fn double_values(data: &mut [i32]) {
    for i in 0..data.len() {
        data[i] *= 2;
    }
}
```

## Good

```rust
// Iterator - bounds checks eliminated, SIMD-friendly
fn sum_squares(data: &[i32]) -> i64 {
    data.iter()
        .map(|&x| (x as i64) * (x as i64))
        .sum()
}

// Zip iterators - no manual length handling
fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| x * y)
        .sum()
}

// Mutable iteration
fn double_values(data: &mut [i32]) {
    for x in data.iter_mut() {
        *x *= 2;
    }
}
```

## When Indexing Is Needed

Sometimes you genuinely need indices:

```rust
// Need the index for output or processing
for (i, value) in data.iter().enumerate() {
    println!("Index {}: {}", i, value);
}

// Non-sequential access patterns
fn interleave(data: &mut [i32]) {
    let mid = data.len() / 2;
    for i in 0..mid {
        data.swap(i * 2, mid + i);
    }
}
```

## Performance Comparison

| Pattern | Bounds Checks | SIMD Potential | Clarity |
|---------|---------------|----------------|---------|
| `for i in 0..len` | Can be eliminated when proven in range | Compiler/workload dependent | Medium |
| `for &x in slice` | No explicit indexing | Compiler/workload dependent | High |
| `.iter().enumerate()` | No explicit indexing | Compiler/workload dependent | High |
| `get_unchecked` | None (unsafe) | High | Low |

## Iterator Advantages

```rust
// Chaining operations - single pass
let result: Vec<_> = data.iter()
    .filter(|x| **x > 0)
    .map(|x| x * 2)
    .collect();

// Early termination optimized
let found = data.iter().any(|&x| x == target);

// Parallel iteration (with rayon)
use rayon::prelude::*;
let sum: i64 = data.par_iter().map(|&x| x as i64).sum();
```

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

## See Also

The [formal experiment](../perf-validation/README.md) checks empty inputs, unequal-length zips, mutation results, and bounded arithmetic; its indexed/iterator benchmarks make no expected-winner assertion. The sums shown here require the result to fit in `i64`, and doubling requires each result to fit in `i32`.

- [perf-iter-lazy](./perf-iter-lazy.md) - Keep iterators lazy
- [opt-bounds-check](./opt-bounds-check.md) - Bounds check elimination
- [anti-index-over-iter](./anti-index-over-iter.md) - Anti-pattern
- [conc-rayon-par-iter](./conc-rayon-par-iter.md) - Parallelize data-parallel loops

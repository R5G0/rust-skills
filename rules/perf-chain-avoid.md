# perf-chain-avoid

> Measure chained iteration before rewriting a hot loop

## Why It Matters

`Iterator::chain()` combines two iterators while preserving order and lazy evaluation. Its state transitions can affect a hot loop, but specialization, inlining, and vectorization determine the actual generated code. Separate loops can be worth measuring; eagerly combining collections also adds work and can change evaluation behavior.

## Bad

```rust
// Chain in hot inner loop
fn process_hot_path(a: &[i32], b: &[i32]) -> i64 {
    let mut sum = 0i64;
    
    // Called millions of times
    for _ in 0..1_000_000 {
        for x in a.iter().chain(b.iter()) {  // Branch every iteration
            sum += *x as i64;
        }
    }
    sum
}

// Chaining multiple small slices in tight loop
fn combine_results(parts: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for part in parts {
        for byte in std::iter::once(&0u8).chain(part.iter()) {
            result.push(*byte);
        }
    }
    result
}
```

## Good

```rust
// Separate loops - branch-free inner loops
fn process_hot_path(a: &[i32], b: &[i32]) -> i64 {
    let mut sum = 0i64;
    
    for _ in 0..1_000_000 {
        for x in a {
            sum += *x as i64;
        }
        for x in b {
            sum += *x as i64;
        }
    }
    sum
}

// Pre-combine outside hot loop
fn combine_results(parts: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for part in parts {
        result.push(0u8);
        result.extend_from_slice(part);
    }
    result
}
```

## When Chain Is Fine

Chain is perfectly acceptable when:

```rust
// One-time iteration, not in hot path
fn collect_all(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.into_iter().chain(b).collect()
}

// Lazy evaluation with short-circuit
fn find_in_either<'a>(a: &'a [Item], b: &'a [Item], target: i32) -> Option<&'a Item> {
    a.iter().chain(b.iter()).find(|x| x.id == target)
}

// Small number of elements
fn get_prefixes() -> impl Iterator<Item = &'static str> {
    ["Mr.", "Mrs.", "Dr."].iter().copied()
        .chain(["Prof."].iter().copied())
}
```

## Alternative Patterns

### Pre-allocate and Extend

```rust
fn merge_slices(slices: &[&[i32]]) -> Vec<i32> {
    let total: usize = slices.iter().map(|s| s.len()).sum();
    let mut result = Vec::with_capacity(total);
    for slice in slices {
        result.extend_from_slice(slice);
    }
    result
}
```

### Use append for Vecs

```rust
fn combine_vecs(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    a.append(&mut b);  // Moves elements, no reallocation if a has capacity
    a
}
```

### Flatten Instead of Chain

```rust
// Instead of: a.iter().chain(b.iter()).chain(c.iter())
let all = [a, b, c];
for item in all.iter().flat_map(|slice| slice.iter()) {
    process(item);
}
```

## Performance Impact

| Pattern | Per-Item Overhead |
|---------|-------------------|
| Single iterator | None |
| `chain(a, b)` | Iterator state transitions; optimized cost varies |
| Nested chains | Measure the optimized loop rather than counting source branches |
| Separate loops | Avoids chained state, but may increase code size |

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

## See Also

The [sum and separator regression tests](../perf-validation/tests/iterators.rs) check equivalent results. [Criterion cases](../perf-validation/benches/cases/iterators.rs) measure chained and separate loops with the same inputs; the source-level table is not an assembly guarantee.

- [perf-iter-over-index](./perf-iter-over-index.md) - Prefer iterators
- [perf-extend-batch](./perf-extend-batch.md) - Batch insertions
- [opt-cache-friendly](./opt-cache-friendly.md) - Cache-friendly patterns

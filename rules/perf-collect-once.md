# perf-collect-once

> Don't collect intermediate iterators

## Why It Matters

Intermediate collections can add allocation, traversal, and peak-memory costs. Keep transformations lazy when callers need only the final result, while preserving observable effects and errors. Allocation counts depend on the collection, iterator specialization, and existing storage; not every `collect` performs a fresh heap allocation.

## Bad

```rust
// Multiple collection stages and traversals; actual allocations depend on specialization
fn process_users(users: Vec<User>) -> Vec<String> {
    let active: Vec<_> = users.into_iter()
        .filter(|u| u.is_active)
        .collect();
    
    let verified: Vec<_> = active.into_iter()
        .filter(|u| u.is_verified)
        .collect();
    
    verified.into_iter()
        .map(|u| u.name)
        .collect()
}

// Collecting to count
fn count_valid(items: &[Item]) -> usize {
    items.iter()
        .filter(|i| i.is_valid())
        .collect::<Vec<_>>()  // Unnecessary!
        .len()
}
```

## Good

```rust
// One final collection stage
fn process_users(users: Vec<User>) -> Vec<String> {
    users.into_iter()
        .filter(|u| u.is_active)
        .filter(|u| u.is_verified)
        .map(|u| u.name)
        .collect()
}

// No allocation needed
fn count_valid(items: &[Item]) -> usize {
    items.iter()
        .filter(|i| i.is_valid())
        .count()
}
```

## Pattern: Deferred Collection

```rust
// Create the iterator chain
fn prepare_data(raw: Vec<RawData>) -> impl Iterator<Item = ProcessedData> {
    raw.into_iter()
        .filter(|d| d.is_valid())
        .map(ProcessedData::from)
}

// Collect only when needed
let data: Vec<_> = prepare_data(input).collect();

// Or consume without collecting
prepare_data(input).for_each(|d| process(d));
```

## When Intermediate Collection Is Needed

```rust
// Need to iterate multiple times
let items: Vec<_> = data.iter()
    .filter(|x| x.is_valid())
    .collect();

let count = items.len();
let first = items.first();
for item in &items {
    process(item);
}

// Need to sort (requires concrete collection)
let mut sorted: Vec<_> = data.iter()
    .filter(|x| x.is_active)
    .collect();
sorted.sort_by_key(|x| x.priority);
```

## Comparison

| Approach | Allocations | Passes | Memory |
|----------|-------------|--------|--------|
| Multiple `.collect()` | Implementation dependent | Multiple stages | Intermediate storage can increase the peak |
| Single chain + `.collect()` | Implementation dependent | One traversal pipeline | Final output storage |
| No `.collect()` (streaming) | No intermediate collection required | One traversal pipeline | Depends on operation state |

## Pattern: Collect with Capacity

When you must collect, pre-allocate:

```rust
// With estimated capacity
let mut result = Vec::with_capacity(items.len());
result.extend(
    items.iter()
        .filter(|x| x.is_valid())
        .map(|x| x.clone())
);
```

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

Combining stages can change the order of predicate calls, errors, and side effects. Check their contract as well as returned values; [regression tests](../perf-validation/tests/iterators.rs) demonstrate why lazy and eager effects can differ.

## See Also

- [perf-iter-lazy](./perf-iter-lazy.md) - Keep iterators lazy
- [mem-with-capacity](./mem-with-capacity.md) - Pre-allocate collections
- [anti-collect-intermediate](./anti-collect-intermediate.md) - Anti-pattern

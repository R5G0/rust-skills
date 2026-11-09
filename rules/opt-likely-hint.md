# opt-likely-hint

> Add branch hints only for measured hot paths; distinguish stable cold hints from nightly intrinsics

## Why It Matters

Branch layout can affect hot-path performance, but the benefit depends on the target, compiler, and actual branch distribution. Use profile data before adding optimizer hints. Early returns and match-arm ordering can improve readability without guaranteeing any particular prediction or generated layout.

## Stable Rust: Code Structure Hints

```rust
// Pattern 1: Early returns for unlikely cases
fn process(data: Option<&Data>) -> i32 {
    // An early return makes the main flow clear; it is not a probability hint.
    let data = match data {
        None => return 0,  // Unlikely
        Some(d) => d,
    };
    
    // Hot path continues here
    complex_processing(data)
}

// Pattern 2: if-else ordering
fn calculate(x: i32) -> i32 {
    if x >= 0 {
        // Put the main case here for readability; measure optimizer effects.
        x * 2
    } else {
        // Unlikely case in "else"
        handle_negative(x)
    }
}

// Pattern 3: Cold function extraction
fn hot_path(data: &[u8]) -> Result<(), Error> {
    if data.is_empty() {
        return cold_empty_error();  // Extracted = unlikely
    }
    
    process_fast(data)
}

#[cold]
fn cold_empty_error() -> Result<(), Error> {
    Err(Error::EmptyInput)
}
```

## Nightly: std::hint

```rust
// Requires nightly; still unstable as of Rust 1.97
#![feature(likely_unlikely)]
use std::hint::{likely, unlikely};

fn process(data: &Data) -> i32 {
    if unlikely(data.is_corrupted()) {
        return handle_corruption(data);
    }
    
    if likely(data.is_cached()) {
        return fast_cached_path(data);
    }
    
    slow_uncached_path(data)
}
```

## Boolean Likely Wrapper (Nightly)

```rust
// Requires nightly; still unstable as of Rust 1.97
#![feature(likely_unlikely)]

#[inline(always)]
fn likely(b: bool) -> bool {
    std::hint::likely(b)
}

#[inline(always)]
fn unlikely(b: bool) -> bool {
    std::hint::unlikely(b)
}

// Usage
if likely(x > 0) {
    hot_path(x)
} else {
    cold_path(x)
}
```

## Stable: std::hint::cold_path

`std::hint::cold_path()` (stable since Rust 1.95) marks the enclosing code path as unlikely so the optimizer can deprioritize it — a stable substitute for nightly `unlikely`. Call it inside the rarely-taken branch:

```rust
fn process(data: &Data) -> i32 {
    if data.is_corrupted() {
        std::hint::cold_path(); // tell the optimizer this branch is rare
        return handle_corruption(data);
    }
    fast_path(data)
}
```

## Stable: likely-stable Crate

```rust
use likely_stable::{likely, unlikely};

fn check(value: i32) -> bool {
    if unlikely(value < 0) {
        handle_negative()
    } else if likely(value < 1000) {
        handle_common()
    } else {
        handle_large()
    }
}
```

## Loop Optimization

```rust
fn search(data: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in data.iter().enumerate() {
        // Assume most iterations DON'T find the target
        if unlikely(item == target) {
            return Some(i);
        }
    }
    None
}

// Alternative: structure for likely case
fn search_common(data: &[i32], target: i32) -> Option<usize> {
    // If target is usually found
    for (i, &item) in data.iter().enumerate() {
        if likely(item == target) {
            return Some(i);
        }
    }
    None
}
```

## Match Arm Ordering

```rust
// Order disjoint variants for readability, not as a guaranteed branch hint.
fn process_message(msg: Message) {
    match msg {
        // Main case - listed first for readability
        Message::Data(d) => handle_data(d),
        Message::Heartbeat => (), // Second most common
        
        // Rare cases last
        Message::Error(e) => handle_error(e),
        Message::Shutdown => shutdown(),
    }
}
```

## Benchmark-Driven Hints

```rust
// Profile first to know which branches are actually likely!
fn speculative(x: i32) -> i32 {
    // DON'T GUESS - measure with profiling
    // perf record / perf report
    // cargo flamegraph
    
    if x > threshold {  // Is this actually common?
        path_a(x)
    } else {
        path_b(x)
    }
}
```

## Verified Project

<!-- verify: fixture=optimization -->

The [stable optimization fixture](../checks/fixtures/optimization/src/main.rs) uses `#[cold]` and tests both branches. Nightly examples above require their feature gate; the stable fixture does not claim to validate their generated machine-code layout.

## See Also

- [opt-cold-unlikely](./opt-cold-unlikely.md) - #[cold] for unlikely functions
- [opt-inline-never-cold](./opt-inline-never-cold.md) - Keeping cold code separate
- [perf-profile-first](./perf-profile-first.md) - Profile to know what's likely

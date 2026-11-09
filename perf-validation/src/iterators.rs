//! Executable adaptations of four iterator-oriented `perf-*` rules.
//!
//! These functions preserve the operations in the rule examples. The million
//! repetitions in `perf-chain-avoid` are delegated to Criterion's timing loop.
//! Arithmetic examples require their mathematical result to fit in the return
//! type, as the original examples do; fixture generators keep that invariant.

/// Sum squares with the indexed loop from `perf-iter-over-index`.
#[expect(
    clippy::needless_range_loop,
    reason = "indexed baseline is the experiment"
)]
pub fn sum_squares_indexed(data: &[i32]) -> i64 {
    let mut sum = 0_i64;
    for index in 0..data.len() {
        sum += i64::from(data[index]) * i64::from(data[index]);
    }
    sum
}

/// Sum the same squares with an iterator.
pub fn sum_squares_iterated(data: &[i32]) -> i64 {
    data.iter().map(|&value| i64::from(value).pow(2)).sum()
}

/// Compute an indexed dot product, truncating to the shorter input.
pub fn dot_product_indexed(left: &[f64], right: &[f64]) -> f64 {
    let mut sum = 0.0;
    for index in 0..left.len().min(right.len()) {
        sum += left[index] * right[index];
    }
    sum
}

/// Compute the same truncating dot product with `zip`.
pub fn dot_product_zipped(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(&a, &b)| a * b).sum()
}

/// Double each value with an indexed loop.
#[expect(
    clippy::needless_range_loop,
    reason = "indexed baseline is the experiment"
)]
pub fn double_values_indexed(data: &mut [i32]) {
    for index in 0..data.len() {
        data[index] *= 2;
    }
}

/// Double each value through a mutable iterator.
pub fn double_values_iterated(data: &mut [i32]) {
    for value in data {
        *value *= 2;
    }
}

/// Filter and map eagerly before retaining the first ten positive values.
pub fn process_eager(data: Vec<i32>) -> Vec<i32> {
    process_eager_observed(data, |_| {})
}

/// Retain ten positive values while leaving later input unevaluated.
pub fn process_lazy(data: Vec<i32>) -> Vec<i32> {
    process_lazy_observed(data, |_| {})
}

/// Run the eager transformation and observe each value that is mapped.
pub fn process_eager_observed(data: Vec<i32>, mut on_map: impl FnMut(i32)) -> Vec<i32> {
    let filtered: Vec<_> = data.into_iter().filter(|value| *value > 0).collect();
    let mapped: Vec<_> = filtered
        .into_iter()
        .map(|value| {
            on_map(value);
            value * 2
        })
        .collect();
    mapped.into_iter().take(10).collect()
}

/// Run the lazy transformation and observe only values actually mapped.
pub fn process_lazy_observed(data: Vec<i32>, mut on_map: impl FnMut(i32)) -> Vec<i32> {
    data.into_iter()
        .filter(|value| *value > 0)
        .map(|value| {
            on_map(value);
            value * 2
        })
        .take(10)
        .collect()
}

/// Check existence after collecting every predicate match.
pub fn any_eager(data: &[i32], mut predicate: impl FnMut(i32) -> bool) -> bool {
    let matches: Vec<_> = data.iter().filter(|&&value| predicate(value)).collect();
    !matches.is_empty()
}

/// Check existence by stopping at the first predicate match.
pub fn any_lazy(data: &[i32], mut predicate: impl FnMut(i32) -> bool) -> bool {
    data.iter().any(|&value| predicate(value))
}

/// A concrete domain model for the `perf-collect-once` example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    /// Name moved into the result for matching users.
    pub name: String,
    /// First filter in both implementations.
    pub is_active: bool,
    /// Second filter in both implementations.
    pub is_verified: bool,
}

/// Collect users after each filtering stage, as in the rule's baseline.
pub fn process_users_staged(users: Vec<User>) -> Vec<String> {
    let active: Vec<_> = users.into_iter().filter(|user| user.is_active).collect();
    let verified: Vec<_> = active.into_iter().filter(|user| user.is_verified).collect();
    verified.into_iter().map(|user| user.name).collect()
}

/// Collect only the final list of matching user names.
pub fn process_users_once(users: Vec<User>) -> Vec<String> {
    users
        .into_iter()
        .filter(|user| user.is_active)
        .filter(|user| user.is_verified)
        .map(|user| user.name)
        .collect()
}

/// Count positive values by first collecting references to all matches.
pub fn count_positive_collected(data: &[i32]) -> usize {
    let matches: Vec<_> = data.iter().filter(|&&value| value > 0).collect();
    matches.len()
}

/// Count the same matches without an intermediate collection.
pub fn count_positive_direct(data: &[i32]) -> usize {
    data.iter().filter(|&&value| value > 0).count()
}

/// Sum both inputs with a chained iterator.
pub fn sum_chained(left: &[i32], right: &[i32]) -> i64 {
    let mut sum = 0_i64;
    for value in left.iter().chain(right) {
        sum += i64::from(*value);
    }
    sum
}

/// Sum the same inputs with two separate loops.
pub fn sum_separate(left: &[i32], right: &[i32]) -> i64 {
    let mut sum = 0_i64;
    for value in left {
        sum += i64::from(*value);
    }
    for value in right {
        sum += i64::from(*value);
    }
    sum
}

/// Prefix each part with a zero byte using `once().chain()`.
pub fn combine_parts_chained(parts: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for part in parts {
        for byte in std::iter::once(&0_u8).chain(part.iter()) {
            result.push(*byte);
        }
    }
    result
}

/// Prefix each part with a zero byte using `push()` and slice extension.
pub fn combine_parts_separate(parts: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for part in parts {
        result.push(0);
        result.extend_from_slice(part);
    }
    result
}

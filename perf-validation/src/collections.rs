//! Executable adaptations of the entry, drain, extend, and collection-reuse rules.

use std::collections::HashMap;
use std::num::NonZeroUsize;

/// Count string keys with the rule's repeated-lookup reference implementation.
#[expect(
    clippy::map_entry,
    reason = "retain the rule's repeated-lookup baseline"
)]
pub fn count_keys_reference(keys: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for key in keys {
        let key = key.clone();
        if counts.contains_key(&key) {
            *counts
                .get_mut(&key)
                .expect("a key checked above must exist") += 1;
        } else {
            counts.insert(key, 1);
        }
    }
    counts
}

/// Count the same keys using the map entry API.
pub fn count_keys_entry(keys: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for key in keys {
        *counts.entry(key.clone()).or_insert(0) += 1;
    }
    counts
}

/// Return an entry's value, evaluating its initializer only when vacant.
pub fn get_or_insert_with<F>(
    map: &mut HashMap<String, Vec<i32>>,
    key: String,
    make: F,
) -> &mut Vec<i32>
where
    F: FnOnce() -> Vec<i32>,
{
    map.entry(key).or_insert_with(make)
}

/// Append values using the insertion-loop reference implementation.
pub fn append_push(destination: &mut Vec<i32>, values: &[i32]) {
    for value in values {
        destination.push(*value);
    }
}

/// Append the same values using `Extend` and an exact-size iterator.
pub fn append_extend(destination: &mut Vec<i32>, values: &[i32]) {
    destination.extend(values.iter().copied());
}

/// Merge a map through explicit insertions; later values replace earlier ones.
pub fn merge_maps_reference(destination: &mut HashMap<String, i32>, source: HashMap<String, i32>) {
    for (key, value) in source {
        destination.insert(key, value);
    }
}

/// Merge a map through `Extend`, preserving the same replacement behavior.
pub fn merge_maps_extend(destination: &mut HashMap<String, i32>, source: HashMap<String, i32>) {
    destination.extend(source);
}

/// Drain a bounded prefix into another vector, retaining source capacity.
pub fn drain_prefix_into(source: &mut Vec<i32>, destination: &mut Vec<i32>, amount: usize) {
    let end = amount.min(source.len());
    destination.extend(source.drain(..end));
}

/// Visit batches with a new temporary vector for every prefix drain.
///
/// A nonzero batch size prevents the non-progressing loop possible in the rule's
/// generic `process_in_chunks` example when a caller supplies zero.
pub fn drain_batches_fresh(
    mut data: Vec<i32>,
    chunk_size: NonZeroUsize,
    mut visit: impl FnMut(&[i32]),
) {
    while !data.is_empty() {
        let end = chunk_size.get().min(data.len());
        let batch: Vec<_> = data.drain(..end).collect();
        visit(&batch);
    }
}

/// Visit the same prefix-drained batches with one reusable temporary vector.
///
/// Source compaction still happens on every prefix drain; the benchmark includes
/// that work instead of interpreting buffer reuse as a complexity improvement.
pub fn drain_batches_reused(
    mut data: Vec<i32>,
    chunk_size: NonZeroUsize,
    mut visit: impl FnMut(&[i32]),
) {
    let mut batch = Vec::with_capacity(chunk_size.get().min(data.len()));
    while !data.is_empty() {
        drain_prefix_into(&mut data, &mut batch, chunk_size.get());
        visit(&batch);
        batch.clear();
    }
}

/// Collect the positive values of a batch into a fresh vector.
pub fn filter_batch_fresh(batch: &[i32]) -> Vec<i32> {
    batch.iter().copied().filter(|value| *value > 0).collect()
}

/// Replace a buffer's contents with positive values using stable `Extend`.
pub fn filter_batch_into(batch: &[i32], buffer: &mut Vec<i32>) {
    buffer.clear();
    buffer.extend(batch.iter().copied().filter(|value| *value > 0));
}

/// Visit filtered batches, allocating a separate output for each one.
pub fn visit_filtered_fresh(batches: &[Vec<i32>], mut visit: impl FnMut(&[i32])) {
    for batch in batches {
        let filtered = filter_batch_fresh(batch);
        visit(&filtered);
    }
}

/// Visit filtered batches using one buffer throughout a complete workload.
pub fn visit_filtered_reused(batches: &[Vec<i32>], mut visit: impl FnMut(&[i32])) {
    let mut buffer = Vec::new();
    for batch in batches {
        filter_batch_into(batch, &mut buffer);
        visit(&buffer);
    }
}

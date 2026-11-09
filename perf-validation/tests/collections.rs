//! Entry, batch insertion, drain, and reusable-buffer regression tests.

use proptest::prelude::*;
use rust_skills_perf_validation::collections::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::num::NonZeroUsize;

#[test]
fn perf_entry_api_counts_empty_repeated_and_distinct_keys() {
    assert!(count_keys_entry(&[]).is_empty());
    let keys = ["apple", "pear", "apple", "apple", "pear", "plum"].map(String::from);
    let expected = HashMap::from([
        (String::from("apple"), 3),
        (String::from("pear"), 2),
        (String::from("plum"), 1),
    ]);
    assert_eq!(count_keys_reference(&keys), expected);
    assert_eq!(count_keys_entry(&keys), expected);
}

#[test]
fn perf_entry_api_initializes_only_vacant_entries_and_returns_stored_value() {
    let calls = Cell::new(0);
    let mut map = HashMap::new();
    get_or_insert_with(&mut map, String::from("key"), || {
        calls.set(calls.get() + 1);
        vec![10]
    })
    .push(20);
    get_or_insert_with(&mut map, String::from("key"), || {
        calls.set(calls.get() + 1);
        vec![99]
    })
    .push(30);
    assert_eq!(calls.get(), 1);
    assert_eq!(map["key"], [10, 20, 30]);
}

#[test]
fn perf_extend_batch_preserves_existing_values_order_and_duplicates() {
    let mut actual = vec![7, 7];
    for batch in [&[][..], &[3, 3, 1], &[9], &[]] {
        append_extend(&mut actual, batch);
    }
    assert_eq!(actual, [7, 7, 3, 3, 1, 9]);
}

#[test]
fn perf_extend_batch_map_extension_replaces_colliding_keys() {
    let initial = HashMap::from([(String::from("keep"), 1), (String::from("replace"), 2)]);
    let source = HashMap::from([(String::from("replace"), 20), (String::from("new"), 3)]);
    let mut reference = initial.clone();
    let mut actual = initial;
    merge_maps_reference(&mut reference, source.clone());
    merge_maps_extend(&mut actual, source);
    assert_eq!(actual, reference);
    assert_eq!(actual["keep"], 1);
    assert_eq!(actual["replace"], 20);
    assert_eq!(actual["new"], 3);
}

#[test]
fn perf_drain_reuse_partial_and_full_drains_preserve_values_and_capacity() {
    let mut source = Vec::with_capacity(16);
    source.extend([1, 2, 3, 4, 5]);
    let source_capacity = source.capacity();
    let mut destination = Vec::with_capacity(16);
    destination.push(0);
    let destination_capacity = destination.capacity();

    drain_prefix_into(&mut source, &mut destination, 0);
    assert_eq!(source, [1, 2, 3, 4, 5]);
    assert_eq!(destination, [0]);

    drain_prefix_into(&mut source, &mut destination, 2);
    assert_eq!(source, [3, 4, 5]);
    assert_eq!(destination, [0, 1, 2]);
    assert_eq!(source.capacity(), source_capacity);
    assert_eq!(destination.capacity(), destination_capacity);

    drain_prefix_into(&mut source, &mut destination, usize::MAX);
    assert!(source.is_empty());
    assert_eq!(destination, [0, 1, 2, 3, 4, 5]);
    assert_eq!(source.capacity(), source_capacity);
    assert_eq!(destination.capacity(), destination_capacity);

    destination.clear();
    drain_prefix_into(&mut source, &mut destination, 1);
    assert!(destination.is_empty());
    assert_eq!(destination.capacity(), destination_capacity);
}

#[test]
fn perf_drain_reuse_visits_final_partial_batch_without_stale_items() {
    let chunk_size = NonZeroUsize::new(3).unwrap();
    let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]];
    let mut actual = Vec::new();
    drain_batches_reused((1..=7).collect(), chunk_size, |batch| {
        actual.push(batch.to_vec())
    });
    assert_eq!(actual, expected);
    let mut visits = 0;
    drain_batches_reused(Vec::new(), chunk_size, |_| visits += 1);
    assert_eq!(visits, 0);
}

#[test]
fn perf_collect_into_repeated_batches_clear_old_values_and_keep_reserved_capacity() {
    let mut buffer = Vec::with_capacity(32);
    let capacity = buffer.capacity();
    for (batch, expected) in [
        (&[1, -1, 2, 0][..], &[1, 2][..]),
        (&[][..], &[][..]),
        (&[-2, 0][..], &[][..]),
        (&[9][..], &[9][..]),
    ] {
        filter_batch_into(batch, &mut buffer);
        assert_eq!(buffer, expected);
        assert_eq!(buffer.capacity(), capacity);
    }
}

#[test]
fn perf_collect_into_grows_when_needed_and_preserves_all_matching_items() {
    let mut buffer = Vec::with_capacity(1);
    let batch: Vec<i32> = (1..=100).collect();
    filter_batch_into(&batch, &mut buffer);
    assert_eq!(buffer, batch);
    assert!(buffer.capacity() >= buffer.len());
    filter_batch_into(&[], &mut buffer);
    assert!(buffer.is_empty());
}

proptest! {
    #[test]
    fn perf_entry_api_matches_reference_for_arbitrary_key_streams(keys in prop::collection::vec("[a-z]{0,8}", 0..100)) {
        prop_assert_eq!(count_keys_entry(&keys), count_keys_reference(&keys));
    }

    #[test]
    fn perf_extend_batch_matches_push_for_arbitrary_batches(
        initial in prop::collection::vec(any::<i32>(), 0..64),
        batches in prop::collection::vec(prop::collection::vec(any::<i32>(), 0..64), 0..8),
    ) {
        let mut reference = initial.clone();
        let mut actual = initial;
        for batch in batches {
            append_push(&mut reference, &batch);
            append_extend(&mut actual, &batch);
        }
        prop_assert_eq!(actual, reference);
    }

    #[test]
    fn perf_drain_reuse_matches_fresh_batches_and_original_order(
        data in prop::collection::vec(any::<i32>(), 0..256),
        chunk_size in 1usize..65,
    ) {
        let chunk_size = NonZeroUsize::new(chunk_size).unwrap();
        let expected: Vec<Vec<i32>> = data.chunks(chunk_size.get()).map(<[i32]>::to_vec).collect();
        let mut fresh = Vec::new();
        let mut reused = Vec::new();
        drain_batches_fresh(data.clone(), chunk_size, |batch| fresh.push(batch.to_vec()));
        drain_batches_reused(data, chunk_size, |batch| reused.push(batch.to_vec()));
        prop_assert_eq!(&fresh, &expected);
        prop_assert_eq!(&reused, &expected);
    }

    #[test]
    fn perf_collect_into_matches_fresh_collection_across_arbitrary_batches(
        batches in prop::collection::vec(prop::collection::vec(any::<i32>(), 0..128), 0..16),
    ) {
        let expected: Vec<Vec<i32>> = batches.iter().map(|batch| filter_batch_fresh(batch)).collect();
        let mut fresh = Vec::new();
        let mut reused = Vec::new();
        visit_filtered_fresh(&batches, |batch| fresh.push(batch.to_vec()));
        visit_filtered_reused(&batches, |batch| reused.push(batch.to_vec()));
        prop_assert_eq!(&fresh, &expected);
        prop_assert_eq!(&reused, &expected);
    }
}

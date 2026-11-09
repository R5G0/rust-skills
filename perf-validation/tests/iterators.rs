//! Equivalence and observable-effect tests for iterator transformations.

use proptest::prelude::*;
use rust_skills_perf_validation::iterators::*;

#[test]
fn perf_iter_over_index_handles_empty_single_and_arithmetic_boundaries() {
    assert_eq!(sum_squares_indexed(&[]), 0);
    assert_eq!(sum_squares_iterated(&[]), 0);
    for value in [i32::MIN, -1, 0, 1, i32::MAX] {
        let expected = i64::from(value) * i64::from(value);
        assert_eq!(sum_squares_indexed(&[value]), expected);
        assert_eq!(sum_squares_iterated(&[value]), expected);
    }
}

#[test]
fn perf_iter_over_index_zip_preserves_shorter_input_contract() {
    assert_eq!(dot_product_indexed(&[], &[2.0]), 0.0);
    assert_eq!(dot_product_zipped(&[2.0], &[]), 0.0);
    assert_eq!(dot_product_indexed(&[2.0, 3.0, 100.0], &[4.0, 5.0]), 23.0);
    assert_eq!(dot_product_zipped(&[2.0, 3.0, 100.0], &[4.0, 5.0]), 23.0);
    assert_eq!(dot_product_zipped(&[4.0, 5.0], &[2.0, 3.0, 100.0]), 23.0);
}

#[test]
fn perf_iter_lazy_preserves_first_ten_results_but_reduces_mapping() {
    let mut eager_mapped = Vec::new();
    let mut lazy_mapped = Vec::new();
    let eager = process_eager_observed((-2..=20).collect(), |value| eager_mapped.push(value));
    let lazy = process_lazy_observed((-2..=20).collect(), |value| lazy_mapped.push(value));
    assert_eq!(eager, (1..=10).map(|value| value * 2).collect::<Vec<_>>());
    assert_eq!(lazy, eager);
    assert_eq!(eager_mapped, (1..=20).collect::<Vec<_>>());
    assert_eq!(lazy_mapped, (1..=10).collect::<Vec<_>>());
}

#[test]
fn perf_iter_lazy_skips_a_later_effect_that_the_eager_version_executes() {
    let reject_late_value = |value| assert!(value <= 10, "later mapping effect executed");
    assert_eq!(
        process_lazy_observed((1..=11).collect(), reject_late_value),
        (1..=10).map(|value| value * 2).collect::<Vec<_>>()
    );
    assert!(
        std::panic::catch_unwind(|| {
            process_eager_observed((1..=11).collect(), reject_late_value)
        })
        .is_err()
    );
}

#[test]
fn perf_iter_lazy_any_stops_only_at_the_first_match() {
    for (input, expected, expected_lazy_calls) in [
        (vec![], false, 0),
        (vec![1, 0, -1], true, 1),
        (vec![0, -1, 1], true, 3),
        (vec![0, -1, -2], false, 3),
    ] {
        let mut eager_calls = 0;
        let mut lazy_calls = 0;
        assert_eq!(
            any_eager(&input, |value| {
                eager_calls += 1;
                value > 0
            }),
            expected
        );
        assert_eq!(
            any_lazy(&input, |value| {
                lazy_calls += 1;
                value > 0
            }),
            expected
        );
        assert_eq!(eager_calls, input.len());
        assert_eq!(lazy_calls, expected_lazy_calls);
    }
}

#[test]
fn perf_collect_once_preserves_name_order_duplicates_and_filtering() {
    let user = |name: &str, is_active, is_verified| User {
        name: name.into(),
        is_active,
        is_verified,
    };
    let users = vec![
        user("duplicate", true, true),
        user("inactive", false, true),
        user("unverified", true, false),
        user("duplicate", true, true),
        user("last", true, true),
    ];
    let expected = vec!["duplicate", "duplicate", "last"];
    assert_eq!(process_users_staged(users.clone()), expected);
    assert_eq!(process_users_once(users), expected);
    assert!(process_users_once(Vec::new()).is_empty());
    assert!(process_users_staged(vec![user("none", false, false)]).is_empty());
    assert!(process_users_once(vec![user("none", false, false)]).is_empty());
}

#[test]
fn perf_chain_avoid_preserves_empty_parts_and_separator_positions() {
    let parts: &[&[u8]] = &[&[], &[1, 2], &[], &[0, 3]];
    let expected = vec![0, 0, 1, 2, 0, 0, 0, 3];
    assert_eq!(combine_parts_chained(parts), expected);
    assert_eq!(combine_parts_separate(parts), expected);
    assert!(combine_parts_chained(&[]).is_empty());
    assert!(combine_parts_separate(&[]).is_empty());
    assert_eq!(sum_chained(&[], &[i32::MIN, i32::MAX]), -1);
    assert_eq!(sum_separate(&[i32::MIN, i32::MAX], &[]), -1);
}

proptest! {
    #[test]
    fn perf_iter_over_index_matches_for_bounded_integer_inputs(
        data in prop::collection::vec(-1_000_000_i32..=1_000_000, 0..128),
    ) {
        let expected: i64 = data.iter().map(|&value| i64::from(value).pow(2)).sum();
        prop_assert_eq!(sum_squares_indexed(&data), expected);
        prop_assert_eq!(sum_squares_iterated(&data), expected);
        let mut indexed = data.clone();
        let mut iterated = data.clone();
        double_values_indexed(&mut indexed);
        double_values_iterated(&mut iterated);
        prop_assert_eq!(&indexed, &iterated);
        prop_assert!(indexed.iter().zip(&data).all(|(&actual, &input)| actual == input * 2));
    }

    #[test]
    fn perf_iter_over_index_dot_product_matches_with_unequal_lengths(
        left in prop::collection::vec(-1000_i32..=1000, 0..128),
        right in prop::collection::vec(-1000_i32..=1000, 0..128),
    ) {
        let left: Vec<_> = left.into_iter().map(f64::from).collect();
        let right: Vec<_> = right.into_iter().map(f64::from).collect();
        prop_assert_eq!(dot_product_indexed(&left, &right), dot_product_zipped(&left, &right));
    }

    #[test]
    fn perf_iter_lazy_results_match_for_pure_bounded_mapping(
        data in prop::collection::vec(-1000_i32..=1000, 0..128),
    ) {
        let expected: Vec<_> = data.iter().copied().filter(|value| *value > 0).take(10).map(|value| value * 2).collect();
        prop_assert_eq!(&process_eager(data.clone()), &expected);
        prop_assert_eq!(process_lazy(data), expected);
    }

    #[test]
    fn perf_collect_once_preserves_results_for_generated_users(
        flags in prop::collection::vec((any::<bool>(), any::<bool>(), any::<u8>()), 0..128),
    ) {
        let users: Vec<_> = flags.iter().map(|&(is_active, is_verified, name)| User {
            name: name.to_string(), is_active, is_verified,
        }).collect();
        let expected: Vec<_> = flags.iter().filter(|&&(active, verified, _)| active && verified).map(|&(_, _, name)| name.to_string()).collect();
        prop_assert_eq!(&process_users_staged(users.clone()), &expected);
        prop_assert_eq!(process_users_once(users), expected);
    }

    #[test]
    fn perf_collect_once_count_matches_without_collecting(
        data in prop::collection::vec(any::<i32>(), 0..256),
    ) {
        prop_assert_eq!(count_positive_collected(&data), count_positive_direct(&data));
    }

    #[test]
    fn perf_chain_avoid_preserves_sum_with_unequal_inputs(
        left in prop::collection::vec(any::<i32>(), 0..128),
        right in prop::collection::vec(any::<i32>(), 0..128),
    ) {
        let expected: i64 = left.iter().map(|&value| i64::from(value)).sum::<i64>()
            + right.iter().map(|&value| i64::from(value)).sum::<i64>();
        prop_assert_eq!(sum_chained(&left, &right), expected);
        prop_assert_eq!(sum_separate(&left, &right), expected);
    }

    #[test]
    fn perf_chain_avoid_preserves_concatenation_for_generated_parts(
        parts in prop::collection::vec(prop::collection::vec(any::<u8>(), 0..64), 0..32),
    ) {
        let slices: Vec<_> = parts.iter().map(Vec::as_slice).collect();
        let mut expected = Vec::new();
        for part in &parts {
            expected.push(0);
            expected.extend_from_slice(part);
        }
        prop_assert_eq!(&combine_parts_chained(&slices), &expected);
        prop_assert_eq!(combine_parts_separate(&slices), expected);
    }
}

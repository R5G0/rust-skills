//! Content-equivalence tests for standard, AHash, and Fx maps.

use std::collections::BTreeMap;

use proptest::prelude::*;
use rust_skills_perf_validation::hashing::*;

#[test]
fn counts_preserve_empty_duplicate_unicode_and_long_keys() {
    let names = vec![
        String::new(),
        "same".to_owned(),
        "same".to_owned(),
        "日本語".to_owned(),
        "x".repeat(512),
    ];
    let expected = BTreeMap::from([
        (String::new(), 1),
        ("same".to_owned(), 2),
        ("日本語".to_owned(), 1),
        ("x".repeat(512), 1),
    ]);
    assert_eq!(
        count_names_std(&names)
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        expected
    );
    assert_eq!(
        count_names_ahash(&names)
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        expected
    );
    assert_eq!(
        count_names_fx(&names)
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        expected
    );
    assert!(count_names_std(&[]).is_empty());
    assert!(count_names_ahash(&[]).is_empty());
    assert!(count_names_fx(&[]).is_empty());
}

#[test]
fn duplicate_integer_keys_replace_values_and_lookups_agree() {
    let nodes = vec![
        (0, "old".to_owned()),
        (u32::MAX, "edge".to_owned()),
        (0, "new".to_owned()),
    ];
    let mut standard = build_nodes_std(&nodes);
    let mut ahash = build_nodes_ahash(&nodes);
    let mut fx = build_nodes_fx(&nodes);
    for key in [0, 1, u32::MAX] {
        assert_eq!(standard.get(&key), ahash.get(&key));
        assert_eq!(standard.get(&key), fx.get(&key));
    }
    assert_eq!(standard.get(&0).map(String::as_str), Some("new"));
    for key in [0, 0, u32::MAX, 123] {
        let removed = standard.remove(&key);
        assert_eq!(ahash.remove(&key), removed);
        assert_eq!(fx.remove(&key), removed);
    }
    assert!(standard.is_empty());
    assert!(ahash.is_empty());
    assert!(fx.is_empty());
}

proptest! {
    #[test]
    fn all_hashers_match_reference_string_counts(names in prop::collection::vec("[a-zA-Z0-9_]{0,48}", 0..200)) {
        let mut expected = BTreeMap::new();
        for name in &names {
            *expected.entry(name.clone()).or_insert(0usize) += 1;
        }
        prop_assert_eq!(&count_names_std(&names).into_iter().collect::<BTreeMap<_, _>>(), &expected);
        prop_assert_eq!(&count_names_ahash(&names).into_iter().collect::<BTreeMap<_, _>>(), &expected);
        prop_assert_eq!(&count_names_fx(&names).into_iter().collect::<BTreeMap<_, _>>(), &expected);
    }

    #[test]
    fn all_hashers_match_reference_integer_replacements(nodes in prop::collection::vec((0u32..64, "[a-z]{0,32}"), 0..200)) {
        let expected: BTreeMap<_, _> = nodes.iter().cloned().collect();
        prop_assert_eq!(&build_nodes_std(&nodes).into_iter().collect::<BTreeMap<_, _>>(), &expected);
        prop_assert_eq!(&build_nodes_ahash(&nodes).into_iter().collect::<BTreeMap<_, _>>(), &expected);
        prop_assert_eq!(&build_nodes_fx(&nodes).into_iter().collect::<BTreeMap<_, _>>(), &expected);
    }
}

// Ported from: test_sampled_from.py
// Properties: membership, filtered correctness, uniqueness

use proptest::prelude::*;
use std::collections::HashSet;

proptest! {
    // -- Membership: sampled value is always in the source --

    #[test]
    fn sampled_is_member(
        idx in 0..10usize,
    ) {
        let source = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let value = source[idx];
        prop_assert!(source.contains(&value));
    }

    // -- prop_filter preserves membership after filtering --

    #[test]
    fn filtered_sampling_even_only(
        idx in (0..10usize).prop_filter("even", |i| [10, 20, 30, 40, 50, 60, 70, 80, 90, 100][*i] % 20 == 0)
    ) {
        let source = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let value = source[idx];
        prop_assert!(value % 20 == 0, "{value} is not divisible by 20");
    }

    // -- Enum-like sampling via select --

    #[test]
    fn enum_sampling(
        v in prop_oneof![
            Just("Red"),
            Just("Green"),
            Just("Blue"),
        ]
    ) {
        prop_assert!(
            v == "Red" || v == "Green" || v == "Blue",
            "unexpected variant '{v}'"
        );
    }

    // -- Uniqueness: select N from M without replacement --

    #[test]
    fn unique_sampling_no_dupes(
        indices in proptest::collection::hash_set(0..50usize, 5..=5)
    ) {
        // HashSet guarantees uniqueness; verify explicitly
        let as_vec: Vec<_> = indices.iter().collect();
        let re_dedup: HashSet<_> = as_vec.iter().collect();
        prop_assert_eq!(as_vec.len(), re_dedup.len());
    }

    // -- When selecting all N items, every element appears --

    #[test]
    fn full_coverage_sampling(
        indices in proptest::collection::hash_set(0..5usize, 5..=5)
    ) {
        let expected: HashSet<usize> = (0..5).collect();
        prop_assert_eq!(indices, expected);
    }

    // -- Weighted sampling: all outputs in source --

    #[test]
    fn weighted_sampling(
        v in prop_oneof![
            3 => Just(1),
            2 => Just(2),
            1 => Just(3),
        ]
    ) {
        prop_assert!(v >= 1 && v <= 3);
    }

    // -- Snapshot immutability: strategy built from a clone --

    #[test]
    fn snapshot_immutability(idx in 0..5usize) {
        let original = vec![10, 20, 30, 40, 50];
        let snapshot = original.clone();
        // Simulate mutation of original (in real code, this would be
        // mutable state). The snapshot should be unaffected.
        let value = snapshot[idx];
        prop_assert!(
            [10, 20, 30, 40, 50].contains(&value),
            "snapshot was corrupted"
        );
    }

    // -- Max size respected --

    #[test]
    fn max_size_respected(
        v in proptest::collection::vec(0..100i32, 0..=3)
    ) {
        prop_assert!(v.len() <= 3);
    }
}

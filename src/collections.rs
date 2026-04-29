// Ported from: test_simple_collections.py
// Properties: size bounds, uniqueness, uniqueness-by-key

use proptest::prelude::*;
use std::collections::HashSet;

proptest! {
    // -- Size bounds --

    #[test]
    fn vec_respects_size_bounds(
        v in proptest::collection::vec(any::<i32>(), 2..=10)
    ) {
        prop_assert!(v.len() >= 2);
        prop_assert!(v.len() <= 10);
    }

    #[test]
    fn vec_exact_size(
        v in proptest::collection::vec(any::<i32>(), 5..=5)
    ) {
        prop_assert_eq!(v.len(), 5);
    }

    #[test]
    fn hashset_respects_size_bounds(
        s in proptest::collection::hash_set(0..100i32, 2..=10)
    ) {
        prop_assert!(s.len() >= 2);
        prop_assert!(s.len() <= 10);
    }

    #[test]
    fn hashmap_respects_size_bounds(
        m in proptest::collection::hash_map(0..100i32, any::<u8>(), 2..=10)
    ) {
        prop_assert!(m.len() >= 2);
        prop_assert!(m.len() <= 10);
    }

    #[test]
    fn hashmap_exact_size(
        m in proptest::collection::hash_map(0..1000i32, any::<u8>(), 5..=5)
    ) {
        prop_assert_eq!(m.len(), 5);
    }

    // -- Uniqueness --
    // HashSet guarantees no duplicates by construction; verify explicitly

    #[test]
    fn hashset_no_duplicates(
        s in proptest::collection::hash_set(any::<i32>(), 0..=20)
    ) {
        let as_vec: Vec<_> = s.iter().collect();
        let deduped: HashSet<_> = as_vec.iter().collect();
        prop_assert_eq!(as_vec.len(), deduped.len());
    }

    // -- Uniqueness by key --
    // Generate a vec then assert uniqueness on a projected key

    #[test]
    fn vec_unique_by_key_mod10(
        v in proptest::collection::vec(0..100i32, 1..=10)
            .prop_filter("unique mod 10", |v| {
                let keys: HashSet<_> = v.iter().map(|x| x % 10).collect();
                keys.len() == v.len()
            })
    ) {
        let keys: HashSet<_> = v.iter().map(|x| x % 10).collect();
        prop_assert_eq!(keys.len(), v.len(), "duplicate mod-10 keys found");
    }

    // -- HashMap keys are unique by construction --

    #[test]
    fn hashmap_keys_unique(
        m in proptest::collection::hash_map(any::<u16>(), any::<u8>(), 0..=20)
    ) {
        let keys: Vec<_> = m.keys().collect();
        let unique: HashSet<_> = keys.iter().collect();
        prop_assert_eq!(keys.len(), unique.len());
    }

    // -- Empty collections --

    #[test]
    fn empty_vec_allowed(
        v in proptest::collection::vec(any::<i32>(), 0..=0)
    ) {
        prop_assert!(v.is_empty());
    }

    #[test]
    fn empty_hashset_allowed(
        s in proptest::collection::hash_set(any::<i32>(), 0..=0)
    ) {
        prop_assert!(s.is_empty());
    }

    // -- Nested collections --

    #[test]
    fn vec_of_vecs_size(
        vv in proptest::collection::vec(
            proptest::collection::vec(any::<u8>(), 0..=3),
            1..=5,
        )
    ) {
        prop_assert!(!vv.is_empty() && vv.len() <= 5);
        for inner in &vv {
            prop_assert!(inner.len() <= 3);
        }
    }
}

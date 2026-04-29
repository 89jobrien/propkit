// Ported from: test_flatmap.py, test_one_of.py (nocover + cover)
// Properties: flatmap/prop_flat_map composition, ordered pairs, constant
// lists, union strategies

use proptest::prelude::*;

proptest! {
    // -- Ordered pairs via flat_map --
    // From: test_in_order (flatmap)

    #[test]
    fn ordered_pair(
        (a, b) in (0i64..1000i64).prop_flat_map(|a| (Just(a), (a + 1)..=1000i64))
    ) {
        prop_assert!(a < b, "{a} >= {b}");
    }

    // -- Constant list: flat_map to repeat a value --
    // From: test_constant_lists_are_constant

    #[test]
    fn constant_list(
        v in any::<u8>().prop_flat_map(|val| {
            proptest::collection::vec(Just(val), 1..=10)
        })
    ) {
        let first = v[0];
        prop_assert!(v.iter().all(|&x| x == first), "not all elements equal");
    }

    // -- Union: all branches satisfy a shared postcondition --
    // From: test_one_of.py filtered union

    #[test]
    fn union_postcondition(
        x in prop_oneof![0..10i32, 100..110i32, 1000..1010i32]
    ) {
        prop_assert!(
            (0..10).contains(&x)
                || (100..110).contains(&x)
                || (1000..1010).contains(&x),
            "unexpected value {x}"
        );
    }

    // -- Flat map preserves bounds --

    #[test]
    fn flatmap_preserves_bounds(
        v in (1usize..=5).prop_flat_map(|n| {
            proptest::collection::vec(any::<u8>(), n..=n)
        })
    ) {
        prop_assert!(!v.is_empty() && v.len() <= 5);
    }

    // -- Chained maps: post-map values satisfy transformed predicate --

    #[test]
    fn chained_map(
        x in (0..100i32).prop_map(|x| x * 2)
    ) {
        prop_assert!(x % 2 == 0, "{x} is not even");
        prop_assert!((0..=198).contains(&x));
    }

    // -- Filter + map composition --

    #[test]
    fn filter_then_map(
        x in (0..100i32)
            .prop_filter("odd only", |x| x % 2 == 1)
            .prop_map(|x| x * 3)
    ) {
        prop_assert!(x % 3 == 0, "{x} not divisible by 3");
        prop_assert!(x % 2 == 1, "{x} is even (should be odd * 3)");
    }

    // -- Nested flat_map --

    #[test]
    fn nested_flatmap(
        v in (1usize..=3).prop_flat_map(|n| {
            proptest::collection::vec(
                (0u8..=9).prop_flat_map(|d| Just(d + b'0')),
                n..=n,
            )
        })
    ) {
        prop_assert!(!v.is_empty() && v.len() <= 3);
        prop_assert!(v.iter().all(|&b| b.is_ascii_digit()));
    }

    // -- Dependent pair: second value bounded by first --

    #[test]
    fn dependent_pair(
        (lo, hi) in (0u32..100).prop_flat_map(|lo| {
            (Just(lo), lo..=100u32)
        })
    ) {
        prop_assert!(lo <= hi);
    }
}

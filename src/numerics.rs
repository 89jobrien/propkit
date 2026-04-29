// Ported from: test_numerics.py
// Properties: integer/float bounds, commutativity, exclusion

use proptest::prelude::*;

proptest! {
    // -- Integer bounds --

    #[test]
    fn i32_in_range(x in -1000i32..=1000i32) {
        prop_assert!((-1000..=1000).contains(&x));
    }

    #[test]
    fn u64_in_range(x in 100u64..=999u64) {
        prop_assert!((100..=999).contains(&x));
    }

    #[test]
    fn i128_in_range(x in i128::MIN..=i128::MAX) {
        // Just verify no panic on the full range
        let _ = x;
    }

    // -- Integer addition commutativity --
    // From: test_fraction_addition_is_well_behaved (x + y + z == y + x + z)

    #[test]
    fn addition_commutative_i64(x in any::<i32>(), y in any::<i32>()) {
        // Use i64 to avoid overflow
        let x = x as i64;
        let y = y as i64;
        prop_assert_eq!(x + y, y + x);
    }

    #[test]
    fn addition_associative_i64(
        x in -1000i64..=1000i64,
        y in -1000i64..=1000i64,
        z in -1000i64..=1000i64,
    ) {
        prop_assert_eq!((x + y) + z, x + (y + z));
    }

    // -- Multiplication commutativity --

    #[test]
    fn multiplication_commutative(x in any::<i32>(), y in any::<i32>()) {
        let x = x as i64;
        let y = y as i64;
        prop_assert_eq!(x * y, y * x);
    }

    // -- Float bounds (echoing floats module but for numeric properties) --

    #[test]
    fn f64_bounds(x in -100.0f64..=100.0f64) {
        prop_assert!((-100.0..=100.0).contains(&x));
    }

    // -- Excluded endpoints via filter --
    // From: test_can_exclude_endpoints

    #[test]
    fn exclude_lower_bound(
        x in (0i32..=100i32).prop_filter("exclude 0", |x| *x != 0)
    ) {
        prop_assert!(x > 0);
    }

    #[test]
    fn exclude_upper_bound(
        x in (0i32..=100i32).prop_filter("exclude 100", |x| *x != 100)
    ) {
        prop_assert!(x < 100);
    }

    // -- Wrapping arithmetic does not panic --

    #[test]
    fn wrapping_add_no_panic(x in any::<u32>(), y in any::<u32>()) {
        let _ = x.wrapping_add(y);
    }

    #[test]
    fn wrapping_mul_no_panic(x in any::<u32>(), y in any::<u32>()) {
        let _ = x.wrapping_mul(y);
    }

    // -- Saturating arithmetic bounds --

    #[test]
    fn saturating_add_bounded(x in any::<u8>(), y in any::<u8>()) {
        let result = x.saturating_add(y);
        prop_assert!(result >= x || result >= y);
    }

    // -- Division properties --

    #[test]
    fn division_rounds_toward_zero(x in any::<i32>(), y in 1..=1000i32) {
        let q = x / y;
        let r = x % y;
        prop_assert_eq!(q * y + r, x);
    }

    // -- Abs is non-negative (except i*::MIN) --

    #[test]
    fn abs_non_negative(x in (i32::MIN + 1)..=i32::MAX) {
        prop_assert!(x.abs() >= 0);
    }
}

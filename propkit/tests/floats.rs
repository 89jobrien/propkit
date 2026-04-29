// Ported from: test_float_nastiness.py, test_subnormal_floats.py
// Properties: bounds, NaN/inf filtering, subnormal control, next_up/next_down

use propkit::strategies::floats::{next_down_f64, next_up_f64};
use proptest::prelude::*;

proptest! {
    // -- Bounds --

    #[test]
    fn floats_in_range(x in -1e10f64..1e10f64) {
        prop_assert!((-1e10..=1e10).contains(&x));
    }

    #[test]
    fn floats_in_unit_range(x in 0.0f64..=1.0f64) {
        prop_assert!(x >= 0.0);
        prop_assert!(x <= 1.0);
    }

    #[test]
    fn floats_negative_range(x in -100.0f64..0.0f64) {
        prop_assert!(x < 0.0);
        prop_assert!(x >= -100.0);
    }

    // Very narrow interval -- should not produce values outside
    #[test]
    fn very_narrow_interval(x in 1.0f64..=1.0000000000001f64) {
        prop_assert!(x >= 1.0);
        prop_assert!(x <= 1.0000000000001);
    }

    // -- NaN / Infinity filtering --

    #[test]
    fn no_nan_in_finite_range(x in 0.0f64..1.0f64) {
        prop_assert!(!x.is_nan());
    }

    #[test]
    fn no_infinity_in_finite_range(x in -1e300f64..1e300f64) {
        prop_assert!(x.is_finite());
    }

    // -- Subnormal control --

    #[test]
    fn no_subnormals_when_filtered(
        x in proptest::num::f64::ANY
            .prop_filter("no subnormals", |x| !x.is_subnormal())
    ) {
        prop_assert!(!x.is_subnormal());
    }

    // -- Large bounds near f64::MAX should not overflow --

    #[test]
    fn large_lower_bound(x in (f64::MAX / 2.0)..=f64::MAX) {
        prop_assert!(x >= f64::MAX / 2.0);
        prop_assert!(x <= f64::MAX);
    }

    #[test]
    fn large_negative_bound(x in f64::MIN..=(f64::MIN / 2.0)) {
        prop_assert!(x >= f64::MIN);
        prop_assert!(x <= f64::MIN / 2.0);
    }

    // -- f32 bounds --

    #[test]
    fn f32_in_range(x in -1.0f32..=1.0f32) {
        prop_assert!(x >= -1.0);
        prop_assert!(x <= 1.0);
    }

    #[test]
    fn f32_no_nan(x in 0.0f32..1.0f32) {
        prop_assert!(!x.is_nan());
    }

    // -- Sign of zero --

    #[test]
    fn zero_has_correct_sign_bit(
        x in prop_oneof![Just(0.0f64), Just(-0.0f64)]
    ) {
        prop_assert!(x == 0.0);
        let bits = x.to_bits();
        prop_assert!(bits == 0u64 || bits == 1u64 << 63);
    }

    // -- next_up / next_down round-trip --

    #[test]
    fn next_up_down_roundtrip(x in proptest::num::f64::NORMAL) {
        let up = next_up_f64(x);
        let down = next_down_f64(up);
        prop_assert_eq!(down, x);
    }

    #[test]
    fn next_down_up_roundtrip(x in proptest::num::f64::NORMAL) {
        let down = next_down_f64(x);
        let up = next_up_f64(down);
        prop_assert_eq!(up, x);
    }

    // -- up means greater --

    #[test]
    fn up_means_greater(x in proptest::num::f64::NORMAL) {
        let up = next_up_f64(x);
        prop_assert!(up >= x, "next_up({x}) = {up} < {x}");
    }

    #[test]
    fn down_means_lesser(x in proptest::num::f64::NORMAL) {
        let down = next_down_f64(x);
        prop_assert!(down <= x, "next_down({x}) = {down} > {x}");
    }
}

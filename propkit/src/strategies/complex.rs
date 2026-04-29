// propkit::strategies::complex -- complex number strategies
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use num_complex::Complex64;
use proptest::prelude::*;

/// Arbitrary complex number with finite components.
pub fn arb_complex() -> impl Strategy<Value = Complex64> {
    (proptest::num::f64::NORMAL, proptest::num::f64::NORMAL)
        .prop_map(|(re, im)| Complex64::new(re, im))
}

/// Arbitrary complex number with components in `[-bound, bound]`.
pub fn arb_complex_bounded(bound: f64) -> impl Strategy<Value = Complex64> {
    (-bound..=bound, -bound..=bound).prop_map(|(re, im)| Complex64::new(re, im))
}

/// Arbitrary complex with no subnormal components.
pub fn arb_complex_no_subnormal() -> impl Strategy<Value = Complex64> {
    (
        proptest::num::f64::ANY.prop_filter("no subnormal re", |x| !x.is_subnormal()),
        proptest::num::f64::ANY.prop_filter("no subnormal im", |x| !x.is_subnormal()),
    )
        .prop_map(|(re, im)| Complex64::new(re, im))
}

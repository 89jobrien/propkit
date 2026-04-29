// propkit::strategies::floats — stable-Rust next_up/next_down via bit manipulation
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Returns the least `f64` greater than `x`.
///
/// Stable-Rust equivalent of the nightly `f64::next_up()` method,
/// implemented via bit manipulation.
pub fn next_up_f64(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x == f64::NEG_INFINITY {
        return f64::MIN;
    }
    if x == 0.0 && x.is_sign_negative() {
        return 0.0;
    }
    let bits = x.to_bits();
    if x >= 0.0 {
        f64::from_bits(bits + 1)
    } else {
        f64::from_bits(bits - 1)
    }
}

/// Returns the greatest `f64` less than `x`.
///
/// Stable-Rust equivalent of the nightly `f64::next_down()` method,
/// implemented via bit manipulation.
pub fn next_down_f64(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x == f64::INFINITY {
        return f64::MAX;
    }
    if x == 0.0 && !x.is_sign_negative() {
        return -0.0;
    }
    let bits = x.to_bits();
    if x > 0.0 {
        f64::from_bits(bits - 1)
    } else {
        f64::from_bits(bits + 1)
    }
}

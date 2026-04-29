// propkit::complex -- property tests for complex numbers
// Ported from Hypothesis test_complex_numbers.py

use num_complex::Complex64;
use propkit::strategies::complex::{arb_complex, arb_complex_bounded, arb_complex_no_subnormal};
use proptest::prelude::*;

proptest! {
    // -- Magnitude --

    #[test]
    fn magnitude_non_negative(z in arb_complex()) {
        prop_assert!(z.norm() >= 0.0);
    }

    #[test]
    fn bounded_magnitude(z in arb_complex_bounded(10.0)) {
        // |z| <= sqrt(bound^2 + bound^2) = bound * sqrt(2)
        prop_assert!(z.norm() <= 10.0 * std::f64::consts::SQRT_2 + 1e-9);
    }

    #[test]
    fn bounded_real_part(z in arb_complex_bounded(5.0)) {
        prop_assert!(z.re >= -5.0 && z.re <= 5.0);
    }

    #[test]
    fn bounded_imag_part(z in arb_complex_bounded(5.0)) {
        prop_assert!(z.im >= -5.0 && z.im <= 5.0);
    }

    // -- Conjugate --

    #[test]
    fn conjugate_roundtrip(z in arb_complex()) {
        let roundtrip = z.conj().conj();
        prop_assert_eq!(roundtrip, z);
    }

    #[test]
    fn conjugate_real_unchanged(z in arb_complex()) {
        prop_assert_eq!(z.conj().re, z.re);
    }

    #[test]
    fn conjugate_flips_imaginary(z in arb_complex()) {
        prop_assert_eq!(z.conj().im, -z.im);
    }

    // -- Arithmetic identities --

    #[test]
    fn addition_commutative(
        a in arb_complex_bounded(1e10),
        b in arb_complex_bounded(1e10),
    ) {
        let sum_ab = a + b;
        let sum_ba = b + a;
        prop_assert_eq!(sum_ab, sum_ba);
    }

    #[test]
    fn addition_associative(
        a in arb_complex_bounded(1e8),
        b in arb_complex_bounded(1e8),
        c in arb_complex_bounded(1e8),
    ) {
        let lhs = (a + b) + c;
        let rhs = a + (b + c);
        let tol = 1e-6 * (a.norm() + b.norm() + c.norm()).max(1e-15);
        prop_assert!((lhs - rhs).norm() < tol,
            "associativity: |{lhs} - {rhs}| = {} >= {tol}", (lhs - rhs).norm());
    }

    #[test]
    fn zero_is_additive_identity(z in arb_complex()) {
        let zero = Complex64::new(0.0, 0.0);
        prop_assert_eq!(z + zero, z);
    }

    #[test]
    fn multiplication_by_conjugate_is_real(z in arb_complex_bounded(1e150)) {
        let product = z * z.conj();
        prop_assert!(product.im.abs() < 1e-10 * z.norm().powi(2).max(1e-15),
            "z*conj(z) imaginary part {} not near zero for z={z}", product.im);
    }

    // -- Triangle inequality --

    #[test]
    fn triangle_inequality(
        a in arb_complex_bounded(1e10),
        b in arb_complex_bounded(1e10),
    ) {
        let sum_norm = (a + b).norm();
        let norm_sum = a.norm() + b.norm();
        prop_assert!(sum_norm <= norm_sum + 1e-9,
            "|a+b| = {sum_norm} > |a|+|b| = {norm_sum}");
    }

    // -- Magnitude of product --

    #[test]
    fn magnitude_of_product(
        a in arb_complex_bounded(1e8),
        b in arb_complex_bounded(1e8),
    ) {
        let prod_norm = (a * b).norm();
        let norm_prod = a.norm() * b.norm();
        let tol = 1e-6 * norm_prod.max(1e-15);
        prop_assert!((prod_norm - norm_prod).abs() < tol,
            "|a*b| = {prod_norm} vs |a|*|b| = {norm_prod}");
    }

    // -- Subnormal filtering --

    #[test]
    fn no_subnormal_components(z in arb_complex_no_subnormal()) {
        prop_assert!(!z.re.is_subnormal(), "real part is subnormal: {}", z.re);
        prop_assert!(!z.im.is_subnormal(), "imag part is subnormal: {}", z.im);
    }
}

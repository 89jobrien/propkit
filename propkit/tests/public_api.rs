//! Red-phase tests: verify the public strategy API compiles and works.
//! These tests require `--features strategies` to compile.

#![cfg(feature = "strategies")]

use proptest::prelude::*;

// -- floats --

#[test]
fn floats_next_up_is_pub() {
    let x = 1.0_f64;
    let up = propkit::strategies::floats::next_up_f64(x);
    assert!(up > x);
}

#[test]
fn floats_next_down_is_pub() {
    let x = 1.0_f64;
    let down = propkit::strategies::floats::next_down_f64(x);
    assert!(down < x);
}

// -- permutations --

proptest! {
    #[test]
    fn permutations_arb_is_pub(perm in propkit::strategies::permutations::arb_permutation(5)) {
        prop_assert_eq!(perm.len(), 5);
    }
}

// -- slices --

#[test]
fn slices_types_are_pub() {
    let slice = propkit::strategies::slices::Slice {
        start: Some(0),
        stop: Some(3),
        step: None,
    };
    let data = vec![1, 2, 3, 4, 5];
    let result = slice.apply(&data);
    assert_eq!(result, vec![1, 2, 3]);
}

proptest! {
    #[test]
    fn slices_arb_is_pub(slice in propkit::strategies::slices::arb_slice(5)) {
        let data = vec![1u8, 2, 3, 4, 5];
        let _ = slice.apply(&data);
    }
}

// -- recursive --

#[test]
fn recursive_types_are_pub() {
    let leaf = propkit::strategies::recursive::Tree::Leaf(42);
    assert!(matches!(
        leaf,
        propkit::strategies::recursive::Tree::Leaf(42)
    ));
}

proptest! {
    #[test]
    fn recursive_arb_tree_is_pub(tree in propkit::strategies::recursive::arb_tree()) {
        // Just verify it generates without panic
        let _ = &tree;
    }

    #[test]
    fn recursive_arb_leaf_only_is_pub(tree in propkit::strategies::recursive::arb_leaf_only()) {
        prop_assert!(matches!(tree, propkit::strategies::recursive::Tree::Leaf(_)));
    }
}

// Ported from: test_recursive.py (Hypothesis cover tests)
// Properties: recursive/tree strategies via prop_recursive

use propkit::strategies::recursive::{Tree, arb_leaf_only, arb_tree};
use proptest::prelude::*;

fn count_leaves(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf(_) => 1,
        Tree::Branch(children) => children.iter().map(count_leaves).sum(),
    }
}

fn depth(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf(_) => 0,
        Tree::Branch(children) => 1 + children.iter().map(depth).max().unwrap_or(0),
    }
}

fn flatten(tree: &Tree) -> Vec<i32> {
    match tree {
        Tree::Leaf(v) => vec![*v],
        Tree::Branch(children) => children.iter().flat_map(flatten).collect(),
    }
}

proptest! {
    #[test]
    fn leaf_count_bounded(tree in arb_tree()) {
        prop_assert!(
            count_leaves(&tree) <= 64,
            "leaf count {} exceeds max_nodes=64",
            count_leaves(&tree)
        );
    }

    #[test]
    fn all_leaves_are_i32(tree in arb_tree()) {
        let leaves = flatten(&tree);
        for v in &leaves {
            prop_assert!((i32::MIN..=i32::MAX).contains(v));
        }
    }

    #[test]
    fn flatten_nonempty(tree in arb_tree()) {
        let leaves = flatten(&tree);
        prop_assert!(!leaves.is_empty(), "flatten returned empty vec");
    }

    #[test]
    fn tree_depth_bounded(tree in arb_tree()) {
        let d = depth(&tree);
        prop_assert!(d <= 5, "depth {d} exceeded expected bound of 5");
    }

    #[test]
    fn leaf_only_is_always_leaf(tree in arb_leaf_only()) {
        prop_assert!(
            matches!(tree, Tree::Leaf(_)),
            "expected Leaf, got Branch"
        );
    }

    #[test]
    fn single_branch_one_leaf(x in any::<i32>()) {
        let tree = Tree::Branch(vec![Tree::Leaf(x)]);
        let leaves = flatten(&tree);
        prop_assert_eq!(leaves.len(), 1, "expected 1 leaf, got {}", leaves.len());
        prop_assert_eq!(leaves[0], x);
    }

    #[test]
    fn leaf_count_equals_flatten_len(tree in arb_tree()) {
        prop_assert_eq!(
            count_leaves(&tree),
            flatten(&tree).len(),
            "count_leaves and flatten length disagree"
        );
    }
}

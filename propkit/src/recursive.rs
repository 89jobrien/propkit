// Ported from: test_recursive.py (Hypothesis cover tests)
// Properties: recursive/tree strategies via prop_recursive

use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Tree type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Tree {
    Leaf(i32),
    Branch(Vec<Tree>),
}

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

// ---------------------------------------------------------------------------
// Strategy helpers
// ---------------------------------------------------------------------------

/// Tree with max_depth=4, max_nodes=64, branching up to 4 children.
fn arb_tree() -> impl Strategy<Value = Tree> {
    let leaf = any::<i32>().prop_map(Tree::Leaf);
    leaf.prop_recursive(4, 64, 4, |inner| {
        proptest::collection::vec(inner, 1..=4).prop_map(Tree::Branch)
    })
}

/// Depth-0 tree: always a leaf.
fn arb_leaf_only() -> impl Strategy<Value = Tree> {
    any::<i32>().prop_map(Tree::Leaf)
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

proptest! {
    // -- Leaf count respects max_nodes bound --
    // prop_recursive(depth, max_nodes, desired_size, ...) guarantees the total
    // number of generated nodes never exceeds max_nodes.
    #[test]
    fn leaf_count_bounded(tree in arb_tree()) {
        prop_assert!(
            count_leaves(&tree) <= 64,
            "leaf count {} exceeds max_nodes=64",
            count_leaves(&tree)
        );
    }

    // -- All leaves carry valid i32 values --
    // Every value collected from flatten() must be a finite, non-wrapping i32.
    // This is trivially true for Rust's i32 but mirrors Hypothesis's "all
    // elements are from the base strategy" property.
    #[test]
    fn all_leaves_are_i32(tree in arb_tree()) {
        let leaves = flatten(&tree);
        for v in &leaves {
            // i32::MIN and i32::MAX are valid; just confirm the type compiles
            prop_assert!((i32::MIN..=i32::MAX).contains(v));
        }
    }

    // -- Flattening a tree always yields a non-empty vec --
    // Every tree has at least one leaf.
    #[test]
    fn flatten_nonempty(tree in arb_tree()) {
        let leaves = flatten(&tree);
        prop_assert!(!leaves.is_empty(), "flatten returned empty vec");
    }

    // -- Tree depth is bounded --
    // prop_recursive depth parameter is 4, so structural depth is bounded.
    // Add 1 for the root Branch wrapper.
    #[test]
    fn tree_depth_bounded(tree in arb_tree()) {
        let d = depth(&tree);
        prop_assert!(d <= 5, "depth {d} exceeded expected bound of 5");
    }

    // -- Depth-0 strategy always produces a leaf --
    // When the base strategy is used directly (no recursion), result is Leaf.
    #[test]
    fn leaf_only_is_always_leaf(tree in arb_leaf_only()) {
        prop_assert!(
            matches!(tree, Tree::Leaf(_)),
            "expected Leaf, got Branch"
        );
    }

    // -- Single-branch tree has exactly one leaf --
    // Branch(vec![Leaf(x)]) must flatten to exactly one element.
    #[test]
    fn single_branch_one_leaf(x in any::<i32>()) {
        let tree = Tree::Branch(vec![Tree::Leaf(x)]);
        let leaves = flatten(&tree);
        prop_assert_eq!(leaves.len(), 1, "expected 1 leaf, got {}", leaves.len());
        prop_assert_eq!(leaves[0], x);
    }

    // -- Leaf count equals flatten length --
    // count_leaves and flatten().len() must agree.
    #[test]
    fn leaf_count_equals_flatten_len(tree in arb_tree()) {
        prop_assert_eq!(
            count_leaves(&tree),
            flatten(&tree).len(),
            "count_leaves and flatten length disagree"
        );
    }
}

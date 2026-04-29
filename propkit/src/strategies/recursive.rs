// propkit::strategies::recursive — tree type and recursive strategies
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;

/// A simple recursive tree for property testing.
#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(i32),
    Branch(Vec<Tree>),
}

/// Tree with max_depth=4, max_nodes=64, branching up to 4 children.
pub fn arb_tree() -> impl Strategy<Value = Tree> {
    let leaf = any::<i32>().prop_map(Tree::Leaf);
    leaf.prop_recursive(4, 64, 4, |inner| {
        proptest::collection::vec(inner, 1..=4).prop_map(Tree::Branch)
    })
}

/// Depth-0 tree: always a leaf.
pub fn arb_leaf_only() -> impl Strategy<Value = Tree> {
    any::<i32>().prop_map(Tree::Leaf)
}

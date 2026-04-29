// propkit::strategies::permutations — random permutation strategy
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;

/// Generate a random permutation of `0..n` by sorting random keys.
pub fn arb_permutation(n: usize) -> impl Strategy<Value = Vec<usize>> {
    proptest::collection::vec(any::<u32>(), n..=n).prop_map(move |rand_keys| {
        let mut indexed: Vec<(u32, usize)> = rand_keys.into_iter().zip(0..n).collect();
        indexed.sort_by_key(|(k, _)| *k);
        indexed.into_iter().map(|(_, i)| i).collect()
    })
}

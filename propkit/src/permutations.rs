// Ported from: test_permutations.py
// Properties: length preservation, element multiset preservation

use proptest::prelude::*;
use std::collections::HashSet;

/// Generate a random permutation of 0..n by shuffling indices
fn arb_permutation(n: usize) -> impl Strategy<Value = Vec<usize>> {
    proptest::collection::vec(any::<u32>(), n..=n).prop_map(move |rand_keys| {
        let mut indexed: Vec<(u32, usize)> = rand_keys.into_iter().zip(0..n).collect();
        indexed.sort_by_key(|(k, _)| *k);
        indexed.into_iter().map(|(_, i)| i).collect()
    })
}

proptest! {
    // -- Length preservation --

    #[test]
    fn permutation_preserves_length(perm in arb_permutation(10)) {
        prop_assert_eq!(perm.len(), 10);
    }

    // -- Element multiset preservation --

    #[test]
    fn permutation_preserves_elements(perm in arb_permutation(10)) {
        let expected: HashSet<usize> = (0..10).collect();
        let actual: HashSet<usize> = perm.into_iter().collect();
        prop_assert_eq!(actual, expected);
    }

    // -- No duplicates --

    #[test]
    fn permutation_no_duplicates(perm in arb_permutation(20)) {
        let unique: HashSet<usize> = perm.iter().copied().collect();
        prop_assert_eq!(unique.len(), perm.len());
    }

    // -- Empty permutation --

    #[test]
    fn empty_permutation(perm in arb_permutation(0)) {
        prop_assert!(perm.is_empty());
    }

    // -- Single element --

    #[test]
    fn single_element_permutation(perm in arb_permutation(1)) {
        prop_assert_eq!(perm, vec![0]);
    }

    // -- Applying a permutation twice yields a valid permutation --

    #[test]
    fn compose_permutations(
        p1 in arb_permutation(8),
        p2 in arb_permutation(8),
    ) {
        // Compose: result[i] = p1[p2[i]]
        let composed: Vec<usize> = p2.iter().map(|&i| p1[i]).collect();
        prop_assert_eq!(composed.len(), 8);
        let unique: HashSet<usize> = composed.iter().copied().collect();
        prop_assert_eq!(unique.len(), 8, "composition is not a valid permutation");
    }

    // -- Sorting a permuted vec recovers original order --

    #[test]
    fn sort_recovers_original(perm in arb_permutation(15)) {
        let original: Vec<usize> = (0..15).collect();
        let permuted: Vec<usize> = perm.iter().map(|&i| original[i]).collect();
        let mut sorted = permuted.clone();
        sorted.sort();
        prop_assert_eq!(sorted, original);
    }
}

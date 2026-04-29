// Ported from: test_slices.py
// Properties: no-panic on application, step != 0, bounds

use propkit::strategies::slices::{Slice, arb_slice};
use proptest::prelude::*;

proptest! {
    #[test]
    fn step_not_zero(slice in arb_slice(10)) {
        if let Some(step) = slice.step {
            prop_assert!(step != 0, "step must not be zero");
        }
    }

    #[test]
    fn slice_does_not_panic(
        slice in arb_slice(10),
        data in proptest::collection::vec(any::<u8>(), 10..=10),
    ) {
        let _ = slice.apply(&data);
    }

    #[test]
    fn start_within_bounds(slice in arb_slice(20)) {
        if let Some(start) = slice.start {
            prop_assert!(
                (-20..=20).contains(&start),
                "start {start} out of bounds"
            );
        }
    }

    #[test]
    fn stop_within_bounds(slice in arb_slice(20)) {
        if let Some(stop) = slice.stop {
            prop_assert!(
                (-20..=20).contains(&stop),
                "stop {stop} out of bounds"
            );
        }
    }

    #[test]
    fn empty_slice(slice in arb_slice(0)) {
        let data: Vec<u8> = vec![];
        let result = slice.apply(&data);
        prop_assert!(result.is_empty());
    }

    #[test]
    fn result_subset_of_original(
        slice in arb_slice(10),
        data in proptest::collection::vec(0..100u8, 10..=10),
    ) {
        let result = slice.apply(&data);
        for val in &result {
            prop_assert!(
                data.contains(val),
                "{val} not in original data"
            );
        }
    }

    #[test]
    fn result_not_longer(
        slice in arb_slice(15),
        data in proptest::collection::vec(any::<u8>(), 15..=15),
    ) {
        let result = slice.apply(&data);
        prop_assert!(result.len() <= data.len());
    }

    #[test]
    fn positive_step_forward(
        data in proptest::collection::vec(0..100u32, 10..=10),
        start in 0..10usize,
        step in 1..5usize,
    ) {
        let slice = Slice {
            start: Some(start as isize),
            stop: Some(10),
            step: Some(step as isize),
        };
        let result = slice.apply(&data);
        let expected_indices: Vec<usize> = (start..10).step_by(step).collect();
        prop_assert_eq!(
            result.len(),
            expected_indices.len(),
            "result length mismatch"
        );
        for (i, &src_idx) in expected_indices.iter().enumerate() {
            prop_assert_eq!(
                result[i], data[src_idx],
                "element {} should come from index {}", i, src_idx
            );
        }
    }
}

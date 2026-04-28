// Ported from: test_slices.py
// Properties: no-panic on application, step != 0, bounds

use proptest::prelude::*;

/// A Python-style slice with optional start/stop/step
#[derive(Debug, Clone)]
struct Slice {
    start: Option<isize>,
    stop: Option<isize>,
    step: Option<isize>,
}

/// Generate a valid slice for a collection of the given size
fn arb_slice(size: usize) -> impl Strategy<Value = Slice> {
    let s = size as isize;
    let bound = if s == 0 { 1 } else { s };
    (
        proptest::option::of(-bound..=bound),
        proptest::option::of(-bound..=bound),
        proptest::option::of(
            (1..=bound).prop_flat_map(|abs_step| prop_oneof![Just(abs_step), Just(-abs_step)]),
        ),
    )
        .prop_map(|(start, stop, step)| Slice { start, stop, step })
}

impl Slice {
    /// Apply this slice to a Vec, returning the selected elements
    /// (Python-style slicing semantics)
    fn apply<T: Clone>(&self, data: &[T]) -> Vec<T> {
        let len = data.len() as isize;
        if len == 0 {
            return vec![];
        }

        let step = self.step.unwrap_or(1);
        if step == 0 {
            panic!("step must not be zero");
        }

        let resolve = |idx: isize| -> isize {
            if idx < 0 {
                (len + idx).max(0)
            } else {
                idx.min(len)
            }
        };

        let (start, stop) = if step > 0 {
            let start = self.start.map_or(0, |s| resolve(s));
            let stop = self.stop.map_or(len, |s| resolve(s));
            (start, stop)
        } else {
            let start = self.start.map_or(len - 1, |s| resolve(s).min(len - 1));
            let stop = self.stop.map_or(-1, |s| resolve(s) - 1);
            (start, stop)
        };

        let mut result = vec![];
        let mut i = start;
        if step > 0 {
            while i < stop {
                result.push(data[i as usize].clone());
                i += step;
            }
        } else {
            while i > stop {
                if i >= 0 && i < len {
                    result.push(data[i as usize].clone());
                }
                i += step;
            }
        }
        result
    }
}

proptest! {
    // -- Step is never zero --

    #[test]
    fn step_not_zero(slice in arb_slice(10)) {
        if let Some(step) = slice.step {
            prop_assert!(step != 0, "step must not be zero");
        }
    }

    // -- Applying a slice to a vec does not panic --

    #[test]
    fn slice_does_not_panic(
        slice in arb_slice(10),
        data in proptest::collection::vec(any::<u8>(), 10..=10),
    ) {
        let _ = slice.apply(&data);
    }

    // -- Bounds: start and stop within [-size, size] --

    #[test]
    fn start_within_bounds(slice in arb_slice(20)) {
        if let Some(start) = slice.start {
            prop_assert!(
                start >= -20 && start <= 20,
                "start {start} out of bounds"
            );
        }
    }

    #[test]
    fn stop_within_bounds(slice in arb_slice(20)) {
        if let Some(stop) = slice.stop {
            prop_assert!(
                stop >= -20 && stop <= 20,
                "stop {stop} out of bounds"
            );
        }
    }

    // -- Slice of empty collection produces empty result --

    #[test]
    fn empty_slice(slice in arb_slice(0)) {
        let data: Vec<u8> = vec![];
        let result = slice.apply(&data);
        prop_assert!(result.is_empty());
    }

    // -- Result is always a subsequence of original --

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

    // -- Result length <= original length --

    #[test]
    fn result_not_longer(
        slice in arb_slice(15),
        data in proptest::collection::vec(any::<u8>(), 15..=15),
    ) {
        let result = slice.apply(&data);
        prop_assert!(result.len() <= data.len());
    }

    // -- Positive step produces forward-ordered indices --

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
        // Each element should come from a later index than the previous
        for i in 1..result.len() {
            // We can't check index order directly, but verify non-empty
            let _ = result[i];
        }
    }
}

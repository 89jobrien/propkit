// propkit::strategies::slices — Python-style slice type and strategy
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;

/// A Python-style slice with optional start/stop/step.
#[derive(Debug, Clone)]
pub struct Slice {
    pub start: Option<isize>,
    pub stop: Option<isize>,
    pub step: Option<isize>,
}

impl Slice {
    /// Apply this slice to a slice, returning the selected elements
    /// using Python-style slicing semantics.
    pub fn apply<T: Clone>(&self, data: &[T]) -> Vec<T> {
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
            let start = self.start.map_or(0, &resolve);
            let stop = self.stop.map_or(len, &resolve);
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

/// Generate a valid slice for a collection of the given size.
pub fn arb_slice(size: usize) -> impl Strategy<Value = Slice> {
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

// propkit::strategies — public strategy modules behind feature flags
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod floats;
pub mod permutations;
pub mod recursive;
pub mod slices;

#[cfg(feature = "complex")]
pub mod complex;
#[cfg(feature = "chrono")]
pub mod datetimes;
#[cfg(feature = "regex")]
pub mod regex_props;
#[cfg(feature = "uuid")]
pub mod uuids;

// propkit::strategies::datetimes — chrono date/time strategies
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use proptest::prelude::*;

/// Generates NaiveDate values in the range 1970-01-01 ..= 2100-12-28.
/// Day capped at 28 to avoid invalid dates across all months.
pub fn arb_naive_date() -> impl Strategy<Value = NaiveDate> {
    (1970i32..=2100, 1u32..=12, 1u32..=28)
        .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap())
}

/// Generates NaiveTime values (hour, minute, second -- no sub-second).
pub fn arb_naive_time() -> impl Strategy<Value = NaiveTime> {
    (0u32..24, 0u32..60, 0u32..60).prop_map(|(h, m, s)| NaiveTime::from_hms_opt(h, m, s).unwrap())
}

/// Generates NaiveDateTime by combining arb_naive_date + arb_naive_time.
pub fn arb_naive_datetime() -> impl Strategy<Value = NaiveDateTime> {
    (arb_naive_date(), arb_naive_time()).prop_map(|(date, time)| NaiveDateTime::new(date, time))
}

/// Generates a Duration in seconds within [-86_400, 86_400] (plus/minus 1 day).
pub fn arb_duration_small() -> impl Strategy<Value = Duration> {
    (-86_400i64..=86_400).prop_map(Duration::seconds)
}

/// Generates a Duration in seconds within [0, 86_400] (0..=1 day).
pub fn arb_duration_nonneg() -> impl Strategy<Value = Duration> {
    (0i64..=86_400).prop_map(Duration::seconds)
}

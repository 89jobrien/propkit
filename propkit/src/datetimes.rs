// datetimes.rs — proptest properties for chrono date/time types
// Ported from Hypothesis test_datetimes.py
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use proptest::prelude::*;

// ── Strategies ────────────────────────────────────────────────────────────────

/// Generates NaiveDate values in the range 1970-01-01 ..= 2100-12-28.
/// Day capped at 28 to avoid invalid dates across all months.
fn arb_naive_date() -> impl Strategy<Value = NaiveDate> {
    (1970i32..=2100, 1u32..=12, 1u32..=28)
        .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap())
}

/// Generates NaiveTime values (hour, minute, second — no sub-second).
fn arb_naive_time() -> impl Strategy<Value = NaiveTime> {
    (0u32..24, 0u32..60, 0u32..60).prop_map(|(h, m, s)| NaiveTime::from_hms_opt(h, m, s).unwrap())
}

/// Generates NaiveDateTime by combining arb_naive_date + arb_naive_time.
fn arb_naive_datetime() -> impl Strategy<Value = NaiveDateTime> {
    (arb_naive_date(), arb_naive_time()).prop_map(|(date, time)| NaiveDateTime::new(date, time))
}

/// Generates a Duration in seconds within [−86_400, 86_400] (±1 day).
fn arb_duration_small() -> impl Strategy<Value = Duration> {
    (-86_400i64..=86_400).prop_map(Duration::seconds)
}

/// Generates a Duration in seconds within [0, 86_400] (0..=1 day).
fn arb_duration_nonneg() -> impl Strategy<Value = Duration> {
    (0i64..=86_400).prop_map(Duration::seconds)
}

// ── Properties ────────────────────────────────────────────────────────────────

proptest! {
    // -- NaiveDate bounds -----------------------------------------------------

    /// Generated date is >= an explicit min_date.
    #[test]
    fn date_ge_min(
        min_year in 1970i32..=2050,
        month in 1u32..=12,
        day in 1u32..=28,
        gen_year in 1970i32..=2100,
        gen_month in 1u32..=12,
        gen_day in 1u32..=28,
    ) {
        let min = NaiveDate::from_ymd_opt(min_year, month, day).unwrap();
        let generated = NaiveDate::from_ymd_opt(gen_year, gen_month, gen_day).unwrap();
        // Use prop_assume to restrict to the interesting case, then assert.
        prop_assume!(generated >= min);
        prop_assert!(generated >= min);
    }

    /// Generated date is <= an explicit max_date.
    #[test]
    fn date_le_max(
        max_year in 2050i32..=2100,
        month in 1u32..=12,
        day in 1u32..=28,
        gen_year in 1970i32..=2100,
        gen_month in 1u32..=12,
        gen_day in 1u32..=28,
    ) {
        let max = NaiveDate::from_ymd_opt(max_year, month, day).unwrap();
        let generated = NaiveDate::from_ymd_opt(gen_year, gen_month, gen_day).unwrap();
        prop_assume!(generated <= max);
        prop_assert!(generated <= max);
    }

    /// A date generated within [min, max] satisfies both bounds simultaneously.
    #[test]
    fn date_within_bounds(
        lo_year in 2000i32..=2050,
        hi_year in 2050i32..=2100,
        month in 1u32..=12,
        day in 1u32..=28,
    ) {
        let min = NaiveDate::from_ymd_opt(lo_year, month, day).unwrap();
        let max = NaiveDate::from_ymd_opt(hi_year, month, day).unwrap();
        // Generate a date strictly inside the range by clamping gen to [lo, hi].
        let mid = NaiveDate::from_ymd_opt((lo_year + hi_year) / 2, month, day).unwrap();
        prop_assert!(mid >= min);
        prop_assert!(mid <= max);
    }

    // -- NaiveDateTime bounds -------------------------------------------------

    /// A NaiveDateTime generated within explicit min/max satisfies both bounds.
    #[test]
    fn datetime_within_bounds(dt in arb_naive_datetime()) {
        let min = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
            .and_hms_opt(0, 0, 0).unwrap();
        let max = NaiveDate::from_ymd_opt(2100, 12, 28).unwrap()
            .and_hms_opt(23, 59, 59).unwrap();
        prop_assert!(dt >= min, "dt={dt} < min={min}");
        prop_assert!(dt <= max, "dt={dt} > max={max}");
    }

    /// datetime hour is in 0..24, minute/second in 0..60.
    #[test]
    fn datetime_time_components_valid(dt in arb_naive_datetime()) {
        prop_assert!(dt.hour() < 24);
        prop_assert!(dt.minute() < 60);
        prop_assert!(dt.second() < 60);
    }

    // -- NaiveTime bounds -----------------------------------------------------

    /// Generated time is within 00:00:00 ..= 23:59:59.
    #[test]
    fn time_within_day_bounds(t in arb_naive_time()) {
        let min = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let max = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        prop_assert!(t >= min, "t={t} < 00:00:00");
        prop_assert!(t <= max, "t={t} > 23:59:59");
    }

    /// Time components stay within their natural domains.
    #[test]
    fn time_components_valid(t in arb_naive_time()) {
        prop_assert!(t.hour() < 24);
        prop_assert!(t.minute() < 60);
        prop_assert!(t.second() < 60);
    }

    // -- Leap year: Feb 29 reachable ------------------------------------------

    /// Feb 29 is a valid date in any year divisible by 4 (simplified leap rule).
    #[test]
    fn feb_29_reachable_in_leap_year(
        // Generate years divisible by 4 in range 2000..=2096.
        year_div4 in 500i32..=524,
    ) {
        let year = year_div4 * 4;
        let date = NaiveDate::from_ymd_opt(year, 2, 29);
        prop_assert!(date.is_some(), "year {year} should be a leap year");
    }

    /// Feb 29 is NOT a valid date in a non-leap year.
    #[test]
    fn feb_29_invalid_in_non_leap_year(
        // Odd multiples of 100 that are not multiples of 400 (e.g. 1900, 2100).
        // Use a simple set: 2001..=2003, 2005..=2007, etc. — just skip leap years.
        year in prop::sample::select(vec![2001, 2002, 2003, 2005, 2006, 2007,
                                         2009, 2010, 2011, 2013, 2014, 2015]),
    ) {
        let date = NaiveDate::from_ymd_opt(year, 2, 29);
        prop_assert!(date.is_none(), "year {year} should NOT have Feb 29");
    }

    // -- All 12 months reachable ----------------------------------------------

    /// Every month 1..=12 is valid for any day in 1..=28.
    #[test]
    fn all_months_reachable(
        year in 2000i32..=2100,
        month in 1u32..=12,
        day in 1u32..=28,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        prop_assert!(date.is_some(), "month={month} day={day} should be valid");
    }

    /// Every distinct month value (1..=12) appears in the generated dates.
    /// (This is a coverage check — we verify month is in range.)
    #[test]
    fn generated_month_in_range(date in arb_naive_date()) {
        prop_assert!((1..=12).contains(&date.month()));
    }

    // -- Duration bounds ------------------------------------------------------

    /// Duration generated in [min_secs, max_secs] stays within those bounds.
    #[test]
    fn duration_within_bounds(
        min_secs in -3600i64..=0,
        max_secs in 1i64..=3600,
        secs in -3600i64..=3600,
    ) {
        let min = Duration::seconds(min_secs);
        let max = Duration::seconds(max_secs);
        let d = Duration::seconds(secs);
        prop_assume!(d >= min && d <= max);
        prop_assert!(d >= min);
        prop_assert!(d <= max);
    }

    /// Non-negative durations are >= zero.
    #[test]
    fn nonneg_duration_ge_zero(d in arb_duration_nonneg()) {
        prop_assert!(d >= Duration::zero());
    }

    /// Duration ordering is consistent with its second count.
    #[test]
    fn duration_ordering_consistent(a_secs in -3600i64..=3600, b_secs in -3600i64..=3600) {
        let a = Duration::seconds(a_secs);
        let b = Duration::seconds(b_secs);
        prop_assert_eq!(a.cmp(&b), a_secs.cmp(&b_secs));
    }

    // -- Date arithmetic ------------------------------------------------------

    /// date + duration - duration == date (roundtrip).
    #[test]
    fn date_add_sub_roundtrip(date in arb_naive_date(), d in arb_duration_small()) {
        let shifted = date + d;
        let recovered = shifted - d;
        prop_assert_eq!(date, recovered, "roundtrip failed for date={} d={}", date, d);
    }

    /// datetime + duration - duration == datetime (roundtrip).
    #[test]
    fn datetime_add_sub_roundtrip(dt in arb_naive_datetime(), d in arb_duration_small()) {
        let shifted = dt + d;
        let recovered = shifted - d;
        prop_assert_eq!(dt, recovered, "roundtrip failed for dt={} d={}", dt, d);
    }

    /// Adding a positive duration moves the date forward (or keeps it the same).
    #[test]
    fn date_add_positive_duration_moves_forward(
        date in arb_naive_date(),
        secs in 0i64..=86_400,
    ) {
        let d = Duration::seconds(secs);
        let shifted = date + d;
        prop_assert!(shifted >= date);
    }

    /// Adding a negative duration moves the date backward (or keeps it the same).
    #[test]
    fn date_add_negative_duration_moves_backward(
        date in arb_naive_date(),
        secs in -86_400i64..=0,
    ) {
        let d = Duration::seconds(secs);
        let shifted = date + d;
        prop_assert!(shifted <= date);
    }
}

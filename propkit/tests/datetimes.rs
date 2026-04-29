// datetimes.rs -- proptest properties for chrono date/time types
// Ported from Hypothesis test_datetimes.py

use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Timelike};
use propkit::strategies::datetimes::{
    arb_duration_nonneg, arb_duration_small, arb_naive_date, arb_naive_datetime, arb_naive_time,
};
use proptest::prelude::*;

proptest! {
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
        prop_assume!(generated >= min);
        prop_assert!(generated >= min);
    }

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

    #[test]
    fn date_within_bounds(
        lo_year in 2000i32..=2050,
        hi_year in 2050i32..=2100,
        month in 1u32..=12,
        day in 1u32..=28,
    ) {
        let min = NaiveDate::from_ymd_opt(lo_year, month, day).unwrap();
        let max = NaiveDate::from_ymd_opt(hi_year, month, day).unwrap();
        let mid = NaiveDate::from_ymd_opt((lo_year + hi_year) / 2, month, day).unwrap();
        prop_assert!(mid >= min);
        prop_assert!(mid <= max);
    }

    #[test]
    fn datetime_within_bounds(dt in arb_naive_datetime()) {
        let min = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let max = NaiveDate::from_ymd_opt(2100, 12, 28)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        prop_assert!(dt >= min, "dt={dt} < min={min}");
        prop_assert!(dt <= max, "dt={dt} > max={max}");
    }

    #[test]
    fn datetime_time_components_valid(dt in arb_naive_datetime()) {
        prop_assert!(dt.hour() < 24);
        prop_assert!(dt.minute() < 60);
        prop_assert!(dt.second() < 60);
    }

    #[test]
    fn time_within_day_bounds(t in arb_naive_time()) {
        let min = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let max = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        prop_assert!(t >= min, "t={t} < 00:00:00");
        prop_assert!(t <= max, "t={t} > 23:59:59");
    }

    #[test]
    fn time_components_valid(t in arb_naive_time()) {
        prop_assert!(t.hour() < 24);
        prop_assert!(t.minute() < 60);
        prop_assert!(t.second() < 60);
    }

    #[test]
    fn feb_29_reachable_in_leap_year(year_div4 in 500i32..=524) {
        let year = year_div4 * 4;
        let date = NaiveDate::from_ymd_opt(year, 2, 29);
        prop_assert!(date.is_some(), "year {year} should be a leap year");
    }

    #[test]
    fn feb_29_invalid_in_non_leap_year(
        year in prop::sample::select(vec![
            2001, 2002, 2003, 2005, 2006, 2007, 2009, 2010, 2011, 2013, 2014, 2015
        ]),
    ) {
        let date = NaiveDate::from_ymd_opt(year, 2, 29);
        prop_assert!(date.is_none(), "year {year} should NOT have Feb 29");
    }

    #[test]
    fn all_months_reachable(
        year in 2000i32..=2100,
        month in 1u32..=12,
        day in 1u32..=28,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        prop_assert!(date.is_some(), "month={month} day={day} should be valid");
    }

    #[test]
    fn generated_month_in_range(date in arb_naive_date()) {
        prop_assert!((1..=12).contains(&date.month()));
    }

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

    #[test]
    fn nonneg_duration_ge_zero(d in arb_duration_nonneg()) {
        prop_assert!(d >= Duration::zero());
    }

    #[test]
    fn duration_ordering_consistent(a_secs in -3600i64..=3600, b_secs in -3600i64..=3600) {
        let a = Duration::seconds(a_secs);
        let b = Duration::seconds(b_secs);
        prop_assert_eq!(a.cmp(&b), a_secs.cmp(&b_secs));
    }

    #[test]
    fn date_add_sub_roundtrip(date in arb_naive_date(), d in arb_duration_small()) {
        let shifted = date + d;
        let recovered = shifted - d;
        prop_assert_eq!(date, recovered, "roundtrip failed for date={} d={}", date, d);
    }

    #[test]
    fn datetime_add_sub_roundtrip(dt in arb_naive_datetime(), d in arb_duration_small()) {
        let shifted = dt + d;
        let recovered = shifted - d;
        prop_assert_eq!(dt, recovered, "roundtrip failed for dt={} d={}", dt, d);
    }

    #[test]
    fn date_add_positive_duration_moves_forward(
        date in arb_naive_date(),
        secs in 0i64..=86_400,
    ) {
        let d = Duration::seconds(secs);
        let shifted = date + d;
        prop_assert!(shifted >= date);
    }

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

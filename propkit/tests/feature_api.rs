//! Red-phase tests for chrono/regex/uuid feature-gated public APIs.

// -- chrono feature --

#[cfg(feature = "chrono")]
mod chrono_tests {
    use chrono::{Datelike, Timelike};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn arb_naive_date_works(d in propkit::strategies::datetimes::arb_naive_date()) {
            prop_assert!(d.year() >= 1970);
        }

        #[test]
        fn arb_naive_time_works(t in propkit::strategies::datetimes::arb_naive_time()) {
            prop_assert!(t.hour() < 24);
        }

        #[test]
        fn arb_naive_datetime_works(
            dt in propkit::strategies::datetimes::arb_naive_datetime()
        ) {
            prop_assert!(dt.date().year() >= 1970);
        }

        #[test]
        fn arb_duration_small_works(
            d in propkit::strategies::datetimes::arb_duration_small()
        ) {
            let secs = d.num_seconds();
            prop_assert!((-86_400..=86_400).contains(&secs));
        }

        #[test]
        fn arb_duration_nonneg_works(
            d in propkit::strategies::datetimes::arb_duration_nonneg()
        ) {
            prop_assert!(d.num_seconds() >= 0);
        }
    }
}

// -- regex feature --

#[cfg(feature = "regex")]
mod regex_tests {
    #[test]
    fn re_digits_is_pub() {
        let re = propkit::strategies::regex_props::re_digits();
        assert!(re.is_match("123"));
    }

    #[test]
    fn re_word_is_pub() {
        let re = propkit::strategies::regex_props::re_word();
        assert!(re.is_match("hello"));
    }

    #[test]
    fn re_hex_is_pub() {
        let re = propkit::strategies::regex_props::re_hex();
        assert!(re.is_match("deadBEEF"));
    }

    #[test]
    fn re_email_is_pub() {
        let re = propkit::strategies::regex_props::re_email();
        assert!(re.is_match("foo@bar.com"));
    }

    #[test]
    fn re_ipv4_is_pub() {
        let re = propkit::strategies::regex_props::re_ipv4();
        assert!(re.is_match("127.0.0.1"));
    }
}

// -- uuid feature --

#[cfg(feature = "uuid")]
mod uuid_tests {
    use proptest::prelude::*;

    #[test]
    fn re_uuid_is_pub() {
        let re = propkit::strategies::uuids::re_uuid();
        assert!(re.is_match("550e8400-e29b-41d4-a716-446655440000"));
    }

    proptest! {
        #[test]
        fn arb_uuid_bytes_works(u in propkit::strategies::uuids::arb_uuid_bytes()) {
            prop_assert_eq!(u.as_bytes().len(), 16);
        }

        #[test]
        fn arb_uuid_v4_works(u in propkit::strategies::uuids::arb_uuid_v4()) {
            prop_assert_eq!(u.get_version(), Some(uuid::Version::Random));
        }
    }
}

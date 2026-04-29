// propkit::uuids -- property tests for UUID generation
// Ported from Hypothesis test_uuids.py

use propkit::strategies::uuids::{arb_uuid_bytes, arb_uuid_v4, re_uuid};
use proptest::prelude::*;
use uuid::{Uuid, Variant, Version};

proptest! {
    #[test]
    fn uuid_string_roundtrip(u in arb_uuid_bytes()) {
        let s = u.to_string();
        let parsed = Uuid::parse_str(&s).expect("must parse");
        prop_assert_eq!(u, parsed);
    }

    #[test]
    fn uuid_bytes_roundtrip(u in arb_uuid_bytes()) {
        let bytes = *u.as_bytes();
        let rebuilt = Uuid::from_bytes(bytes);
        prop_assert_eq!(u, rebuilt);
    }

    #[test]
    fn uuid_string_format(u in arb_uuid_bytes()) {
        let s = u.to_string();
        prop_assert!(re_uuid().is_match(&s), "UUID string did not match pattern: {s}");
    }

    #[test]
    fn uuid_string_length(u in arb_uuid_bytes()) {
        prop_assert_eq!(u.to_string().len(), 36);
    }

    #[test]
    fn uuid_v4_version(u in arb_uuid_v4()) {
        prop_assert_eq!(u.get_version(), Some(Version::Random));
    }

    #[test]
    fn uuid_v4_variant(u in arb_uuid_v4()) {
        prop_assert_eq!(u.get_variant(), Variant::RFC4122);
    }

    #[test]
    fn uuid_not_nil_unless_all_zero(u in arb_uuid_bytes()) {
        let all_zero = u.as_bytes().iter().all(|&b| b == 0);
        if all_zero {
            prop_assert_eq!(u, Uuid::nil());
        } else {
            prop_assert_ne!(u, Uuid::nil());
        }
    }

    #[test]
    fn uuid_two_independent_differ(
        a in any::<[u8; 16]>(),
        b in any::<[u8; 16]>(),
    ) {
        prop_assume!(a != b);
        let ua = Uuid::from_bytes(a);
        let ub = Uuid::from_bytes(b);
        prop_assert_ne!(ua, ub);
    }

    #[test]
    fn uuid_eq_consistent_with_bytes(u in arb_uuid_bytes()) {
        let bytes = *u.as_bytes();
        let v = Uuid::from_bytes(bytes);
        prop_assert_eq!(u, v);
    }
}

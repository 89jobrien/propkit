// propkit::uuids -- property tests for UUID generation
// Ported from Hypothesis test_uuids.py
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;
use regex::Regex;
use std::sync::OnceLock;
use uuid::{Uuid, Variant, Version};

fn re_uuid() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap()
    })
}

/// Arbitrary UUID v4 built from random bytes.
///
/// Uuid::new_v4() requires getrandom which proptest can't seed; instead we
/// take 16 raw bytes and construct a UUID from them so proptest controls the
/// entropy. For variant/version tests we set the RFC4122 fields manually.
fn arb_uuid_bytes() -> impl Strategy<Value = Uuid> {
    any::<[u8; 16]>().prop_map(Uuid::from_bytes)
}

/// Arbitrary RFC 4122 v4 UUID: version nibble = 4, variant = RFC4122.
fn arb_uuid_v4() -> impl Strategy<Value = Uuid> {
    any::<[u8; 16]>().prop_map(|mut bytes| {
        // Set version to 4: high nibble of byte 6
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        // Set variant to RFC4122: high two bits of byte 8 = 0b10
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Uuid::from_bytes(bytes)
    })
}

proptest! {
    // --- roundtrip -------------------------------------------------------

    /// A UUID serialised to its hyphenated string form parses back to itself.
    #[test]
    fn uuid_string_roundtrip(u in arb_uuid_bytes()) {
        let s = u.to_string();
        let parsed = Uuid::parse_str(&s).expect("must parse");
        prop_assert_eq!(u, parsed);
    }

    /// A UUID serialised to bytes roundtrips through from_bytes.
    #[test]
    fn uuid_bytes_roundtrip(u in arb_uuid_bytes()) {
        let bytes = *u.as_bytes();
        let rebuilt = Uuid::from_bytes(bytes);
        prop_assert_eq!(u, rebuilt);
    }

    // --- format ----------------------------------------------------------

    /// Hyphenated string matches the canonical UUID regex pattern.
    #[test]
    fn uuid_string_format(u in arb_uuid_bytes()) {
        let s = u.to_string();
        prop_assert!(re_uuid().is_match(&s), "UUID string did not match pattern: {s}");
    }

    /// Hyphenated string is exactly 36 characters long.
    #[test]
    fn uuid_string_length(u in arb_uuid_bytes()) {
        prop_assert_eq!(u.to_string().len(), 36);
    }

    // --- v4 fields -------------------------------------------------------

    /// A v4 UUID has version == 4.
    #[test]
    fn uuid_v4_version(u in arb_uuid_v4()) {
        prop_assert_eq!(u.get_version(), Some(Version::Random));
    }

    /// A v4 UUID has the RFC4122 variant.
    #[test]
    fn uuid_v4_variant(u in arb_uuid_v4()) {
        prop_assert_eq!(u.get_variant(), Variant::RFC4122);
    }

    // --- nil -------------------------------------------------------------

    /// No UUID constructed from random bytes equals the nil UUID (all zeros).
    ///
    /// The probability that proptest generates 16 zero bytes is negligible;
    /// we verify the invariant holds for every generated value.
    #[test]
    fn uuid_not_nil_unless_all_zero(u in arb_uuid_bytes()) {
        let all_zero = u.as_bytes().iter().all(|&b| b == 0);
        if all_zero {
            prop_assert_eq!(u, Uuid::nil());
        } else {
            prop_assert_ne!(u, Uuid::nil());
        }
    }

    // --- uniqueness ------------------------------------------------------

    /// Two independently drawn byte arrays almost surely yield different UUIDs.
    ///
    /// The test fails only on a 2^-128 collision — treated as impossible.
    #[test]
    fn uuid_two_independent_differ(
        a in any::<[u8; 16]>(),
        b in any::<[u8; 16]>(),
    ) {
        // Only assert if the byte arrays differ; equal inputs trivially give equal UUIDs.
        prop_assume!(a != b);
        let ua = Uuid::from_bytes(a);
        let ub = Uuid::from_bytes(b);
        prop_assert_ne!(ua, ub);
    }

    // --- ordering / hashing consistency ---------------------------------

    /// PartialEq is consistent with the byte representation.
    #[test]
    fn uuid_eq_consistent_with_bytes(u in arb_uuid_bytes()) {
        let bytes = *u.as_bytes();
        let v = Uuid::from_bytes(bytes);
        prop_assert_eq!(u, v);
    }
}

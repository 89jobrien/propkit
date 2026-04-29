// propkit::strategies::uuids — UUID strategies
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use proptest::prelude::*;
use regex::Regex;
use std::sync::OnceLock;
use uuid::Uuid;

pub fn re_uuid() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap()
    })
}

/// Arbitrary UUID built from random bytes (not necessarily v4).
pub fn arb_uuid_bytes() -> impl Strategy<Value = Uuid> {
    any::<[u8; 16]>().prop_map(Uuid::from_bytes)
}

/// Arbitrary RFC 4122 v4 UUID: version nibble = 4, variant = RFC4122.
pub fn arb_uuid_v4() -> impl Strategy<Value = Uuid> {
    any::<[u8; 16]>().prop_map(|mut bytes| {
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Uuid::from_bytes(bytes)
    })
}

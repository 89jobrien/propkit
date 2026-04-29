// Ported from: test_simple_strings.py
// Properties: alphabet constraints, encoding validity, char exclusion,
//             ASCII-only, byte length bounds

use proptest::prelude::*;

proptest! {
    // -- All Rust strings are valid UTF-8 by construction --

    #[test]
    fn string_is_valid_utf8(s in ".*") {
        prop_assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }

    // -- Byte vec respects max size --

    #[test]
    fn binary_respects_max_size(
        v in proptest::collection::vec(any::<u8>(), 0..=5)
    ) {
        prop_assert!(v.len() <= 5);
    }

    // -- Alphabet constraint: chars from a fixed set --

    #[test]
    fn respects_alphabet_ab(s in "[ab]{0,20}") {
        prop_assert!(
            s.chars().all(|c| c == 'a' || c == 'b'),
            "unexpected char in '{s}'"
        );
    }

    #[test]
    fn respects_alphabet_cdef(s in "[c-f]{0,20}") {
        prop_assert!(
            s.chars().all(|c| ('c'..='f').contains(&c)),
            "unexpected char in '{s}'"
        );
    }

    // -- Character exclusion --

    #[test]
    fn no_newlines(s in "[^\n]{0,50}") {
        prop_assert!(
            !s.contains('\n'),
            "found newline in '{s}'"
        );
    }

    #[test]
    fn no_whitespace(s in "[^ \t\n\r]{0,50}") {
        prop_assert!(
            !s.contains(' ')
                && !s.contains('\t')
                && !s.contains('\n')
                && !s.contains('\r'),
            "found whitespace in '{s}'"
        );
    }

    // -- ASCII only --

    #[test]
    fn ascii_only(s in "[[:ascii:]]{0,50}") {
        prop_assert!(
            s.is_ascii(),
            "non-ASCII char in '{s}'"
        );
    }

    // -- Digit-only strings --

    #[test]
    fn digits_only(s in "[0-9]{1,10}") {
        prop_assert!(
            s.chars().all(|c| c.is_ascii_digit()),
            "non-digit in '{s}'"
        );
    }

    // -- Hex strings --

    #[test]
    fn hex_only(s in "[0-9a-fA-F]{1,16}") {
        prop_assert!(
            s.chars().all(|c| c.is_ascii_hexdigit()),
            "non-hex char in '{s}'"
        );
    }

    // -- Fixed-length strings --

    #[test]
    fn fixed_length_string(s in "[a-z]{5}") {
        prop_assert_eq!(s.len(), 5);
        prop_assert!(s.chars().all(|c| c.is_ascii_lowercase()));
    }

    // -- String length bounds --

    #[test]
    fn string_length_bounded(s in ".{3,8}") {
        let char_count = s.chars().count();
        prop_assert!((3..=8).contains(&char_count));
    }

    // -- Empty string is valid --

    #[test]
    fn can_generate_empty(s in ".{0,5}") {
        // Just verify it doesn't panic; empty is a valid output
        let _ = s.len();
    }
}

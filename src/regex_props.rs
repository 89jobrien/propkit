// Ported from: test_regex.py
// Properties: generated strings match the source regex pattern

use proptest::prelude::*;
use regex::Regex;

proptest! {
    // -- Core property: generated string matches the pattern --

    #[test]
    fn digits_match(s in "[0-9]+") {
        let re = Regex::new(r"^[0-9]+$").unwrap();
        prop_assert!(re.is_match(&s), "'{s}' does not match [0-9]+");
    }

    #[test]
    fn word_chars_match(s in r"\w+") {
        let re = Regex::new(r"^\w+$").unwrap();
        prop_assert!(re.is_match(&s), "'{s}' does not match \\w+");
    }

    #[test]
    fn hex_pattern_match(s in "[0-9a-fA-F]{8}") {
        let re = Regex::new(r"^[0-9a-fA-F]{8}$").unwrap();
        prop_assert!(re.is_match(&s), "'{s}' does not match hex pattern");
    }

    #[test]
    fn email_like_match(s in "[a-z]{1,10}@[a-z]{1,10}\\.[a-z]{2,4}") {
        let re = Regex::new(r"^[a-z]{1,10}@[a-z]{1,10}\.[a-z]{2,4}$").unwrap();
        prop_assert!(re.is_match(&s), "'{s}' does not match email pattern");
    }

    #[test]
    fn ipv4_like_match(
        s in "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}"
    ) {
        let re = Regex::new(
            r"^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$"
        ).unwrap();
        prop_assert!(re.is_match(&s), "'{s}' does not match IPv4 pattern");
    }

    // -- Character class membership --

    #[test]
    fn whitespace_class(s in r"\s+") {
        prop_assert!(
            s.chars().all(|c| c.is_whitespace()),
            "non-whitespace char in '{s}'"
        );
    }

    #[test]
    fn digit_class(s in "[0-9]+") {
        prop_assert!(
            s.chars().all(|c| c.is_ascii_digit()),
            "non-digit in '{s}'"
        );
    }

    #[test]
    fn alpha_class(s in "[a-zA-Z]+") {
        prop_assert!(
            s.chars().all(|c| c.is_ascii_alphabetic()),
            "non-alpha in '{s}'"
        );
    }

    // -- Anchoring: full match --

    #[test]
    fn anchored_pattern(s in "abc") {
        prop_assert_eq!(s, "abc");
    }

    #[test]
    fn alternation(s in "(foo|bar|baz)") {
        prop_assert!(
            s == "foo" || s == "bar" || s == "baz",
            "unexpected value '{s}'"
        );
    }

    // -- Dot does not produce newline (by default in proptest regex) --

    #[test]
    fn dot_no_newline(s in ".{1,20}") {
        prop_assert!(
            !s.contains('\n'),
            "dot produced newline in '{s}'"
        );
    }

    // -- Optional and repetition --

    #[test]
    fn optional_group(s in "a(bc)?d") {
        prop_assert!(s == "ad" || s == "abcd", "unexpected '{s}'");
    }

    #[test]
    fn kleene_star(s in "a*") {
        prop_assert!(s.chars().all(|c| c == 'a'));
    }

    #[test]
    fn plus_nonempty(s in "a+") {
        prop_assert!(!s.is_empty());
        prop_assert!(s.chars().all(|c| c == 'a'));
    }

    // -- Output type matches pattern type --
    // In Rust, proptest regex always produces String (UTF-8)

    #[test]
    fn output_is_utf8(s in ".*") {
        prop_assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }
}

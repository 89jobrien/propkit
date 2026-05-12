// Property tests for the property generator — verify output invariants.

#[path = "../src/analyzer.rs"]
mod analyzer;
#[path = "../src/generators/property.rs"]
mod property;

use analyzer::{Confidence, FileAnalysis, Property, TypeInfo};
use proptest::prelude::*;

/// Strategy for valid Rust type names.
fn arb_type_name() -> impl Strategy<Value = String> {
    "[A-Z][a-z]{1,9}([A-Z][a-z]{1,9})?"
}

/// Build a FileAnalysis with one type having the given properties.
fn make_analysis(type_name: &str, props: Vec<(&str, Confidence)>) -> Vec<FileAnalysis> {
    let properties = props
        .into_iter()
        .map(|(desc, conf)| Property {
            description: desc.to_string(),
            confidence: conf,
        })
        .collect();
    vec![FileAnalysis {
        path: "test.rs".to_string(),
        types: vec![TypeInfo {
            name: type_name.to_string(),
            derives: vec!["Clone".to_string()],
            impl_traits: vec![],
            properties,
        }],
        functions: vec![],
    }]
}

mod generator_properties {
    use super::*;

    proptest! {
        #[test]
        fn empty_analysis_produces_empty_output(_x in 0u8..1) {
            let output = property::generate_tests(&[], Confidence::Low);
            prop_assert!(output.is_empty(), "empty analysis should give empty output");
        }

        #[test]
        fn nonempty_output_has_proptest_block(name in arb_type_name()) {
            let analyses = make_analysis(
                &name,
                vec![("clone equality", Confidence::High)],
            );
            let output = property::generate_tests(&analyses, Confidence::Low);
            if !output.is_empty() {
                prop_assert!(
                    output.contains("proptest!"),
                    "non-empty output should contain proptest! macro"
                );
                prop_assert!(
                    output.contains("use proptest::prelude::*;"),
                    "non-empty output should contain proptest import"
                );
            }
        }

        #[test]
        fn nonempty_output_contains_type_name(name in arb_type_name()) {
            let analyses = make_analysis(
                &name,
                vec![("clone equality", Confidence::High)],
            );
            let output = property::generate_tests(&analyses, Confidence::Low);
            if !output.is_empty() {
                prop_assert!(
                    output.contains(&name),
                    "output should reference type name {name}"
                );
            }
        }

        #[test]
        fn confidence_filtering_is_monotonic(name in arb_type_name()) {
            let analyses = make_analysis(
                &name,
                vec![
                    ("clone equality", Confidence::High),
                    ("serde roundtrip", Confidence::Medium),
                    ("hash/eq consistency", Confidence::Low),
                ],
            );
            let low = property::generate_tests(&analyses, Confidence::Low);
            let med = property::generate_tests(&analyses, Confidence::Medium);
            let high = property::generate_tests(&analyses, Confidence::High);
            prop_assert!(
                low.len() >= med.len(),
                "Low filter ({}) should produce >= Medium filter ({})",
                low.len(), med.len()
            );
            prop_assert!(
                med.len() >= high.len(),
                "Medium filter ({}) should produce >= High filter ({})",
                med.len(), high.len()
            );
        }

        #[test]
        fn high_tests_subset_of_low(name in arb_type_name()) {
            let analyses = make_analysis(
                &name,
                vec![
                    ("clone equality", Confidence::High),
                    ("serde roundtrip", Confidence::Medium),
                ],
            );
            let low = property::generate_tests(&analyses, Confidence::Low);
            let high = property::generate_tests(&analyses, Confidence::High);
            // Every line in high output should appear in low output
            if !high.is_empty() {
                for line in high.lines().filter(|l| l.contains("fn ")) {
                    prop_assert!(
                        low.contains(line.trim()),
                        "high-confidence test line missing from low output: {line}"
                    );
                }
            }
        }

        #[test]
        fn unknown_property_produces_no_test(name in arb_type_name()) {
            let analyses = make_analysis(
                &name,
                vec![("some unknown property xyz", Confidence::High)],
            );
            let output = property::generate_tests(&analyses, Confidence::Low);
            // Unknown property → generate_type_test returns None → no test body
            // Output might still have the proptest! wrapper but no test functions
            prop_assert!(
                !output.contains("#[test]"),
                "unknown property should not produce a test function"
            );
        }
    }
}

mod to_snake_case_properties {
    use super::*;

    proptest! {
        #[test]
        fn output_is_lowercase(s in "[A-Za-z]{1,20}") {
            let result = property::to_snake_case(&s);
            prop_assert!(
                result.chars().all(|c| c.is_lowercase() || c == '_'),
                "to_snake_case({s:?}) = {result:?} contains uppercase"
            );
        }

        #[test]
        fn output_at_least_as_long(s in "[A-Za-z]{1,20}") {
            let result = property::to_snake_case(&s);
            prop_assert!(
                result.len() >= s.len(),
                "to_snake_case({s:?}) = {result:?} is shorter than input"
            );
        }

        #[test]
        fn idempotent(s in "[A-Za-z]{1,20}") {
            let once = property::to_snake_case(&s);
            let twice = property::to_snake_case(&once);
            let msg = format!("to_snake_case is not idempotent for {s:?}");
            prop_assert_eq!(once, twice, "{}", msg);
        }

        #[test]
        fn no_leading_underscore(s in "[A-Za-z]{1,20}") {
            let result = property::to_snake_case(&s);
            prop_assert!(
                !result.starts_with('_'),
                "to_snake_case({s:?}) = {result:?} starts with underscore"
            );
        }
    }
}

// Property tests for the analyzer — verify detection invariants hold for all inputs.

#[path = "../src/analyzer.rs"]
mod analyzer;

use proptest::prelude::*;
use std::io::Write;

/// Strategy for valid Rust type names: PascalCase, 1-20 chars.
fn arb_type_name() -> impl Strategy<Value = String> {
    "[A-Z][a-z]{1,9}([A-Z][a-z]{1,9})?"
}

/// Write source to a temp file, analyze it, return the result.
fn analyze_source(source: &str) -> Option<analyzer::FileAnalysis> {
    let mut f = tempfile::NamedTempFile::new().expect("create tempfile");
    f.write_all(source.as_bytes()).expect("write source");
    f.flush().expect("flush");
    analyzer::analyze_file(f.path())
}

mod analyzer_properties {
    use super::*;

    proptest! {
        #[test]
        fn empty_source_produces_no_types(_x in 0u8..1) {
            let result = analyze_source("");
            if let Some(analysis) = result {
                prop_assert!(analysis.types.is_empty());
                prop_assert!(analysis.functions.is_empty());
            }
        }

        #[test]
        fn hash_eq_detected(name in arb_type_name()) {
            let source = format!(
                "#[derive(Hash, Eq, PartialEq)]\nstruct {name} {{ x: u32 }}\n"
            );
            let analysis = analyze_source(&source).expect("should parse");
            let has_hash_eq = analysis.types.iter().any(|t| {
                t.properties.iter().any(|p| p.description == "hash/eq consistency")
            });
            prop_assert!(has_hash_eq, "should detect hash/eq consistency for {name}");
        }

        #[test]
        fn clone_detected(name in arb_type_name()) {
            let source = format!(
                "#[derive(Clone, PartialEq)]\nstruct {name} {{ x: u32 }}\n"
            );
            let analysis = analyze_source(&source).expect("should parse");
            let has_clone = analysis.types.iter().any(|t| {
                t.properties.iter().any(|p| p.description == "clone equality")
            });
            prop_assert!(has_clone, "should detect clone equality for {name}");
        }

        #[test]
        fn serde_detected(name in arb_type_name()) {
            let source = format!(
                "#[derive(Serialize, Deserialize)]\nstruct {name} {{ x: u32 }}\n"
            );
            let analysis = analyze_source(&source).expect("should parse");
            let has_serde = analysis.types.iter().any(|t| {
                t.properties.iter().any(|p| p.description == "serde roundtrip")
            });
            prop_assert!(has_serde, "should detect serde roundtrip for {name}");
        }

        #[test]
        fn more_derives_never_reduce_properties(name in arb_type_name()) {
            let small = format!(
                "#[derive(Clone, PartialEq)]\nstruct {name} {{ x: u32 }}\n"
            );
            let big = format!(
                "#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]\n\
                 struct {name} {{ x: u32 }}\n"
            );
            let small_count = analyze_source(&small)
                .map(|a| a.types.iter().map(|t| t.properties.len()).sum::<usize>())
                .unwrap_or(0);
            let big_count = analyze_source(&big)
                .map(|a| a.types.iter().map(|t| t.properties.len()).sum::<usize>())
                .unwrap_or(0);
            prop_assert!(
                big_count >= small_count,
                "more derives ({big_count}) should not reduce properties ({small_count})"
            );
        }

        #[test]
        fn types_are_sorted_by_name(_x in 0u8..1) {
            let source = "\
                #[derive(Clone, PartialEq)]\nstruct Zebra { x: u32 }\n\
                #[derive(Clone, PartialEq)]\nstruct Alpha { x: u32 }\n\
                #[derive(Clone, PartialEq)]\nstruct Mango { x: u32 }\n";
            let analysis = analyze_source(source).expect("should parse");
            let names: Vec<&str> = analysis.types.iter().map(|t| t.name.as_str()).collect();
            let mut sorted = names.clone();
            sorted.sort();
            prop_assert_eq!(names, sorted, "types should be sorted by name");
        }

        #[test]
        fn analysis_is_deterministic(name in arb_type_name()) {
            let source = format!(
                "#[derive(Clone, PartialEq, Eq, Hash)]\nstruct {name} {{ x: u32 }}\n"
            );
            let a = analyze_source(&source);
            let b = analyze_source(&source);
            let a_props: Vec<String> = a.iter()
                .flat_map(|a| a.types.iter())
                .flat_map(|t| t.properties.iter())
                .map(|p| p.description.clone())
                .collect();
            let b_props: Vec<String> = b.iter()
                .flat_map(|a| a.types.iter())
                .flat_map(|t| t.properties.iter())
                .map(|p| p.description.clone())
                .collect();
            prop_assert_eq!(a_props, b_props);
        }
    }
}

// Property tests for scaffold generators — verify invariants hold for all inputs.

#[path = "../src/generators/smolvm.rs"]
mod smolvm;
#[path = "../src/generators/testlinux.rs"]
mod testlinux;

use proptest::prelude::*;

/// Strategy for valid Rust crate names: lowercase ascii + underscores, 1-30 chars,
/// must start with a letter.
fn arb_crate_name() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,29}".prop_filter("must not be empty", |s| !s.is_empty())
}

mod smolvm_properties {
    use super::*;

    proptest! {
        #[test]
        fn output_is_valid_rust(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            syn::parse_file(&output)
                .map_err(|e| format!("invalid Rust for crate_name={name:?}: {e}"))
                .unwrap();
        }

        #[test]
        fn output_contains_crate_name(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            prop_assert!(
                output.contains(&name),
                "output should contain crate name {name:?}"
            );
        }

        #[test]
        fn output_is_deterministic(name in arb_crate_name()) {
            let a = smolvm::generate(&name);
            let b = smolvm::generate(&name);
            prop_assert_eq!(a, b);
        }

        #[test]
        fn output_is_nonempty(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            prop_assert!(!output.is_empty());
        }

        #[test]
        fn braces_are_balanced(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            prop_assert_eq!(
                output.matches('{').count(),
                output.matches('}').count(),
                "unbalanced braces"
            );
        }

        #[test]
        fn contains_key_api_surface(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            for symbol in &["SmolvmMachine", "free_port", "wait_for_tcp",
                            "smolvm_or_skip", "start_postgres", "start_redis"] {
                prop_assert!(
                    output.contains(symbol),
                    "missing {symbol} in output"
                );
            }
        }

        #[test]
        fn no_raw_format_placeholders(name in arb_crate_name()) {
            let output = smolvm::generate(&name);
            prop_assert!(
                !output.contains("{crate_name}"),
                "leaked raw {{crate_name}} placeholder"
            );
            prop_assert!(
                !output.contains("{{}}"),
                "leaked empty format escape {{}}"
            );
        }
    }
}

mod testlinux_properties {
    use super::*;

    proptest! {
        #[test]
        fn output_is_valid_rust(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            syn::parse_file(&output)
                .map_err(|e| format!("invalid Rust for crate_name={name:?}: {e}"))
                .unwrap();
        }

        #[test]
        fn output_contains_crate_name(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            prop_assert!(
                output.contains(&name),
                "output should contain crate name {name:?}"
            );
        }

        #[test]
        fn output_is_deterministic(name in arb_crate_name()) {
            let a = testlinux::generate(&name);
            let b = testlinux::generate(&name);
            prop_assert_eq!(a, b);
        }

        #[test]
        fn output_is_nonempty(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            prop_assert!(!output.is_empty());
        }

        #[test]
        fn braces_are_balanced(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            prop_assert_eq!(
                output.matches('{').count(),
                output.matches('}').count(),
                "unbalanced braces"
            );
        }

        #[test]
        fn contains_key_api_surface(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            for symbol in &["TestLinux", "Compiler", "InitramfsBuilder",
                            "VmRunner", "MuslCompiler", "CpioInitramfs", "QemuHvf"] {
                prop_assert!(
                    output.contains(symbol),
                    "missing {symbol} in output"
                );
            }
        }

        #[test]
        fn no_raw_format_placeholders(name in arb_crate_name()) {
            let output = testlinux::generate(&name);
            prop_assert!(
                !output.contains("{crate_name}"),
                "leaked raw {{crate_name}} placeholder"
            );
            prop_assert!(
                !output.contains("{{}}"),
                "leaked empty format escape {{}}"
            );
        }
    }
}

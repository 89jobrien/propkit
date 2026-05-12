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
    }
}

//! Integration tests for `propkit generate`

use std::process::Command;

fn propkit_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_propkit"))
}

#[test]
fn generate_dry_run_outputs_proptest_block() {
    let output = propkit_bin()
        .args(["generate", "--dry-run", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("proptest!"),
        "should contain proptest! macro\n{stdout}"
    );
}

#[test]
fn generate_dry_run_contains_hash_eq_test() {
    let output = propkit_bin()
        .args(["generate", "--dry-run", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hash") && stdout.contains("eq"),
        "should generate hash/eq test\n{stdout}"
    );
}

#[test]
fn generate_dry_run_contains_serde_test() {
    let output = propkit_bin()
        .args(["generate", "--dry-run", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("serde"),
        "should generate serde roundtrip test\n{stdout}"
    );
}

#[test]
fn generate_dry_run_no_propkit_dep() {
    let output = propkit_bin()
        .args(["generate", "--dry-run", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("use propkit"),
        "generated tests should not depend on propkit\n{stdout}"
    );
}

#[test]
fn generate_high_confidence_only() {
    let output = propkit_bin()
        .args([
            "generate",
            "--dry-run",
            "--confidence",
            "high",
            "tests/fixtures/sample_crate",
        ])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Low-confidence properties should be excluded
    assert!(
        !stdout.contains("idempotence"),
        "should exclude low-confidence idempotence\n{stdout}"
    );
    assert!(
        !stdout.contains("commutativity"),
        "should exclude low-confidence commutativity\n{stdout}"
    );
}

#[test]
fn generate_prints_cargo_hint() {
    let output = propkit_bin()
        .args(["generate", "--dry-run", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("proptest") && stderr.contains("dev-dependencies"),
        "should print cargo hint about proptest\n{stderr}"
    );
}

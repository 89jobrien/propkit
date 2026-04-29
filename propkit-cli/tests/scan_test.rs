//! Integration tests for `propkit scan`

use std::process::Command;

fn propkit_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_propkit"))
}

#[test]
fn scan_finds_eq_hash_type() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("UserId"),
        "should find UserId type\n{stdout}"
    );
    assert!(
        stdout.contains("hash/eq consistency"),
        "should suggest hash/eq for Eq+Hash\n{stdout}"
    );
}

#[test]
fn scan_finds_ord_type() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Priority"),
        "should find Priority type\n{stdout}"
    );
    assert!(
        stdout.contains("transitivity"),
        "should suggest transitivity for Ord\n{stdout}"
    );
}

#[test]
fn scan_finds_serde_roundtrip() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Config"),
        "should find Config type\n{stdout}"
    );
    assert!(
        stdout.contains("serde roundtrip"),
        "should suggest serde roundtrip for Serialize+Deserialize\n{stdout}"
    );
}

#[test]
fn scan_finds_display_fromstr_roundtrip() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("parse/display roundtrip"),
        "should suggest roundtrip for Display+FromStr\n{stdout}"
    );
}

#[test]
fn scan_finds_default_property() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("default doesn't panic"),
        "should suggest default property\n{stdout}"
    );
}

#[test]
fn scan_finds_encode_decode_roundtrip() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("codec roundtrip"),
        "should suggest codec roundtrip for encode/decode pair\n{stdout}"
    );
}

#[test]
fn scan_finds_sort_property() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("length/element preservation"),
        "should suggest sort properties\n{stdout}"
    );
}

#[test]
fn scan_finds_commutativity_candidate() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("commutativity candidate"),
        "should suggest commutativity for (T, T) -> T\n{stdout}"
    );
}

#[test]
fn scan_finds_idempotence_candidate() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("idempotence candidate"),
        "should suggest idempotence for f(T) -> T\n{stdout}"
    );
}

#[test]
fn scan_prints_summary_line() {
    let output = propkit_bin()
        .args(["scan", "tests/fixtures/sample_crate"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("types,") && stdout.contains("suggested properties"),
        "should print summary\n{stdout}"
    );
}

use std::process::Command;

fn propkit_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_propkit"))
}

#[test]
fn scaffold_smolvm_dry_run_outputs_module() {
    let output = propkit_bin()
        .args(["scaffold", "smolvm", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SmolvmMachine"),
        "should contain SmolvmMachine struct\n{stdout}"
    );
}

#[test]
fn scaffold_testlinux_dry_run_outputs_module() {
    let output = propkit_bin()
        .args(["scaffold", "testlinux", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("TestLinux"),
        "should contain TestLinux struct\n{stdout}"
    );
}

#[test]
fn scaffold_smolvm_contains_free_port() {
    let output = propkit_bin()
        .args(["scaffold", "smolvm", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("free_port"), "missing free_port\n{stdout}");
}

#[test]
fn scaffold_smolvm_contains_skip_macro() {
    let output = propkit_bin()
        .args(["scaffold", "smolvm", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("smolvm_or_skip"),
        "missing skip macro\n{stdout}"
    );
}

#[test]
fn scaffold_smolvm_contains_postgres_fixture() {
    let output = propkit_bin()
        .args(["scaffold", "smolvm", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("start_postgres"),
        "missing postgres fixture\n{stdout}"
    );
}

#[test]
fn scaffold_smolvm_custom_crate_name() {
    let output = propkit_bin()
        .args(["scaffold", "smolvm", "--dry-run", "--crate-name", "myapp"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("myapp"),
        "should include custom crate name\n{stdout}"
    );
}

#[test]
fn scaffold_testlinux_contains_compiler_trait() {
    let output = propkit_bin()
        .args(["scaffold", "testlinux", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("trait Compiler"),
        "missing Compiler trait\n{stdout}"
    );
}

#[test]
fn scaffold_testlinux_contains_qemu_adapter() {
    let output = propkit_bin()
        .args(["scaffold", "testlinux", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("QemuHvf"),
        "missing QemuHvf adapter\n{stdout}"
    );
}

#[test]
fn scaffold_testlinux_contains_initramfs_builder() {
    let output = propkit_bin()
        .args(["scaffold", "testlinux", "--dry-run"])
        .output()
        .expect("failed to run propkit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("CpioInitramfs"),
        "missing CpioInitramfs\n{stdout}"
    );
}

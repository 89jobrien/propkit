# Plan: Scaffold Generators — smolvm + TestLinux

## Goal

Add a `propkit scaffold` subcommand family that generates reusable integration test
infrastructure modules, starting with two generators: `smolvm` (Docker-free VM test
harness) and `testlinux` (cross-compile + QEMU boot test runner).

## Architecture

- Crate affected: `propkit-cli`
- New modules:
  - `src/generators/mod.rs` — generator registry
  - `src/generators/property.rs` — existing property test generator (moved)
  - `src/generators/smolvm.rs` — smolvm test utils generator
  - `src/generators/testlinux.rs` — TestLinux VM runner generator
- CLI: new `Scaffold` subcommand with `--kind smolvm|testlinux`
- Data flow: CLI args → generator function → Rust source string → file write

## Tech Stack

- Rust edition 2024
- Existing deps: clap 4, syn 2, quote 1, walkdir 2
- No new dependencies — generators emit static code templates with variable substitution

## Tasks

### Task 1: Refactor existing generator into generators module

**Crate**: `propkit-cli`
**File(s)**: `propkit-cli/src/generators/mod.rs`, `propkit-cli/src/generators/property.rs`,
`propkit-cli/src/main.rs`
**Run**: `cargo nextest run -p propkit-cli`

1. Create `propkit-cli/src/generators/mod.rs`:

   ```rust
   pub mod property;
   pub mod smolvm;
   pub mod testlinux;
   ```

2. Move `propkit-cli/src/generator.rs` to `propkit-cli/src/generators/property.rs`.
   No content changes — just the file move.

3. Update `propkit-cli/src/main.rs`:
   - Change `mod generator;` to `mod generators;`
   - Update the `generate` handler to use `generators::property::generate_tests`

4. Verify existing tests still pass:

   ```
   cargo nextest run -p propkit-cli    → all green
   cargo clippy -p propkit-cli -- -D warnings  → zero warnings
   ```

5. Run: `git branch --show-current`
   Commit: `git commit -m "refactor(cli): move generator into generators module"`

### Task 2: Add Scaffold subcommand to CLI

**Crate**: `propkit-cli`
**File(s)**: `propkit-cli/src/main.rs`
**Run**: `cargo nextest run -p propkit-cli`

1. Write failing test in `propkit-cli/tests/scaffold_test.rs`:

   ```rust
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
       assert!(output.status.success(), "stderr: {}",
           String::from_utf8_lossy(&output.stderr));
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("SmolvmMachine"),
           "should contain SmolvmMachine struct\n{stdout}");
   }

   #[test]
   fn scaffold_testlinux_dry_run_outputs_module() {
       let output = propkit_bin()
           .args(["scaffold", "testlinux", "--dry-run"])
           .output()
           .expect("failed to run propkit");
       assert!(output.status.success(), "stderr: {}",
           String::from_utf8_lossy(&output.stderr));
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("TestLinux"),
           "should contain TestLinux struct\n{stdout}");
   }
   ```

   Run: `cargo nextest run -p propkit-cli -- scaffold`
   Expected: FAIL (compile error — no `scaffold` subcommand yet)

2. Add the `Scaffold` subcommand to `main.rs`:

   ```rust
   #[derive(Subcommand)]
   enum ScaffoldKind {
       /// Generate smolvm test helpers (RAII VM guard, port utils, fixtures)
       Smolvm {
           /// Print to stdout instead of writing a file
           #[arg(long)]
           dry_run: bool,
           /// Custom output path
           #[arg(short)]
           o: Option<PathBuf>,
           /// Crate name for module header
           #[arg(long, default_value = "my_crate")]
           crate_name: String,
       },
       /// Generate TestLinux VM runner (cross-compile + QEMU boot)
       Testlinux {
           /// Print to stdout instead of writing a file
           #[arg(long)]
           dry_run: bool,
           /// Custom output path
           #[arg(short)]
           o: Option<PathBuf>,
           /// Crate name for module header
           #[arg(long, default_value = "my_crate")]
           crate_name: String,
       },
   }
   ```

   Add to the top-level `Command` enum:

   ```rust
   /// Scaffold reusable test infrastructure modules
   Scaffold {
       #[command(subcommand)]
       kind: ScaffoldKind,
   },
   ```

   Add handler skeleton in `main()`:

   ```rust
   Command::Scaffold { kind } => match kind {
       ScaffoldKind::Smolvm { dry_run, o, crate_name } => {
           let output = generators::smolvm::generate(&crate_name);
           emit_scaffold(output, dry_run, o, "tests/smolvm_helpers.rs");
       }
       ScaffoldKind::Testlinux { dry_run, o, crate_name } => {
           let output = generators::testlinux::generate(&crate_name);
           emit_scaffold(output, dry_run, o, "tests/testlinux.rs");
       }
   },
   ```

   Add `emit_scaffold` helper:

   ```rust
   fn emit_scaffold(output: String, dry_run: bool, o: Option<PathBuf>, default: &str) {
       if dry_run {
           print!("{output}");
       } else {
           let out_path = o.unwrap_or_else(|| PathBuf::from(default));
           if let Some(parent) = out_path.parent() {
               std::fs::create_dir_all(parent).ok();
           }
           std::fs::write(&out_path, &output)
               .unwrap_or_else(|e| eprintln!("error writing {}: {e}", out_path.display()));
           eprintln!("wrote {}", out_path.display());
       }
   }
   ```

3. Create stub generators that return placeholder strings (enough to compile):

   `src/generators/smolvm.rs`:

   ```rust
   pub fn generate(_crate_name: &str) -> String {
       String::new()
   }
   ```

   `src/generators/testlinux.rs`:

   ```rust
   pub fn generate(_crate_name: &str) -> String {
       String::new()
   }
   ```

4. Verify: tests still fail (empty output) but the binary compiles.

5. Commit: `git commit -m "feat(cli): add scaffold subcommand with smolvm + testlinux stubs"`

### Task 3: Implement smolvm generator

**Crate**: `propkit-cli`
**File(s)**: `propkit-cli/src/generators/smolvm.rs`
**Run**: `cargo nextest run -p propkit-cli -- scaffold_smolvm`

1. Implement `generate(crate_name: &str) -> String` that produces a complete Rust module
   containing:

   ```rust
   pub fn generate(crate_name: &str) -> String {
       format!(
           r#"// Generated by propkit — smolvm integration test helpers for {crate_name}
   // These helpers have no dependency on propkit at runtime.
   //
   // Usage: place this file in tests/smolvm_helpers.rs and use as a module:
   //   mod smolvm_helpers;
   //   use smolvm_helpers::{{SmolvmMachine, free_port, smolvm_available}};

   use std::net::TcpListener;
   use std::process::{{Command, Stdio}};
   use std::time::{{Duration, Instant}};

   /// Check whether `smolvm` is available on PATH.
   pub fn smolvm_available() -> bool {{
       Command::new("smolvm")
           .arg("--version")
           .stdout(Stdio::null())
           .stderr(Stdio::null())
           .status()
           .map(|s| s.success())
           .unwrap_or(false)
   }}

   /// Skip the calling test if smolvm is not available.
   #[macro_export]
   macro_rules! smolvm_or_skip {{
       () => {{
           if !$crate::smolvm_helpers::smolvm_available() {{
               eprintln!("SKIP: smolvm not available");
               return;
           }}
       }};
   }}

   /// Find a free TCP port by binding to port 0.
   pub fn free_port() -> u16 {{
       let listener = TcpListener::bind("127.0.0.1:0")
           .expect("failed to bind ephemeral port");
       listener.local_addr().unwrap().port()
   }}

   /// Block until a TCP connection to `addr` succeeds or `timeout` elapses.
   pub fn wait_for_tcp(addr: &str, timeout: Duration) -> bool {{
       let start = Instant::now();
       let interval = Duration::from_millis(100);
       while start.elapsed() < timeout {{
           if std::net::TcpStream::connect(addr).is_ok() {{
               return true;
           }}
           std::thread::sleep(interval);
       }}
       false
   }}

   /// RAII guard for a smolvm machine. Stops and removes the machine on drop.
   pub struct SmolvmMachine {{
       name: String,
       stopped: bool,
   }}

   impl SmolvmMachine {{
       /// Create and start a new smolvm machine from the given image.
       pub fn start(image: &str, name: &str, ports: &[(u16, u16)]) -> Self {{
           let port_args: Vec<String> = ports
               .iter()
               .map(|(host, guest)| format!("-p {{}}:{{}}", host, guest))
               .collect();
           let status = Command::new("smolvm")
               .args(["create", "--name", name, "--image", image])
               .args(&port_args)
               .status()
               .expect("failed to create smolvm machine");
           assert!(status.success(), "smolvm create failed");

           let status = Command::new("smolvm")
               .args(["start", name])
               .status()
               .expect("failed to start smolvm machine");
           assert!(status.success(), "smolvm start failed");

           Self {{
               name: name.to_string(),
               stopped: false,
           }}
       }}

       /// Stop the machine without removing it.
       pub fn stop(&mut self) {{
           if !self.stopped {{
               let _ = Command::new("smolvm")
                   .args(["stop", &self.name])
                   .status();
               self.stopped = true;
           }}
       }}

       /// The machine name.
       pub fn name(&self) -> &str {{
           &self.name
       }}
   }}

   impl Drop for SmolvmMachine {{
       fn drop(&mut self) {{
           self.stop();
           let _ = Command::new("smolvm")
               .args(["rm", "--force", &self.name])
               .status();
       }}
   }}

   /// Start a PostgreSQL instance via smolvm. Returns (machine, connection_url).
   pub fn start_postgres(
       port: u16,
       password: &str,
       timeout: Duration,
   ) -> (SmolvmMachine, String) {{
       let name = format!("propkit-pg-{{}}", port);
       let machine = SmolvmMachine::start("postgres:16", &name, &[(port, 5432)]);
       let addr = format!("127.0.0.1:{{}}", port);
       assert!(
           wait_for_tcp(&addr, timeout),
           "PostgreSQL did not become ready within {{:?}}",
           timeout
       );
       let url = format!(
           "postgres://postgres:{{}}@127.0.0.1:{{}}/postgres",
           password, port
       );
       (machine, url)
   }}

   /// Start a Redis instance via smolvm. Returns (machine, address).
   pub fn start_redis(port: u16, timeout: Duration) -> (SmolvmMachine, String) {{
       let name = format!("propkit-redis-{{}}", port);
       let machine = SmolvmMachine::start("redis:7", &name, &[(port, 6379)]);
       let addr = format!("127.0.0.1:{{}}", port);
       assert!(
           wait_for_tcp(&addr, timeout),
           "Redis did not become ready within {{:?}}",
           timeout
       );
       (machine, addr)
   }}

   /// Start a Redis instance with password authentication.
   pub fn start_redis_with_auth(
       port: u16,
       password: &str,
       timeout: Duration,
   ) -> (SmolvmMachine, String) {{
       let name = format!("propkit-redis-auth-{{}}", port);
       // Redis AUTH is set via the --requirepass flag in the CMD.
       let machine = SmolvmMachine::start(
           &format!("redis:7 --requirepass {{}}", password),
           &name,
           &[(port, 6379)],
       );
       let addr = format!("127.0.0.1:{{}}", port);
       assert!(
           wait_for_tcp(&addr, timeout),
           "Redis (auth) did not become ready within {{:?}}",
           timeout
       );
       (machine, addr)
   }}
   "#,
           crate_name = crate_name,
       )
   }
   ```

2. Verify:

   ```
   cargo nextest run -p propkit-cli -- scaffold_smolvm  → all green
   cargo clippy -p propkit-cli -- -D warnings           → zero warnings
   ```

3. Commit: `git commit -m "feat(cli): implement smolvm scaffold generator"`

### Task 4: Implement testlinux generator

**Crate**: `propkit-cli`
**File(s)**: `propkit-cli/src/generators/testlinux.rs`
**Run**: `cargo nextest run -p propkit-cli -- scaffold_testlinux`

1. Implement `generate(crate_name: &str) -> String` that produces a complete Rust module
   containing:
   - `Compiler` trait with `fn compile(&self, crate_dir, out_dir) -> Result<PathBuf>`
   - `InitramfsBuilder` trait with `fn build(&self, test_binary, out_dir) -> Result<PathBuf>`
   - `VmRunner` trait with `fn run(&self, kernel, initramfs) -> Result<TestResult>`
   - `TestResult` struct with `passed: usize, failed: usize, output: String`
   - `MuslCompiler` struct implementing `Compiler` via `cargo-zigbuild`
   - `CpioInitramfs` struct implementing `InitramfsBuilder`
   - `QemuHvf` struct implementing `VmRunner` with serial sentinel parsing
   - `TestLinux<C, I, V>` orchestrator that wires compile → initramfs → boot → parse
   - `build_run_tests_sh()` helper that generates a shell script for the initramfs init
   - All `Duration` timeouts are configurable parameters, not hardcoded constants

2. Verify:

   ```
   cargo nextest run -p propkit-cli -- scaffold_testlinux  → all green
   cargo clippy -p propkit-cli -- -D warnings              → zero warnings
   ```

3. Commit: `git commit -m "feat(cli): implement testlinux scaffold generator"`

### Task 5: Add scaffold output tests and CLI integration

**Crate**: `propkit-cli`
**File(s)**: `propkit-cli/tests/scaffold_test.rs`
**Run**: `cargo nextest run -p propkit-cli -- scaffold`

1. Add content-validation tests to `scaffold_test.rs`:

   ```rust
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
       assert!(stdout.contains("smolvm_or_skip"),
           "missing skip macro\n{stdout}");
   }

   #[test]
   fn scaffold_smolvm_contains_postgres_fixture() {
       let output = propkit_bin()
           .args(["scaffold", "smolvm", "--dry-run"])
           .output()
           .expect("failed to run propkit");
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("start_postgres"),
           "missing postgres fixture\n{stdout}");
   }

   #[test]
   fn scaffold_smolvm_custom_crate_name() {
       let output = propkit_bin()
           .args(["scaffold", "smolvm", "--dry-run", "--crate-name", "myapp"])
           .output()
           .expect("failed to run propkit");
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("myapp"),
           "should include custom crate name\n{stdout}");
   }

   #[test]
   fn scaffold_testlinux_contains_compiler_trait() {
       let output = propkit_bin()
           .args(["scaffold", "testlinux", "--dry-run"])
           .output()
           .expect("failed to run propkit");
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("trait Compiler"),
           "missing Compiler trait\n{stdout}");
   }

   #[test]
   fn scaffold_testlinux_contains_qemu_adapter() {
       let output = propkit_bin()
           .args(["scaffold", "testlinux", "--dry-run"])
           .output()
           .expect("failed to run propkit");
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("QemuHvf"),
           "missing QemuHvf adapter\n{stdout}");
   }

   #[test]
   fn scaffold_testlinux_contains_initramfs_builder() {
       let output = propkit_bin()
           .args(["scaffold", "testlinux", "--dry-run"])
           .output()
           .expect("failed to run propkit");
       let stdout = String::from_utf8_lossy(&output.stdout);
       assert!(stdout.contains("CpioInitramfs"),
           "missing CpioInitramfs\n{stdout}");
   }
   ```

2. Verify:

   ```
   cargo nextest run -p propkit-cli -- scaffold  → all green
   cargo clippy -p propkit-cli -- -D warnings    → zero warnings
   ```

3. Commit: `git commit -m "test(cli): add scaffold output validation tests"`

### Task 6: Update CLAUDE.md and close issues

**Crate**: `propkit` (workspace root)
**File(s)**: `CLAUDE.md`
**Run**: `cargo nextest run`

1. Update `CLAUDE.md` to document the scaffold subcommand:
   - Add `propkit scaffold smolvm --dry-run` and `propkit scaffold testlinux --dry-run`
     to the Commands section
   - Add generators table entry for smolvm and testlinux

2. Close GitHub issues:

   ```bash
   gh issue close 61 --repo 89jobrien/godmode --comment "Implemented as propkit scaffold smolvm"
   gh issue close 62 --repo 89jobrien/godmode --comment "Implemented as propkit scaffold testlinux"
   ```

3. Verify full workspace:

   ```
   cargo nextest run                → all green
   cargo clippy --all-targets       → zero warnings
   cargo fmt --all --check          → clean
   ```

4. Commit: `git commit -m "docs: add scaffold generators to CLAUDE.md, close #61 #62"`

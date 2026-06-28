# propkit — Agent Operating Guide

propkit is a Rust property-based testing (PBT) toolkit: a library exposing
proptest strategies across 12 domains, plus a CLI tool that scans source code,
recommends property tests, and scaffolds test infrastructure (smolvm, testlinux).

## Build, Lint, and Test Commands

```bash
# Build both crates
cargo build --workspace

# Run all property tests
cargo test -p propkit

# Run tests for a single module (e.g., floats, collections, regex_props)
cargo test -p propkit module_name

# Scaffold test helpers (smolvm or testlinux)
cargo run -p propkit-cli -- scaffold smolvm --dry-run
cargo run -p propkit-cli -- scaffold testlinux --dry-run

# Format and lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Workspace Layout

```
propkit/                 # workspace root
├── Cargo.toml           # [workspace] manifest, edition 2024
├── propkit/             # lib crate: strategies + PBT tests
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs       # module declarations
│       └── strategies/
│           ├── floats.rs           # f32/f64 bounds, NaN/inf, roundtrips
│           ├── collections.rs      # Vec, HashSet, HashMap
│           ├── strings.rs          # UTF-8, char exclusion, alphabet
│           ├── numerics.rs         # int/float commutativity, wrapping
│           ├── permutations.rs     # length, uniqueness, composition
│           ├── regex_props.rs      # pattern matching, anchoring
│           ├── sampling.rs         # membership, weighted selection
│           ├── slices.rs           # Python-style slice semantics
│           ├── complex.rs          # Complex64 magnitude, conjugate
│           ├── composition.rs      # flat_map, map, filter chaining
│           ├── datetimes.rs        # NaiveDate/Time/DateTime
│           ├── recursive.rs        # tree generation, depth bounds
│           └── uuids.rs            # UUID v4 version, roundtrip
└── propkit-cli/         # bin crate: code scanner + generator
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs              # CLI entry (scan, generate, scaffold)
    │   ├── analyzer.rs          # syn-based trait/fn detection
    │   └── generators/
    │       ├── mod.rs
    │       ├── property.rs      # proptest code generation
    │       ├── smolvm.rs        # RAII VM guard, fixtures, helpers
    │       └── testlinux.rs     # Compiler, InitramfsBuilder traits
    └── tests/
        └── scaffold_test.rs     # integration tests
```

## Strategy Modules (lib crate)

| Module         | Coverage                                        |
| -------------- | ----------------------------------------------- |
| `floats`       | f64/f32 bounds, NaN, inf, subnormal, roundtrips |
| `collections`  | Vec/HashSet/HashMap size, uniqueness, nesting   |
| `strings`      | UTF-8, char exclusion, alphabet constraints     |
| `numerics`     | Integer/float commutativity, associativity      |
| `permutations` | Length/element preservation, no duplicates      |
| `regex_props`  | Pattern match-back, char class, alternation     |
| `sampling`     | Membership, weighted selection, uniqueness      |
| `slices`       | Python-style step, bounds, subset properties    |
| `complex`      | Complex64 magnitude, conjugate, triangle ineq   |
| `composition`  | flat_map, map, filter chaining, unions          |
| `datetimes`    | Date/Time/DateTime bounds, duration, leap years |
| `recursive`    | Tree generation, depth bounds, leaf-only        |
| `uuids`        | UUID v4 version/variant, string roundtrip       |

## Code Conventions

- Rust edition 2024
- All tests use `proptest!` macro (not `#[proptest]` attribute)
- Feature flags gate public strategy exports
- `floats.rs` includes stable-Rust `next_up_f64`/`next_down_f64` helpers
- `slices.rs` defines `Slice` struct for Python-style slicing
- `permutations.rs` exposes `arb_permutation(n)` via sorted random keys
- Line width: 100 characters (matches workspace standard)

## CLI Tool (propkit-cli)

### Commands

**scan** — Analyze source code for testable traits/functions:

```bash
cargo run -p propkit-cli -- scan /path/to/src
```

**generate** — Create standalone property test file:

```bash
cargo run -p propkit-cli -- generate module_name --output tests/module_tests.rs
```

**scaffold** — Build test infrastructure (smolvm or testlinux):

```bash
cargo run -p propkit-cli -- scaffold smolvm --output tests/smolvm_helpers.rs
cargo run -p propkit-cli -- scaffold testlinux --output tests/testlinux.rs
```

### Output Generators

| Generator   | Output Path            | Provides                                   |
| ----------- | ---------------------- | ------------------------------------------ |
| `smolvm`    | `tests/smolvm_helpers` | RAII VM guard, `free_port`, `wait_for_tcp` |
| `testlinux` | `tests/testlinux`      | Compiler, InitramfsBuilder, VmRunner       |

## Development Workflow

1. **Run all tests**: `cargo test -p propkit`
2. **Test one module**: `cargo test -p propkit floats`
3. **Lint**: `cargo clippy --all-targets -- -D warnings`
4. **Format**: `cargo fmt --all`
5. **Scaffold helpers**: `cargo run -p propkit-cli -- scaffold smolvm --dry-run`

## Key Dependencies

- **proptest** — property-based testing framework
- **syn** — Rust AST parsing (CLI analyzer)
- **quote** — code generation (CLI generator)
- **tempfile** — temp directory/file isolation (tests)

## Testing Patterns

### Property Tests

All tests in `propkit/src/strategies/` use the `proptest!` macro:

```rust
proptest! {
    #[test]
    fn test_property_name(x in arb_strategy()) {
        // Assertion
    }
}
```

### Integration Tests

CLI integration tests live in `propkit-cli/tests/`:

```bash
cargo test --test scaffold_test
```

## Commit Guidelines

Follow Conventional Commits:

- `feat:` New strategy module or generator
- `fix:` Bug fix in strategy or CLI
- `test:` New test coverage or PBT case
- `docs:` Documentation updates
- `refactor:` Code restructuring

Example: `feat(floats): add subnormal detection strategy`

## Self-Protection Rules

When running tests in containers, avoid:

- Hardcoded absolute paths in test fixtures
- Global state or shared temp directories
- Thread-unsafe randomness seeding

Use injected directories or `tempfile::TempDir` for isolation.

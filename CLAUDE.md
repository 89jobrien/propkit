# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## What This Is

propkit is a Cargo workspace containing:

- **propkit** (lib) -- reusable proptest strategies behind feature flags, plus
  133 property-based tests ported from Python's Hypothesis framework
- **propkit-cli** (bin) -- scans a target crate's source, recommends property
  tests, generates standalone test files, and scaffolds test infrastructure

## Workspace Layout

```
propkit/              # workspace root
  Cargo.toml          # [workspace] manifest
  propkit/            # lib crate
    Cargo.toml
    src/
      lib.rs
      *.rs            # strategy + test modules
  propkit-cli/        # bin crate
    src/
      main.rs         # CLI entry (scan, generate, scaffold)
      analyzer.rs     # syn-based trait/fn detection
      generators/
        mod.rs
        property.rs   # proptest code generation
        smolvm.rs     # smolvm test helpers scaffold
        testlinux.rs  # TestLinux VM runner scaffold
    tests/
      scaffold_test.rs
```

### Scaffold Generators

| Generator   | Output                    | What it produces                                        |
| ----------- | ------------------------- | ------------------------------------------------------- |
| `smolvm`    | `tests/smolvm_helpers.rs` | RAII VM guard, free_port, wait_for_tcp, DB fixtures     |
| `testlinux` | `tests/testlinux.rs`      | Compiler/InitramfsBuilder/VmRunner traits, orchestrator |

## Commands

```bash
cargo test                                          # run all property tests
cargo test -p propkit floats                        # run a single module
cargo nextest run -p propkit-cli -- scaffold         # scaffold tests only
cargo run -p propkit-cli -- scaffold smolvm --dry-run     # preview smolvm helpers
cargo run -p propkit-cli -- scaffold testlinux --dry-run  # preview testlinux runner
cargo fmt --all                                     # format
cargo clippy --all-targets                          # lint
```

## Architecture

The lib crate contains test modules in `propkit/src/`, each covering a domain
of universal PBT properties. Several modules expose public strategies behind
feature flags (see design spec for details).

| Module            | What it tests                                                   |
| ----------------- | --------------------------------------------------------------- |
| `floats.rs`       | f64/f32 bounds, NaN/inf, subnormal, next_up/next_down roundtrip |
| `collections.rs`  | Vec/HashSet/HashMap size, uniqueness, nesting                   |
| `strings.rs`      | Alphabet constraints, UTF-8, char exclusion, ASCII-only         |
| `numerics.rs`     | Integer/float bounds, commutativity, associativity, wrapping    |
| `permutations.rs` | Length/element preservation, no duplicates, composition         |
| `regex_props.rs`  | Pattern match-back, char class, anchoring, alternation          |
| `sampling.rs`     | Membership, filtered correctness, uniqueness, weighted          |
| `slices.rs`       | Python-style slice semantics: step != 0, bounds, subset         |
| `complex.rs`      | Complex64 magnitude, conjugate, arithmetic, triangle inequality |
| `composition.rs`  | flat_map, map, filter chaining, ordered pairs, unions           |
| `datetimes.rs`    | NaiveDate/Time/DateTime bounds, duration, leap years            |
| `recursive.rs`    | Tree generation, depth bounds, leaf-only, size limits           |
| `uuids.rs`        | UUID v4 version/variant, string roundtrip, uniqueness           |

## Conventions

- Rust edition 2024.
- All tests use the `proptest!` macro, not `#[proptest]` attribute.
- `floats.rs` includes stable-Rust `next_up_f64`/`next_down_f64` helpers
  since the std methods are nightly-only.
- `slices.rs` defines a `Slice` struct for Python-style slicing.
- `permutations.rs` defines `arb_permutation(n)` -- custom strategy via
  sorted random keys.

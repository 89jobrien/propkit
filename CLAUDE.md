# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

propkit is a test-only Rust crate containing 99 proptest property-based tests ported from
Python's Hypothesis framework (hypothesis-python/tests/cover + nocover). It has no library
code -- all modules are `#[cfg(test)]` only.

## Commands

```bash
cargo test                         # run all 99 property tests
cargo test floats                  # run a single module
cargo test floats::up_means_greater # run a single test
cargo fmt --all                    # format (pre-commit hook enforces this)
cargo clippy                       # lint
```

## Architecture

The crate is a flat set of test modules in `src/`, each covering a domain of universal
PBT properties. There is no public API -- this is a test suite, not a library.

| Module            | What it tests                                                                              |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `floats.rs`       | f64/f32 bounds, NaN/inf filtering, subnormal control, next_up/next_down roundtrip          |
| `collections.rs`  | Vec/HashSet/HashMap size bounds, uniqueness, uniqueness-by-key, nesting                    |
| `strings.rs`      | Alphabet constraints, UTF-8 validity, char exclusion, ASCII-only                           |
| `numerics.rs`     | Integer/float bounds, commutativity, associativity, division identity, wrapping/saturating |
| `permutations.rs` | Length/element preservation, no duplicates, composition, sort recovery                     |
| `regex_props.rs`  | Pattern match-back, char class membership, anchoring, alternation                          |
| `sampling.rs`     | Membership, filtered correctness, uniqueness, weighted sampling                            |
| `slices.rs`       | Python-style slice semantics: step != 0, no-panic application, bounds, result subset       |
| `composition.rs`  | prop_flat_map, prop_map, prop_filter chaining, ordered pairs, constant lists, unions       |

## Conventions

- Rust edition 2024.
- All tests use the `proptest!` macro, not `#[proptest]` attribute.
- `floats.rs` includes stable-Rust `next_up_f64`/`next_down_f64` helpers (bit manipulation)
  since the std methods are nightly-only.
- `slices.rs` defines a `Slice` struct implementing Python-style slicing for test purposes.
- `permutations.rs` defines `arb_permutation(n)` -- a custom strategy that generates
  permutations of `0..n` by sorting random keys.

## Adding New Modules

1. Create `src/<domain>.rs` with `use proptest::prelude::*;` and a `proptest! {}` block.
2. Add `#[cfg(test)] mod <domain>;` to `src/lib.rs`.
3. If new dev-dependencies are needed, add to `[dev-dependencies]` in Cargo.toml.

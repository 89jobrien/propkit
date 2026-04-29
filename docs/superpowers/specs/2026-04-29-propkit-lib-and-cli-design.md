# propkit: Strategy Library + CLI Scanner Design

**Date:** 2026-04-29
**Status:** Approved

## Summary

Transform propkit from a test-only crate into two independent artifacts:

1. **propkit** (lib) — reusable proptest strategies behind feature flags
2. **propkit-cli** (bin) — scans a target crate's source, recommends property
   tests, and optionally generates standalone test files

## Workspace Structure

```
propkit/
  Cargo.toml          # workspace root
  propkit/             # lib crate
    Cargo.toml
    src/
      lib.rs
      strategies/      # public strategy modules
      tests/           # existing 133 property tests (integration tests)
  propkit-cli/         # bin crate
    Cargo.toml
    src/
      main.rs
```

## propkit (Library)

### Feature Flags

| Feature      | Enables                | Deps pulled          |
| ------------ | ---------------------- | -------------------- |
| (none)       | Empty lib, no deps     | —                    |
| `strategies` | Core strategy modules  | `proptest`           |
| `chrono`     | Date/time strategies   | `proptest`, `chrono` |
| `regex`      | Regex helper functions | `proptest`, `regex`  |
| `uuid`       | UUID strategies        | `proptest`, `uuid`   |

All domain features imply `strategies` which implies `proptest`.

### Public Strategy Modules

Only modules with custom strategies get a public API. Modules that only
contain `proptest!` blocks using built-in strategies remain as test-only
examples.

| Module         | Public API                                                                                            | Feature      |
| -------------- | ----------------------------------------------------------------------------------------------------- | ------------ |
| `floats`       | `next_up_f64`, `next_down_f64`                                                                        | `strategies` |
| `permutations` | `arb_permutation(n)`                                                                                  | `strategies` |
| `slices`       | `Slice`, `arb_slice(size)`                                                                            | `strategies` |
| `recursive`    | `Tree`, `arb_tree()`, `arb_leaf_only()`                                                               | `strategies` |
| `datetimes`    | `arb_naive_date`, `arb_naive_time`, `arb_naive_datetime`, `arb_duration_small`, `arb_duration_nonneg` | `chrono`     |
| `regex_props`  | `re_digits`, `re_word`, `re_hex`, `re_email`, `re_ipv4`                                               | `regex`      |
| `uuids`        | `arb_uuid_bytes`, `arb_uuid_v4`, `re_uuid`                                                            | `uuid`       |

Modules with no custom strategies (numerics, collections, strings,
sampling, composition) are not exposed publicly. Their property tests
serve as examples in the test suite.

### Usage

```rust
use propkit::strategies::datetimes::arb_naive_date;
use propkit::strategies::permutations::arb_permutation;

proptest! {
    #[test]
    fn my_test(d in arb_naive_date()) { /* ... */ }
}
```

## propkit-cli (Binary)

### Commands

- `propkit scan <path>` — analyze source files, print property test
  recommendations to stdout
- `propkit generate <path>` — write standalone test file to
  `<path>/tests/propkit_properties.rs`

### Scanning Mechanism

Source parsing via `syn` (no compilation required). Two analysis passes:

#### Trait-Based Analysis

Reads `#[derive(...)]` attributes and `impl Trait for Type` blocks.

| Detected traits          | Suggested properties                 |
| ------------------------ | ------------------------------------ |
| `Eq` + `Hash`            | hash/eq consistency                  |
| `Ord`                    | transitivity, antisymmetry, totality |
| `PartialOrd, PartialEq`  | reflexivity, antisymmetry            |
| `Clone`                  | clone equality                       |
| `Serialize, Deserialize` | serde roundtrip                      |
| `Display` + `FromStr`    | parse/display roundtrip              |
| `Default`                | default doesn't panic                |
| `Add, Sub, Mul`          | commutativity, identity elements     |

#### Signature-Based Analysis

Pattern-matches public function signatures.

| Signature pattern                       | Property                    |
| --------------------------------------- | --------------------------- |
| `fn foo(T) -> T`                        | idempotence candidate       |
| `fn parse(&str) -> Result<T>` + display | roundtrip                   |
| `fn from(A) -> B` + `fn from(B) -> A`   | conversion roundtrip        |
| `fn sort(&mut [T])` / `-> Vec<T>`       | length/element preservation |
| `fn contains` + `fn insert`             | membership after insert     |
| `fn encode(T) -> bytes` + `fn decode`   | codec roundtrip             |
| `fn (T, T) -> T`                        | commutativity candidate     |

### Confidence Levels

- **high** — derive-based properties (universal, always correct)
- **medium** — signature-matched with naming alignment
- **low** — structural matches only (may not apply)

### Scan Output Format

```
src/lib.rs:
  MyStruct (derives: Eq, Hash, Clone, Serialize, Deserialize)
    - [high] hash/eq consistency
    - [high] clone equality
    - [high] serde roundtrip

  Price (derives: PartialOrd, PartialEq)
    - [high] partial_ord reflexivity
    - [high] partial_ord antisymmetry

  parse_price(&str) -> Result<Price, Error>
    - [medium] roundtrip with Display impl

  2 types, 1 function, 6 suggested properties
```

### Code Generation

Generated tests are **standalone** — they depend only on `proptest`, not
on propkit. Users can delete propkit after generating.

**Strategy selection:**

- Type derives `Arbitrary` → use `any::<T>()`
- Otherwise → TODO placeholder with `prop_compose!` skeleton

**CLI flags:**

- `--dry-run` — print to stdout, don't write
- `--append` — add to existing file
- `--confidence high` — only high-confidence properties (default: high+medium)
- `--exclude <type>` — skip specific types
- `-o <path>` — custom output path

**Cargo.toml hint:** if proptest is not in dev-dependencies, print:
`note: add proptest to dev-dependencies: cargo add --dev proptest`

## Dependencies

### propkit (lib)

```toml
[features]
default = []
strategies = ["dep:proptest"]
chrono = ["strategies", "dep:chrono"]
regex = ["strategies", "dep:regex"]
uuid = ["strategies", "dep:uuid"]

[dependencies]
proptest = { version = "1", optional = true }
chrono = { version = "0.4", optional = true }
regex = { version = "1", optional = true }
uuid = { version = "1", features = ["v4"], optional = true }
```

### propkit-cli (bin)

```toml
[dependencies]
syn = { version = "2", features = ["full", "visit"] }
quote = "1"
proc-macro2 = "1"
clap = { version = "4", features = ["derive"] }
walkdir = "2"
```

`cargo install propkit-cli` pulls zero proptest/chrono/regex/uuid deps.

## Existing Tests

The 133 property tests remain as integration tests in the lib crate.
They validate the strategies and serve as usage examples. No tests are
deleted.

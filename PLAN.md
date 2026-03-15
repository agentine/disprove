# disprove — Property-Based Testing for Rust

**Replaces:** [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck)
**Crate name:** `disprove` (verified available on crates.io)
**License:** MIT OR Apache-2.0 (dual-license, Rust convention)
**MSRV:** Rust 1.75+ (2024 edition ready)

## Why

quickcheck is the foundational property-based testing crate for Rust:
- 43.8M direct downloads on crates.io
- 52 major crate dependents (9.9B transitive downloads)
- 2.7k GitHub stars, 160 forks
- **Abandoned for 5+ years** (last release v1.0.3, ~Feb 2021)
- 19 open issues with no maintainer activity
- Single maintainer (BurntSushi) no longer active on this project

No maintained drop-in replacement exists:
- **proptest**: Different API (strategy-based vs trait-based), not drop-in compatible. Itself in "passive maintenance" only.
- **qcheck**: Fork with 0 stars, 0 forks — no ecosystem adoption
- **quickcheck2**: Minimal adoption

Known quickcheck issues unaddressed:
- No async test support
- Debug repr overhead (41% of test runtime in some cases)
- Integer negation stack overflow edge case
- No const generics support for arrays
- Missing `Arbitrary` implementations for newer std types
- No `#[no_std]` support

## Scope

Property-based testing library: generate random inputs, run property functions, shrink failing inputs to minimal counterexamples. API-compatible with quickcheck to enable drop-in migration.

## Architecture

### Core Crate: `disprove`

```
src/
  lib.rs           — public API, re-exports
  arbitrary.rs     — Arbitrary trait + built-in impls
  gen.rs           — Gen (random value generator with size parameter)
  shrink.rs        — shrink iterators and combinators
  tester.rs        — QuickCheck/TestResult types, test runner
  error.rs         — error types and formatting
```

### Proc Macro Crate: `disprove_macros`

```
disprove_macros/
  src/lib.rs       — #[quickcheck] proc macro
```

### Compat Crate: `disprove/compat`

```
compat/
  src/lib.rs       — re-exports under quickcheck names
```

## Major Components

### 1. Arbitrary Trait
Core trait for generating random values. API-compatible with quickcheck's `Arbitrary`:

```rust
pub trait Arbitrary: Clone + Debug + 'static {
    fn arbitrary(g: &mut Gen) -> Self;
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        empty_shrinker()
    }
}
```

Built-in implementations for:
- Primitives: `bool`, `u8`–`u128`, `i8`–`i128`, `f32`, `f64`, `char`
- Collections: `Vec<T>`, `HashMap<K,V>`, `HashSet<T>`, `BTreeMap<K,V>`, `BTreeSet<T>`, `VecDeque<T>`, `LinkedList<T>`, `BinaryHeap<T>`
- Smart pointers: `Box<T>`, `Rc<T>`, `Arc<T>`, `Cell<T>`, `RefCell<T>`
- Strings: `String`, `CString`, `OsString`, `PathBuf`
- Option/Result: `Option<T>`, `Result<T, E>`
- Tuples: up to 12-element tuples
- Arrays: `[T; N]` via const generics (improvement over quickcheck)
- Non-zero types: `NonZeroU8`–`NonZeroU128`, `NonZeroI8`–`NonZeroI128`
- Ranges: `Range<T>`, `RangeInclusive<T>`
- Wrapping types: `Wrapping<T>`, `Saturating<T>`
- Duration, Ipv4Addr, Ipv6Addr, SocketAddr

### 2. Gen (Generator)
Random value generator with configurable size parameter:

```rust
pub struct Gen {
    rng: SmallRng,
    size: usize,
}
```

Size controls complexity of generated values (e.g., max length of vectors, magnitude of integers). Compatible with quickcheck's `Gen` API.

### 3. Shrinking
When a property fails, shrink the input to find the minimal failing case:
- Integers shrink toward 0
- Vectors shrink by removing elements and shrinking remaining
- Strings shrink by removing characters
- Tuples shrink component-wise
- Custom shrink via `Arbitrary::shrink()`

Improvement: lazy shrinking with `Iterator` (same as quickcheck) but with better performance for common cases.

### 4. Test Runner

```rust
pub struct QuickCheck {
    tests: u64,
    max_tests: u64,
    min_tests_passed: u64,
    gen: Gen,
}
```

Supports:
- `quickcheck(property_fn)` — run with defaults
- `QuickCheck::new().tests(1000).quickcheck(property_fn)` — configurable
- `TestResult` — pass, fail, discard, with optional error messages
- Deterministic replay via seed

### 5. Proc Macro: `#[quickcheck]`
Attribute macro for test functions:

```rust
#[quickcheck]
fn prop_reverse_reverse(xs: Vec<i32>) -> bool {
    let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
    rev == xs
}
```

Desugars to a `#[test]` function that calls the QuickCheck runner.

### 6. Compatibility Layer: `disprove/compat`
Re-exports all types under `quickcheck` module names for zero-friction migration:

```rust
// In Cargo.toml: quickcheck = { package = "disprove", version = "..." }
// Or use the compat feature
pub use disprove::{Arbitrary, Gen, QuickCheck, TestResult};
```

## Improvements Over quickcheck

1. **Const generics**: `[T; N]` for any N (quickcheck only supports up to `[T; 32]`)
2. **Async support**: `#[quickcheck(async)]` for async property functions
3. **Better shrinking performance**: avoid debug repr overhead during shrinking
4. **More Arbitrary impls**: newer std types, NonZero types, IP addresses, Duration
5. **Deterministic seeds**: `QUICKCHECK_SEED` env var for reproducible failures
6. **`#[no_std]` support**: core-only mode with `default-features = false`
7. **Rust 2024 edition**: modern idioms, MSRV 1.75+

## Dependencies

- `rand` (random number generation)
- `env_logger` — NO, use `QUICKCHECK_*` env vars directly
- Zero other runtime dependencies

## Deliverables

1. `disprove` core crate with Arbitrary trait, Gen, shrinking, test runner
2. `disprove_macros` proc macro crate with `#[quickcheck]` attribute
3. Comprehensive Arbitrary implementations for all std types
4. Compatibility guide for quickcheck migration
5. Test suite validating compatibility with quickcheck behavior
6. Published on crates.io

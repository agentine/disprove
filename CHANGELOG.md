# Changelog

## 0.1.0 — 2026-03-16

Initial release.

### Features

- **Arbitrary trait** with `arbitrary()` and `shrink()` for generating and shrinking random test inputs
- **Gen** random value generator backed by `SmallRng`, with `from_seed`, `choose`, `gen_range`, `gen_bool`
- **Primitive impls** for all integer types, floats, `bool`, `char`, `()`
- **Collection impls** for `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`, `VecDeque`, `LinkedList`, `BinaryHeap`
- **String/path impls** for `String`, `CString`, `OsString`, `PathBuf`
- **Smart pointer impls** for `Box`, `Rc`, `Arc`, `Cell`, `RefCell`
- **Wrapper impls** for `Option`, `Result`, tuples (1-12), arrays (const generics), `NonZero` types, `Range`, `RangeInclusive`, `Wrapping`, `Saturating`, `Duration`, IP/socket address types
- **TestResult** with `Pass`, `Fail`, `Discard` variants and conversions from `bool`, `()`, `Result`
- **QuickCheck test runner** with builder pattern, configurable via `QUICKCHECK_TESTS`, `QUICKCHECK_GENERATOR_SIZE`, `QUICKCHECK_SEED` env vars
- **Testable trait** supporting functions with 1-8 arguments
- **Deterministic seed replay** for reproducing failures
- **Shrinking** on failure to find minimal counterexamples
- **`#[quickcheck]` proc macro** for annotating test functions (1-8 args)
- **Compatibility module** with `quickcheck` re-exports for drop-in migration
- **`no_std` support** via default feature flags
- Dual-licensed under MIT and Apache-2.0

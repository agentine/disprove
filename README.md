# disprove

Property-based testing for Rust — a modern, maintained replacement for [quickcheck](https://github.com/BurntSushi/quickcheck).

## Features

- **API-compatible** with quickcheck for easy migration
- **Const generics** support for arrays of any size
- **Deterministic seeds** via `QUICKCHECK_SEED` for reproducible failures
- **More Arbitrary impls** for newer std types
- **`#[quickcheck]` proc macro** for ergonomic test definitions
- **Rust 1.75+ MSRV**

## Usage

```rust
use disprove::{quickcheck, Arbitrary, Gen};

fn prop_sort_is_idempotent(mut xs: Vec<i32>) -> bool {
    xs.sort();
    let sorted = xs.clone();
    xs.sort();
    xs == sorted
}

#[test]
fn test_sort() {
    quickcheck(prop_sort_is_idempotent as fn(Vec<i32>) -> bool);
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

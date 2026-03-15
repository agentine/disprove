//! # disprove
//!
//! Property-based testing for Rust — a modern, maintained replacement for
//! [quickcheck](https://github.com/BurntSushi/quickcheck).
//!
//! Generate random inputs, run property functions, and automatically shrink
//! failing inputs to minimal counterexamples.
//!
//! ## Quick Start
//!
//! ```rust
//! use disprove::{quickcheck, Arbitrary, Gen};
//!
//! fn prop_reverse_reverse(xs: Vec<i32>) -> bool {
//!     let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
//!     rev == xs
//! }
//!
//! #[test]
//! fn test_reverse() {
//!     quickcheck(prop_reverse_reverse as fn(Vec<i32>) -> bool);
//! }
//! ```

mod arbitrary;
mod arbitrary_std;
mod error;
mod gen;
mod shrink;
mod tester;

pub use arbitrary::Arbitrary;
pub use gen::Gen;
pub use shrink::empty_shrinker;
pub use tester::{
    quickcheck, testable_args1, testable_args2, testable_args3, testable_args4, testable_args5,
    testable_args6, testable_args7, testable_args8, QuickCheck, TestResult, Testable,
};

#[cfg(feature = "macros")]
pub use disprove_macros::quickcheck as quickcheck_macro;

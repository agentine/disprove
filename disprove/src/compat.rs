//! Compatibility module for quickcheck migration.
//!
//! This module re-exports all public types so that existing code
//! using `quickcheck` can migrate with minimal changes.
//!
//! # Migration Strategy
//!
//! In `Cargo.toml`, use the package rename:
//! ```toml
//! [dependencies]
//! quickcheck = { package = "disprove", version = "0.1" }
//! ```
//!
//! Or use this compat module:
//! ```rust
//! use disprove::compat::*;
//! ```

pub use crate::arbitrary::Arbitrary;
pub use crate::gen::Gen;
pub use crate::shrink::{chain, empty_shrinker};
pub use crate::tester::{quickcheck, QuickCheck, TestResult, Testable};

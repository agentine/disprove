use std::fmt::Debug;

use crate::gen::Gen;
use crate::shrink::empty_shrinker;

/// Trait for generating arbitrary values and shrinking them.
///
/// Types implementing `Arbitrary` can produce random instances via
/// [`arbitrary`](Arbitrary::arbitrary) and optionally provide a
/// [`shrink`](Arbitrary::shrink) method to reduce failing inputs.
pub trait Arbitrary: Clone + Debug + 'static {
    /// Generate an arbitrary value using the given generator.
    fn arbitrary(g: &mut Gen) -> Self;

    /// Return an iterator of values that are "smaller" than `self`.
    ///
    /// Used to find minimal counterexamples when a property fails.
    /// The default implementation returns an empty iterator.
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        empty_shrinker()
    }
}

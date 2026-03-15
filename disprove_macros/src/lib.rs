extern crate proc_macro;

use proc_macro::TokenStream;

/// Attribute macro for property-based tests.
///
/// Transforms a function into a `#[test]` that runs the property
/// through the QuickCheck test runner.
///
/// # Example
///
/// ```ignore
/// use disprove::quickcheck_macro as quickcheck;
///
/// #[quickcheck]
/// fn prop_reverse(xs: Vec<i32>) -> bool {
///     let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
///     rev == xs
/// }
/// ```
#[proc_macro_attribute]
pub fn quickcheck(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Placeholder — full implementation in Phase 5
    item
}

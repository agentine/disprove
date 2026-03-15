extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Attribute macro for property-based tests.
///
/// Transforms a function with [`Arbitrary`] arguments into a `#[test]`
/// that runs the property through the QuickCheck test runner.
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
///
/// The macro desugars the above into:
///
/// ```ignore
/// #[test]
/// fn prop_reverse() {
///     fn prop(xs: Vec<i32>) -> bool {
///         let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
///         rev == xs
///     }
///     disprove::QuickCheck::new().quickcheck(
///         disprove::testable_args1(prop)
///     );
/// }
/// ```
#[proc_macro_attribute]
pub fn quickcheck(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_output = &input.sig.output;

    let args: Vec<_> = input.sig.inputs.iter().collect();

    if input.sig.asyncness.is_some() {
        return syn::Error::new_spanned(
            input.sig.asyncness,
            "#[quickcheck] does not yet support async functions",
        )
        .to_compile_error()
        .into();
    }

    if args.is_empty() {
        // Zero-arg: just wrap in #[test] and call QuickCheck
        let expanded = quote! {
            #[test]
            #fn_vis fn #fn_name() {
                fn prop() #fn_output #fn_block
                ::disprove::QuickCheck::new().quickcheck(prop);
            }
        };
        return expanded.into();
    }

    // Extract argument names and types
    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();
    for arg in &args {
        match arg {
            syn::FnArg::Typed(pat_type) => {
                arg_names.push(&*pat_type.pat);
                arg_types.push(&*pat_type.ty);
            }
            syn::FnArg::Receiver(_) => {
                return syn::Error::new_spanned(arg, "#[quickcheck] cannot be used on methods")
                    .to_compile_error()
                    .into();
            }
        }
    }

    let n_args = arg_names.len();

    // Select the appropriate testable_args wrapper
    let wrapper_name = match n_args {
        1 => quote!(::disprove::testable_args1),
        2 => quote!(::disprove::testable_args2),
        3 => quote!(::disprove::testable_args3),
        4 => quote!(::disprove::testable_args4),
        5 => quote!(::disprove::testable_args5),
        6 => quote!(::disprove::testable_args6),
        7 => quote!(::disprove::testable_args7),
        8 => quote!(::disprove::testable_args8),
        _ => {
            return syn::Error::new_spanned(
                &input.sig,
                "#[quickcheck] supports at most 8 arguments",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        #[test]
        #fn_vis fn #fn_name() {
            fn prop(#(#arg_names: #arg_types),*) #fn_output #fn_block
            ::disprove::QuickCheck::new().quickcheck(
                #wrapper_name(prop)
            );
        }
    };

    expanded.into()
}

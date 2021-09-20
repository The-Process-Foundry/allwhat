#![feature(proc_macro_diagnostic)]
//! Support macros that implement uses of the allwhat aggregators

pub(crate) mod map_struct;

#[proc_macro]
/// Assign fields of a struct using closures or existing results, aggregating errors
pub fn map_struct(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  map_struct::run_macro(proc_macro2::TokenStream::from(item)).into()
}

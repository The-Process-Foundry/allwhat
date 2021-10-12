//! Map a set of result values into a struct initialization
//!
//! Create each field individually, gather all the errors, and if none of the errors are fatal
//! assign the fields directly to a struct. This is fairly useful when one has a diverse set of
//! independent fields to assign. The two big examples that come to mind is command line interfaces
//! and deserialization. Generally these are done with "fail-first" mentality, where only the first
//! problem found is reported, even though more than one field can fail validation simultaneously.
//! This allows us to return all the errors found instead of the painful fix one and test again
//! method of repairing the inputs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

mod ast;
use ast::{Expr, Questionable, Unwrapped};

use ymlog::ymlog;
// use super::ast::AssignmentRoot;

/// Run the bulk_try macro
pub fn run_macro(input: TokenStream) -> TokenStream {
  ymlog!("_+" => "Starting the macro");
  let parsed: Expr = match syn::parse2(input) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return err.to_compile_error(),
  };

  let error_span = parsed.span().unwrap();

  let Unwrapped { try_count, expr } = match parsed.unwrap_tries() {
    Ok(res) => res,
    Err(err) => return err.to_compile_error(),
  };

  // A warning if there were no tries found anywhere
  //
  //     warning: No accessable tries found. Are you sure this is necessary?
  //       --> src/main.rs:10:16
  //        |
  //     10 |
  //        |                ^^^
  if try_count == 0 {
    error_span
      .warning("No accessable tries found. Are you sure this is necessary?")
      .emit();
  }

  // And we wrap up the final result based on everything found
  quote! {{
    let mut __error_group = ErrorGroup::new(Some("Bulk Try Aggregation".to_string()));

    let expr = {
      #expr
    };

    match __error_group.len() > 0 {
      true => Err(__error_group),
      false => Ok(expr.unwrap()),
    }
  }}
}

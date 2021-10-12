//! Various tests for the bulk_try macro
//!
//! These require investigating each variant of syn::Expr, so to reduce the noise I'm separating
//! them all out into separate files.

// pub mod models;

mod expr_call;
mod expr_field;
mod expr_struct;

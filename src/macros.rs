//! Some additional Result based macros
//!
//! Quick ways of removing boilerplate when errors may be created

pub use anyhow::{anyhow, Result};

pub mod all {
  pub use crate::{err, extract_errors, map_struct};
}

/// Returns a SubparError with context".to_string()
///
/// This syntax was taken from https://docs.rs/anyhow/1.0.43/anyhow/macro.anyhow.html
#[macro_export]
macro_rules! err {
  /*
  ($kind:expr) => {
    SubparError::new($kind, anyhow!($kind))
  };
  ($kind:expr, source => $err:ident) => {
    SubparError::new($kind, anyhow!($err))
  };
  ($kind:expr, ctx => $($terms:expr),+) => {
    Err(err!($kind).ctx(format!($($terms, )*)))
  };
  ($kind:expr, source => $err:ident, ctx => $($terms:expr),+) => {
    Err(err!($kind, $err).ctx(format!($($terms, )*)))
  };
  */
  ($kind:expr) => {
    anyhow!($kind)
  };
  ($kind:expr, ctx => $($terms:expr),+) => {
    Err(err!($kind).context(format!($($terms, )*)))
  };
}

/// Maps an iterator to a function and returns a SplitResult, a tuple containing a Vec<T> and
/// Vec<Error>
///
/// Example
/// let result: SplitResult<T, Error> {
///
/// }
#[macro_export]
macro_rules! split_result {
  () => {};
}

/// Assign fields of a struct using closures or existing results, aggregating errors
///
/// This is pretty standard in a proc_macro derive, but checking each value and then adding it gets
/// tedious fast in large structs.
#[macro_export]
macro_rules! map_struct {
  ($struct:ident {($($field:ident $(=> $source:expr )?),+) }) => {{}};
}

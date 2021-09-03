//! Tools for playing with specialized results

mod macros;
mod monadic;

#[cfg(feature = "groups")]
mod group;

pub mod exports {
  pub use {super::monadic::Monadic, crate::kc};

  #[cfg(feature = "groups")]
  pub use super::group::ErrorGroup;
}

use core::fmt::Display;

#[cfg(test)]
mod tools {
  use anyhow::Result;

  /// Test that the internals of errors are ok
  pub fn cmp<T: Eq>(left: Result<T>, right: Result<T, &str>) -> bool {
    match (left, right) {
      (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
      (Ok(l), Ok(r)) => l == r,
      (Err(l), Err(r)) => match l.to_string().eq(&r.to_string()) {
        true => true,
        false => {
          println!("No Match:\n\tEval:     '{}'\n\tExpected: '{}'", l, r);
          false
        }
      },
    }
  }
}

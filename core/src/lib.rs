//! Tools for playing with specialized results

mod monadic;

// #[cfg(feature = "groups")]
mod group;
pub use group::ErrorGroup;

// #[cfg(feature = "groups")]
mod split;

mod batch;

#[cfg(feature = "macros")]
pub use allwhat_macros::*;

// #[cfg(feature = "try_mut")]
// mod try_mut;

/// Standard items used in components of this crate
pub(crate) mod local {
  pub(crate) use std::fmt::{Debug, Display, Formatter};

  pub(crate) use anyhow::{anyhow, Error as AnyhowError};

  pub(crate) use super::group::ErrorGroup;

  pub use super::prelude::*;
}

pub mod prelude {
  pub use {super::monadic::Monadic, crate::extract_errors};

  // #[cfg(feature = "groups")]
  pub use super::{
    batch::BatchResult,
    group::{ErrorGroup, Grouper},
    split::SplitResult,
  };
}

// fn test_macro_defunct() {
//   pub use prelude::*;

//   struct TestMap {
//     pub int_maybe: i32,
//     pub int_sure: i32,
//     pub float_maybe: f32,
//     pub float_sure: f32,
//   }

//   let map: Result<TestMap, ErrorGroup> = bulk_try! {
//     TestMap {
//       float_maybe: 3.14159,
//       int_maybe: 42,
//       float_sure: 1.602,
//       int_sure: 82
//     }
//   };

//   assert!(false)
// }

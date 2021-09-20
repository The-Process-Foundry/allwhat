//! Tools for playing with specialized results

mod monadic;

// #[cfg(feature = "groups")]
mod group;

// #[cfg(feature = "groups")]
mod split;

mod batch;

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

  #[cfg(feature = "macros")]
  pub use allwhat_macros::*;
}

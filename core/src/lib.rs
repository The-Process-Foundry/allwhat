//! Tools for playing with specialized results

// TODO: Is this a concrete error, or a set of traits used by a derive
// mod base;

mod monadic;

// #[cfg(feature = "groups")]
mod group;

// #[cfg(feature = "groups")]
mod split;

mod batch;

#[cfg(feature = "macros")]
pub use allwhat_macros::*;

/// Export the basics
pub use crate::{batch::BatchResult, group::ErrorGroup, split::SplitResult};

// #[cfg(feature = "try_mut")]
// mod try_mut;

/// Standard items used in components of this crate
pub(crate) mod local {
  pub use super::prelude::*;

  pub(crate) use std::fmt::{Debug, Display, Formatter};
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

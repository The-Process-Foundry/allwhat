//! Tools for playing with specialized results

mod macros;
mod monadic;

// #[cfg(feature = "groups")]
mod group;

// #[cfg(feature = "groups")]
mod split;

mod batch;

#[cfg(feature = "try_mut")]
mod try_mut;

/// Standard items used in components of this crate
pub(crate) mod local {
  pub(crate) use std::fmt::{Debug, Display, Formatter};

  pub(crate) use anyhow::{anyhow, Error as AnyhowError};

  pub(crate) use super::group::ErrorGroup;

  pub use super::prelude::*;
}

pub mod prelude {
  pub use {super::monadic::Monadic, crate::kc};

  // #[cfg(feature = "groups")]
  pub use super::{batch::BatchResult, group::ErrorGroup, split::SplitResult};

  #[cfg(feature = "try_mut")]
  pub use super::try_mut::*;
}

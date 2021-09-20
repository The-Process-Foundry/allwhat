//! The tools needed to implement a generic try with rollback
//!
//! This is mostly going to move to Patchwork

use crate::local::*;

use std::collections::HashMap;

/// Wrappers for the reversion strategy
///
/// This is a bit more than Patchwork, since a function cannot be serialized
#[derive(Debug)]
pub enum PatchItem {
  /// A serialized raw value
  /// TODO: Make a custom patch serializer for serde, using raw values
  Value(serde_json::Value),
  // /// Use a custom closure to transform the value back
  // Function(
  //   Box<dyn FnOnce(Patch<E, A>) -> PatchResult<E>>,
  // ),
}

/// An object that stores methods and values to use in rolling back a transaction
#[derive(Debug)]
pub struct Patch<E, A>
where
  A: Invertable<Error = E>,
  E: Debug + Display + Send + Sync + 'static,
{
  data: core::marker::PhantomData<E>,
  pub action: A,
  pub patch: HashMap<String, PatchItem>,
}

/// A pair of functions that can be reversed
pub trait Invertable
where
  Self: Sized,
{
  type Error: Debug + Display + Send + Sync + 'static;

  fn apply(&self, item: impl Revertable) -> PatchResult<Self::Error, Self>;
  fn undo(item: impl Revertable, patch: Patch<Self::Error, Self>)
    -> PatchResult<Self::Error, Self>;
}

/// A set of closures needed to try an action
pub trait Revertable {
  /// The type of error expected to be returned when running each step
  type Error: Display + Debug + Send + Sync + 'static;

  /// An enumeration of the different processes that can generate a patch for this object
  type Action: Invertable<Error = Self::Error>;

  /// The actual function to run
  fn run(&mut self, action: Self::Action) -> PatchResult<Self::Error, Self::Action>;

  /// How to handle errors. Default is to just restore, but the response can be based on error type
  ///
  /// This can allow for retries by recursively calling itself
  fn revert(
    &self,
    patch: Patch<Self::Error, Self::Action>,
    err: Option<Self::Error>,
  ) -> PatchResult<(), Self::Error, Self::Action>;
}

#[derive(Debug)]
pub enum PatchResult<R, E, A>
where
  A: Invertable<Error = E>,
  E: Debug + Display + Send + Sync + 'static,
{
  /// The apply succeeded
  Ok(R, Patch<E, A>),

  /// The apply failed but was able to clean up
  Err(E),

  /// Failed to apply the patch and return
  ///
  /// The first value is error from attempting to apply the function, the second is the error from
  /// the restore
  Poisoned(E, E),
}

impl<R, E, A> std::fmt::Display for PatchResult<R, E, A>
where
  A: Invertable<Error = E> + Debug,
  E: Debug + Display + Send + Sync + 'static,
  R: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl<R, E, A> std::error::Error for PatchResult<R, E, A>
where
  A: Invertable<Error = E> + Debug,
  E: Debug + Display + Send + Sync + 'static,
{
}

//! The tools needed to implement a generic try with rollback

use crate::local::*;

/// Wrappers for the reversion strategy
///
/// This is a bit more than Patchwork, since a function cannot be serialized
pub enum StashType {
  ///
  Value(serde_json::Value),

  ///
  Function(Box<dyn FnMut(dyn TryMut, TryMutStash) -> PoisonedMut<E>>),
}

pub struct StashItem {}

/// An object that stores methods and values to use in rolling back a transaction
pub struct TryMutStash {
  stash: HashMap<Uuid, HashMap<String, StashType>>,
}

/// A set of closures needed to try an action
pub trait TryMutAction {
  /// The type this action is meant to act upon
  type Item;

  /// This is the stored information needed for
  type Stash: TryMutStash;

  type Error: Into<AnyhowError> + Display + Debug + Send + Sync + 'static;

  /// The errors that can be returned

  /// Stash items needed to restore upon failure
  fn stash(&self, item: &Self::Item) -> Self::Stash;

  /// The actual function to run
  ///
  /// The stash is passed in since it may be need to be appended to by intermediate steps
  fn run(&mut self, item: &mut Self::Item, stash: &mut Self::Stash) -> Result<(), Self::Error>;

  /// How to handle errors. Default is to just restore, but the response can be based on error type
  ///
  /// This can allow for retries by recursively calling itself
  fn revert(
    &self,
    item: &mut Self::Item,
    err: Self::Error,
    stash: Self::Stash,
  ) -> PoisonedMut<Self::Error>;
}

#[derive(Debug)]
pub enum PoisonedMut<E>
where
  E: Into<AnyhowError> + Debug + Display + Send + Sync + 'static,
{
  /// The apply succeeded
  Ok,

  /// The apply failed, but restore was successful
  Err(E),

  /// Failed to restore the item to it's pre-apply state
  ///
  /// The first value is error from attempting to apply the function, the second is the error from
  /// the restore
  Poisoned(E, E),
}

impl<E> std::fmt::Display for PoisonedMut<E>
where
  E: Into<AnyhowError> + Debug + Display + Send + Sync + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl<E> std::error::Error for PoisonedMut<E> where
  E: Into<AnyhowError> + Debug + Display + Send + Sync + 'static
{
}

/// All potential accumulators need to be able to try to apply a function and rollback if it fails
///
/// Clone/Copy is the simplest way to implement this, but both are rather heavy and cannot
/// target something nested.
///
/// This is used for when it's cheaper to beg forgiveness than to ask for permission, meaning that
/// rolling back after an issue is detected is easier than trying to duplicate the business logic
/// ensuring that the action is valid beforehand.
///
/// FIXME: Backup doesn't make sense, since backup may need the context. Should it return a closure
/// or require a struct of preloaded context to add to the backup?
pub trait TryMut {
  /// An error type that can help the restore function
  type Error: Into<AnyhowError> + Display + Debug + Send + Sync + 'static;
  type Action: TryMutAction;

  /// Try to apply a closure to the accumulator and rollback any errors
  fn try_mut(&mut self, action: &mut Self::Action) -> PoisonedMut<Self::Error>;
}

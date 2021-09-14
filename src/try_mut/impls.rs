//! Implementations of TryMut on the primitives and some common structs
//!
//! Since try_mut is recursive,

use super::traits::*;
use anyhow::Error as AnyhowError;
use paste::paste;

/// All the primitives just use a simple copy as a backup, so we can implement them in the same way
macro_rules! try_mut_primitive {
  ([$(($type:ty, $str:ident)),+]) => {
    paste! {
    $(
    /// Make sure it can be used as a stash by itself
    impl TryMutStash for $type {}

    #[allow(non_camel_case_types)]
    pub enum [<$str TryMut>] {
      Op(Box<dyn FnMut(&mut $type) -> Result<(), AnyhowError>>),
    }

    impl TryMutAction for [<$str TryMut>] {
      type Item = $type;
      type Stash = $type;
      type Error = AnyhowError;

      /// Stash items needed to restore upon failure
      ///
      /// Primitives simply copy themselves, as anything more is too complex
      fn stash(&self, item: &Self::Item) -> Self::Stash {
        match self {
          _ => item.clone(),
        }
      }

      /// The actual function to run
      fn run(
        &mut self,
        item: &mut Self::Item,
        _stash: &mut Self::Stash,
      ) -> Result<(), Self::Error> {
        match self {
          Self::Op(func) => func(item),
        }
      }

      /// How to handle errors. Default is to just restore, but the response can be based on error type
      fn revert(
        &self,
        item: &mut Self::Item,
        err: Self::Error,
        stash: Self::Stash,
      ) -> PoisonedMut<Self::Error> {
        *item = stash;
        PoisonedMut::Err(err)
      }
    }

    impl TryMut for $type {
      type Error = AnyhowError;
      type Action = [<$str TryMut>];

      fn try_mut(&mut self, action: &mut Self::Action) -> PoisonedMut<Self::Error> {
        let mut stash = action.stash(self);

        match action.run(self, &mut stash) {
          Ok(_) => PoisonedMut::Ok,
          Err(err) => action.revert(self, err, stash),
        }
      }
    }
  )+}
  };
  }

try_mut_primitive! {
  [(bool, bool), (char, char), (i128, i128), (i64, i64), (i32, i32), (i16, i16), (i8, i8), (f64, f64), (f32, f32), (isize, isize), (u128, u128), (u64, u64), (u32, u32), (u16, u16), (u8, u8), (usize, usize)]
}

//! Iterator based functions that allow partial success
//!
//! TODO: Add the ? functionality for split results to throw errors

use super::group::{ErrorGroup, Grouper};
use anyhow::anyhow;

#[derive(Debug)]
pub struct SplitResult<T>
// where
// E: From<ErrorGroup> + std::error::Error + Sync + Send + 'static,
{
  values: Vec<T>,
  errors: Option<ErrorGroup>,
}

impl<T> SplitResult<T> {
  /// Apply a function to each value of an iterator, sorting successes from errors
  ///
  // Similar to iterator.partition
  pub fn map<U, E, F>(list: impl Iterator<Item = U>, func: F) -> SplitResult<T>
  where
    F: Fn(U) -> Result<T, E>,
    E: std::fmt::Display + std::fmt::Debug + Sync + Send + 'static,
  {
    let mut values = vec![];
    let mut group = ErrorGroup::new(None);
    for item in list {
      match func(item) {
        Ok(value) => values.push(value),
        Err(err) => group.append(anyhow!(err)),
      }
    }

    SplitResult {
      values,
      errors: match group.len() {
        0 => None,
        _ => Some(group),
      },
    }
  }
}

impl<T> Grouper for SplitResult<T> {
  type Result = Vec<T>;

  fn context(self, ctx: String) -> SplitResult<T> {
    SplitResult {
      errors: Some(match self.errors {
        Some(group) => group.set_label(ctx),
        None => ErrorGroup::new(Some(ctx)),
      }),
      ..self
    }
  }

  /// Convert this to a result, Ok(values) if errors is None and Err(errors) if not
  fn as_result<E: From<ErrorGroup>>(self) -> Result<Self::Result, E> {
    match self.errors {
      Some(err) => Err(err)?,
      None => Ok(self.values),
    }
  }
}

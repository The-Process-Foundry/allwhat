//! Iterator based functions that allow partial success
//!
//! TODO: Add the ? functionality for split results to throw errors

use super::group::ErrorGroup;
use anyhow::Error as AnyhowError;

#[derive(Debug)]
pub struct SplitResult<T>
// where
// E: From<ErrorGroup> + std::error::Error + Sync + Send + 'static,
{
  values: Vec<T>,
  errors: Option<ErrorGroup>,
}

impl<T> SplitResult<T> {
  /// Convert this to a result, Ok(values) if errors is None and Err(errors) if not
  pub fn as_result<E: From<ErrorGroup>>(self) -> Result<Vec<T>, E> {
    match self.errors {
      Some(err) => Err(err)?,
      None => Ok(self.values),
    }
  }

  /// Apply a function to each value of an iterator, sorting successes from errors
  pub fn map<U, E, F>(list: impl Iterator<Item = U>, func: F) -> SplitResult<T>
  where
    F: Fn(U) -> Result<T, E>,
    E: Into<AnyhowError>,
  {
    let mut values = vec![];
    let mut group = ErrorGroup::new();
    for item in list {
      match func(item) {
        Ok(value) => values.push(value),
        Err(err) => group.append(err),
      }
    }

    SplitResult {
      values: values,
      errors: match group.len() {
        0 => None,
        _ => Some(group),
      },
    }
  }
}

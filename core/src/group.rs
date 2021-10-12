//! Tools for grouping lists of sibling errors
//!
//! On many occasions, I want to accumulate errors rather than failing fast. Sometimes

use crate::local::*;

/*
/// The generally required traits to be used as a grouped error
///
/// All external errors are wrapped up using Anyhow, allowing us to use context, backtrace and other
/// features already implemented.
pub(crate) struct AnyhowError {
  inner: anyhow::Error,
}

impl AnyhowError {
  pub fn convert<E>(err: E) -> AnyhowError
  where
    E: Display + Debug + Send + Sync + 'static,
  {
    anyhow::anyhow!(err)
  }
}

impl<E> From<E> for AnyhowError
where
  E: Display + Debug + Send + Sync + 'static,
{
  fn from(err: E) -> AnyhowError {
    anyhow::anyhow!(err)
  }
}

impl Display for AnyhowError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self.inner)
  }
}

impl Debug for AnyhowError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self.inner)
  }
}

// impl Display for AnyhowError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.inner)
//   }
// }

*/

pub trait Grouper {
  type Result;

  // Set a label to apply to all the errors
  fn context(self, ctx: String) -> Self;

  fn as_result<E: From<ErrorGroup>>(self) -> Result<Self::Result, E>;
}

/// An error accumulator
///
/// This is intended to enumerate all the errors found in a transaction rather than failing on
/// the first
#[derive(Debug)]
pub struct ErrorGroup {
  label: Option<String>,
  errors: Vec<AnyhowError>,
}

impl std::error::Error for ErrorGroup {}

impl Display for ErrorGroup {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    let errors = self
      .errors
      .iter()
      .enumerate()
      .fold(String::new(), |acc, (i, err)| {
        format!("{}\t{}) {}\n", acc, i + 1, err)
      });
    let label = match &self.label {
      Some(val) => val.clone(),
      None => "Error Group".to_string(),
    };
    write!(f, "{}:\n{}", label, errors)
  }
}

impl ErrorGroup {
  /// A simple constructor
  pub fn new(label: Option<String>) -> ErrorGroup {
    ErrorGroup {
      label,
      errors: vec![],
    }
  }

  /// Return the number of errors contained
  pub fn len(&self) -> usize {
    self.errors.len()
  }

  /// Return if there are any errors in the list
  pub fn is_empty(&self) -> bool {
    self.errors.is_empty()
  }

  pub fn set_label(self, label: String) -> Self {
    ErrorGroup {
      label: Some(label),
      ..self
    }
  }

  /// Add a new error to the ErrorGroup in place
  pub fn append<F: Into<AnyhowError>>(&mut self, error: F) {
    self.errors.push(error.into());
  }

  /// Add a new error to the ErrorGroup as functional pattern for chaining terms
  pub fn appendf<F: Into<AnyhowError>>(mut self, error: F) -> ErrorGroup {
    self.append(error.into());
    self
  }

  /// Add on a list of errors
  pub fn extend<T, F: Into<AnyhowError>>(&mut self, _list: impl Iterator<Item = Result<T, F>>) {
    unimplemented!("'' still needs to be implemented")
  }

  /// Pull the error out from the result and append it to the group
  ///
  /// THINK: Since there is no clone for Anyhow's error, do we want to leave unit or the debug
  /// string in place?
  ///
  /// Example:
  /// ```rust
  /// use anyhow::{anyhow, Context, Result};
  /// use allwhat::ErrorGroup;

  ///
  /// let mut group: ErrorGroup = ErrorGroup::new(Some("Group Label".to_string()));
  ///
  /// let value1: Result<&str> = Ok("Ok does nothing");
  /// assert!(cmp(group.extract(value1), Ok("Ok does nothing")));

  /// let value2: Result<()> = Err(anyhow!("Value2 Error"));
  /// assert!(cmp(
  ///   group.extract(value2),
  ///   Err(())
  /// ));

  /// let value3: Result<()> = Err(anyhow!("Value3 Error")).context("Context Value");
  /// assert!(cmp(
  ///   group.extract(value3),
  ///   Err(())
  /// ));
  /// ```
  pub fn extract<T, F>(&mut self, result: Result<T, F>) -> Result<T, AnyhowError>
  where
    F: Into<AnyhowError> + Display,
  {
    match result {
      Ok(t) => Ok(t),
      Err(err) => {
        let new_err = anyhow!("(Extracted) - {}", err);
        self.append(err);
        Err(new_err)
      }
    }
  }

  /// Unwrap a list of results, splitting it into unwrapped values and an optional flattened error
  ///
  /// THINK: Should there al
  pub fn unwrap_all<T, F: Into<AnyhowError> + Display>(
    results: impl Iterator<Item = Result<T, F>>,
  ) -> (Vec<T>, Option<Self>) {
    let mut result = vec![];
    let mut errors = ErrorGroup::new(None);
    for item in results {
      match item {
        Ok(x) => result.push(x),
        Err(err) => errors.append(err),
      }
    }

    match errors.len() {
      0 => (result, None),
      _ => (result, Some(errors)),
    }
  }
}

/// Iterates through a group of variables and moves all the errors into a single group.
///
/// Since errors cannot be cloned, they are replaced with the result of running display in the
/// original result moving the error to the ErrorGroup.
///
/// Example:
/// ```rust
/// use anyhow::{anyhow, Context, Result};
/// use allwhat::{ErrorGroup, extract_errors};
///
/// fn get_int(val: i64, is_ok: bool) -> Result<u64, String> {
///   match is_ok {
///     true: Ok(val as u64),
///     false: Err(format!("Forced Error for val {}", val)),
///   }
/// }
///
/// fn get_str(val: &str, is_ok: bool) -> Result<String, &str> {
///   match is_ok {
///     true: Ok(format!("Valid: {}", val)),
///     false: Err(format!("Invalid: {}", val)),
///   }
/// }
///
/// // Create the variables before the macro
/// let int_1 = get_int(1, true);
/// let int_2 = get_int(2, false);
/// let int_3 = get_int(3, false).context("Forced 3 with a context");
///
/// extract_errors!(
///   result = [
///     int_1,
///     int_2,
///     int_3,
///     // Just add a function call or block of code to execute
///     str_1 => get_str("Str 1", true);
///     str_2 => {
///       let block = get_str("String 3", true);
///       block.context("No Error, but adding a context anyways")
///     },
///     str_3 => get_str("String 3", false),
///     str_4 => {
///       let err_str = get_str("String 4", false);
///       err_str.context("String 4 errored with context")
///     },
///   ]
/// );
/// ```
///
#[macro_export]
macro_rules! extract_errors {
  (@inner $result:ident, $var:ident) => {
    let $var: Result<_> = $result.extract($var);
  };

  (@inner $result:ident, $var:ident => $val:expr) => {
    let $var = $result.extract($val);
  };

  (@inner $result:ident, $var:ident : $type:ty => $val:expr) => {
    let $var : $type = { $val };

    extract_errors!(@inner $result, $var);
  };

  ($result:ident = [$($var:ident $($(: $type:ty)? => $val:expr)?),+ $(,)?]) => {
    let mut $result = ErrorGroup::new(Some("Extracted Errors".to_string()));

    $(
      extract_errors!(
        @inner $result, $var $( $(: $type)? => $val)?
      );
    )+
  };
}

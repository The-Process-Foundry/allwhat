//! Tools for grouping lists of sibling errors
//!
//! On many occasions, I want to accumulate errors rather than failing fast. Sometimes

use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Error, Result};

/// An error accumulator
///
/// This is intended to enumerate all the errors found in a transaction rather than failing on
/// the first
#[derive(Debug)]
pub struct ErrorGroup {
  errors: Vec<Error>,
}

impl Display for ErrorGroup {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    let errors = self
      .errors
      .iter()
      .enumerate()
      .fold(String::new(), |acc, (i, err)| {
        format!("{}\t{}) {}", acc, i, err)
      });
    write!(f, "ErrorGroup:\n{}", errors)
  }
}

impl ErrorGroup {
  /// A simple constructor
  pub fn new() -> ErrorGroup {
    ErrorGroup { errors: vec![] }
  }

  /// Return the number of errors contained
  pub fn len(&self) -> usize {
    self.errors.len()
  }

  /// Add a new error to the ErrorGroup in place
  pub fn append<E: Into<Error>>(&mut self, error: E) -> () {
    self.errors.push(error.into());
  }

  /// Add a new error to the ErrorGroup as functional pattern for chaining terms
  pub fn appendf<E: Into<Error>>(mut self, error: E) -> ErrorGroup {
    self.append(error);
    self
  }

  /// Add on a list of errors
  pub fn extend<F: Into<Error> + Display>(&mut self, _list: impl Iterator<Item = Result<F>>) -> () {
    unimplemented!("'' still needs to be implemented")
  }

  /// Pull the error out from the result and append it to the group
  ///
  /// THINK: Since there is no clone for Anyhow's error, do we want to leave unit or the debug
  /// string in place?
  ///
  /// Example:
  /// ```rust
  /// use anyhow::{anyhow, Context, ErrorGroup, Result};
  ///
  /// let mut group: ErrorGroup = ErrorGroup::new();
  ///
  /// fn cmp<T: Eq>(left: Result<T>, right: Result<T, &str>) -> bool {
  ///   match (left, right) {
  ///     (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
  ///     (Ok(l), Ok(r)) => l == r,
  ///     (Err(l), Err(r)) => l.to_string().eq(&r.to_string()),
  ///   }
  /// }
  ///
  /// let value1: Result<&str> = Ok("Ok does nothing");
  /// assert!(cmp(group.extract(value1), Ok("Ok does nothing")));

  /// let value2: Result<()> = Err(anyhow!("Value2 Error"));
  /// assert!(cmp(
  ///   group.extract(value2),
  ///   Err("(Extracted) - Value2 Error")
  /// ));

  /// let value3: Result<()> = Err(anyhow!("Value3 Error")).context("Context Value");
  /// assert!(cmp(
  ///   group.extract(value3),
  ///   Err("(Extracted) - Context Value")
  /// ));
  /// ```
  pub fn extract<T, F: Into<Error> + Display>(&mut self, result: Result<T, F>) -> Result<T> {
    match result {
      Ok(t) => Ok(t),
      Err(err) => {
        let new_err = anyhow!("(Extracted) - {}", err);
        self.append(err);
        println!("Creating new error from: {}", new_err);
        Err(new_err)
      }
    }
  }

  /// Unwrap a list of results, splitting it into unwrapped values and an optional flattened error
  ///
  /// THINK: Should there al
  pub fn unwrap_all<'a, T, F: Into<Error> + Display>(
    results: impl Iterator<Item = Result<T, F>>,
  ) -> (Vec<T>, Option<Self>) {
    let mut result = vec![];
    let mut errors = ErrorGroup::new();
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
/// use anyhow::{anyhow, Context, ErrorGroup, Result, extract_errors};
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
    let mut $result = ErrorGroup::new();

    $(
      extract_errors!(
        @inner $result, $var $( $(: $type)? => $val)?
      );
    )+
  };
}

#[cfg(test)]
mod tests {
  use super::ErrorGroup;
  use crate::extract_errors;
  use crate::tools::*;
  use anyhow::{anyhow, Context, Result};

  #[test]
  fn error_group() -> () {
    let mut group: ErrorGroup = ErrorGroup::new();

    let value1: Result<&str> = Ok("Ok does nothing");
    assert!(cmp(group.extract(value1), Ok("Ok does nothing")));

    let value2: Result<()> = Err(anyhow!("Value2 Error"));
    assert!(cmp(
      group.extract(value2),
      Err("(Extracted) - Value2 Error")
    ));

    let value3: Result<()> = Err(anyhow!("Value3 Error")).context("Context Value");
    assert!(cmp(
      group.extract(value3),
      Err("(Extracted) - Context Value")
    ));
  }

  #[test]
  #[allow(unused_assignments)]
  fn test_extract_errors() -> () {
    use super::ErrorGroup;
    use crate::extract_errors;
    use anyhow::{Context, Result};

    fn get_int(val: i64, is_ok: bool) -> Result<u64> {
      match is_ok {
        true => Ok(val as u64),
        false => Err(anyhow!("Forced Error for val {}", val)),
      }
    }

    fn get_str(val: &str, is_ok: bool) -> Result<String> {
      match is_ok {
        true => Ok(format!("Valid: {}", val)),
        false => Err(anyhow!("Invalid: {}", val)),
      }
    }

    // Create the variables before the macro
    let int_1 = get_int(1, true);
    let int_2 = get_int(2, false);
    let int_3 = get_int(3, false).context("Forced 3 with a context");

    extract_errors!(
      err_res = [
        int_1,
        int_2,
        int_3,
        // Just add a function call or block of code to execute
        str_1 => get_str("String 1", true),
        str_2: Result<String> => Ok(format!("String 2")),
        str_3 => {
          let block = get_str("String 3", true);
          block.context("No Error, but adding a context anyways")
        },
        str_4 => get_str("String 4", false),
        str_5 => {
          let err_str = get_str("String 5", false);
          err_str.context("String 5 errored with context")
        },
      ]
    );

    // Now we have an 3 values that are Ok, 4 Errors, and an ErrorResult named err_res with 4 errors.
    assert_eq!(int_1.unwrap(), 1);
    assert_eq!(str_1.unwrap().to_string(), "Valid: String 1".to_string());
    assert_eq!(str_2.unwrap().to_string(), format!("String 2"));
    assert_eq!(str_3.unwrap().to_string(), format!("Valid: String 3"));

    assert_eq!(err_res.len(), 4);

    // Check the left-overs from the errors
    assert_eq!(
      int_2.unwrap_err().to_string(),
      "(Extracted) - Forced Error for val 2".to_string()
    );
    assert_eq!(
      int_3.unwrap_err().to_string(),
      "(Extracted) - Forced 3 with a context".to_string()
    );
    assert_eq!(
      str_4.unwrap_err().to_string(),
      "(Extracted) - Invalid: String 4".to_string()
    );
    assert_eq!(
      str_5.unwrap_err().to_string(),
      "(Extracted) - String 5 errored with context".to_string()
    );

    // And inspect the contents of the Error Group
    let display = "ErrorGroup:\n\t0) Forced Error for val 2\t1) Forced 3 with a context\t2) Invalid: String 4\t3) String 5 errored with context".to_string();
    assert_eq!(err_res.to_string(), display);

    let debug = "ErrorGroup { errors: [Forced Error for val 2, Forced 3 with a context\n\nCaused by:\n    Forced Error for val 3, Invalid: String 4, String 5 errored with context\n\nCaused by:\n    Invalid: String 5] }".to_string();
    assert_eq!(format!("{:?}", err_res), debug);
  }
}

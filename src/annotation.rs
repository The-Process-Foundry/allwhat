//! Evaluate a value and
//!
//!

/// A
pub struct AnnotatedResult<T> {
  value: Option<T>,
  errors: Option<ErrorGroup<Error>>,
}

impl<T> AnnotatedResult<T> {
  pub new(value: T) -> AnnotatedResult {
    value: Some(value),
    errors: None,
  }

  /// Use a function's result to create a new AnnotatedResult
  pub create(creator: Fn<()> -> Result<T>) -> AnnotatedResult<T> {
    unimplemented!("'AnnotatedResult.create' still needs to be implemented")
  }

  /// Run the value through a list of tests and add failures to the result
  pub evaluate(mut self, tests: impl Iterator<Item = Fn<T> -> Result<()>>) -> AnnotatedResult<T> {
    unimplemented!("'AnnotatedResult.evaluate' still needs to be implemented")
  }
}
//! Add additional monadic functions to result
//!
//! These should allow for reducing boilerplate some known patterns.
//!

use core::fmt::Display;

// Add some monad features to the result
pub trait Monadic<T, E>
where
  E: Display + Send + Sync + 'static,
{
  /// Kleisli Composition
  ///
  /// Similar to the .map function, but consumes the result and returns a new one instead
  /// of only replacing the inner value of an Ok with a sub-result
  ///
  /// Does Signature
  /// Result<T, E> => Result<U, E>
  fn kc<U, F>(self, func: F) -> Result<U, E>
  where
    E: Display + Send + Sync + 'static,
    F: FnOnce(T) -> Result<U, E>;
}

impl<T, E> Monadic<T, E> for Result<T, E>
where
  E: Display + Send + Sync + 'static,
{
  // Kleisli
  fn kc<U, F>(self, func: F) -> Result<U, E>
  where
    F: FnOnce(T) -> Result<U, E>,
  {
    match self {
      Ok(value) => func(value),
      Err(err) => Err(err),
    }
  }
}

/// The Kleisli Composition
///
/// Similar to the .map function, but consumes the result and replaces it with a new one.
/// Does Signature
/// Result<T, E> => Result<U, E>
///
/// THINK: Do I want to add a "map_err" into the syntax as an optional second block?
///
/// Example:
/// let result = kc {
///   Ok("".to_string)
///   >=> |t: &str| OK()
///   >=> |updated| Ok(Some("No longer a bare string"))
/// }
/// assert_eq!(result, "Hello World");
///
///
#[macro_export]
macro_rules! kc {
  ($start:expr => $($term:expr)=>+) => {
    $start $( .kc($term) )+
  };
}

#[cfg(test)]
mod tests {
  use anyhow::anyhow;

  use super::Monadic;

  #[test]
  fn test_kleisli() -> () {
    use crate::kc;
    use std::boxed::Box;
    use std::cell::RefCell;

    // The simple case, which returns the same as using the basic Result::map function
    let all_ok: Result<Option<String>, &str> = kc!(
      Ok("".to_string())
      => |mut t| {
        t.push_str("Hello World");
        Ok(t)
      }
      => |t| Ok(Some(t))
    );
    assert_eq!(all_ok, Ok(Some("Hello World".to_string())));

    // Same with starting with an error
    let start = kc!(
      Err(anyhow!("Start Error"))
      => |t| Ok(Some(t))
      => |t: Option<String>| {
        let u = t.unwrap().push_str("Back To a String");
        Ok(u)
      }
    );

    assert_eq!(
      start.unwrap_err().to_string(),
      anyhow!("Start Error").to_string()
    );

    // Here's where it gets interesting, where the error happens in the middle
    let middle: Result<i32, _> = kc!(
      Ok(10)
      => |t| Ok(t + 5)
      => |_| Err(anyhow!("Raising an error here"))
      => |t: i32| Ok(t + 20)
    );

    assert_eq!(
      middle.unwrap_err().to_string(),
      anyhow!("Raising an error here").to_string()
    );

    // A caution on mutable variables. This does not prevent mutation on values before the error
    // occurs
    let value = RefCell::new(100);
    let mutant = kc!(
      Ok(Box::new(&value))
      => |t| {
        *t.borrow_mut() += 5;
        Ok(t)
      }
      => |_| Err("Now an error")
      => |t: Box<RefCell<i32>>| {
        *t.borrow_mut() += 10;
        Ok(t)
      }
    );
    // Returns an error
    assert_eq!(
      mutant.unwrap_err().to_string(),
      anyhow!("Now an error").to_string()
    );
    // But the value has been partially increased
    assert_eq!(*value.borrow(), 105);
  }
}

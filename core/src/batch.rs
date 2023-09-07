//! Accumulate the results of running groups of functions against a single object
//!
//! There two primary use cases that I can think of.
//!
//! 1) Bulk processing - This is items like inserting to a database or parsing tabular data. I don't
//!    want to throw away completed work if there is one error. Even though there is one database,
//!    it doesn't hurt it internally when one query in hundreds fails.
//! 2) Validation - In this case, I want to gather as many errors as I can and report at once. It is
//!    quite time consuming to fail fast and have to rerun to find the next issue.
//!

use crate::local::*;

/// Tells the caller whether the restore after an error
/// Gather the results from applying multiple
#[derive(Debug)]
pub struct BatchResult<T> {
  count: u32,
  value: T,
  errors: ErrorGroup,
}

// impl<T> TryMut for BatchResult<T>
// where
//   T: TryMut,
// {
//   type Error = AnyhowError;
//   type Backup = ();

//   // fn backup<B>(&self) -> B {
//   //   self.value.backup()
//   // }

//   // /// Return the object to its previous state because of an error
//   // fn restore<B>(&mut self, backup: B) -> () {
//   //   self.value.restore(backup)
//   // }

//   ///
//   fn try_mut<E, F>(&mut self, _func: F) -> Result<(), Self::Error>
//   where
//     E: Into<AnyhowError>,
//     F: FnMut(&mut Self) -> Result<(), E>,
//   {
//     unimplemented!("BatchResult should only use try_mut_quiet")
//   }

//   fn try_mut_quiet<E, F>(&mut self, mut func: F)
//   where
//     E: Into<Self::Error>,
//     F: FnMut(Self) -> Result<(), E>,
//   {
//     // This increments regardless and doesn't get rolled back
//     self.count += 1;
//     // let backup: B = self.backup();
//     if let Err(err) = func(&mut self) {
//       self.append(err);
//       // self.restore(backup);
//     }
//   }
// }

impl<T> BatchResult<T> {
  pub fn new(init: T) -> Self {
    BatchResult {
      count: 0,
      value: init,
      errors: ErrorGroup::new(Some("Batch Errors".to_string())),
    }
  }

  /// Change the label on the error group
  pub fn set_label(self, label: &str) -> BatchResult<T> {
    BatchResult {
      errors: self.errors.set_label(label.into()),
      ..self
    }
  }

  /// The number of errors accumulated
  pub fn count_error(&self) -> u32 {
    self.errors.len() as u32
  }

  /// The number of successful functions run against this result
  pub fn count_valid(&self) -> u32 {
    self.count - self.count_error()
  }

  /// The total number of functions run against this result
  pub fn count(&self) -> u32 {
    self.count
  }

  /// Whether any errors were found in the batch
  pub fn is_ok(&self) -> bool {
    self.count_error() == 0
  }

  /// Add an additional error to the batch result
  pub fn append<E>(&mut self, err: E)
  where
    E: Debug,
  {
    self.errors.append(format!("{:#?}", err));
  }

  /// Run the value through a list of tests and add failures to the result
  pub fn validate<E, Func>(value: T, tests: impl Iterator<Item = Func>) -> BatchResult<T>
  where
    Func: FnOnce(&T) -> Result<(), E>,
    E: Debug,
  {
    let mut errors = ErrorGroup::new(None);
    let mut count = 0;
    for test in tests {
      count += 1;
      if let Err(err) = test(&value) {
        errors.append(format!("{:#?}", err))
      };
    }

    BatchResult {
      count,
      value,
      errors,
    }
  }

  /// Uses a function to apply each item to the accumulator, storing errors for future examination
  ///
  /// Please note, errors have the potential to corrupt the accumulator since it mutates
  pub fn apply<Err, Func>(mut self, func: Func) -> BatchResult<T>
  where
    Func: Fn(&mut T) -> Result<(), Err>,
    Err: Display + Debug + Send + Sync + 'static,
  {
    self.count += 1;
    let res = func(&mut self.value);
    if let Err(err) = res {
      self.append(format!("{:#?}", err));
    }
    self
  }

  /// Uses a function to apply each item to the accumulator, storing errors for future examination
  ///
  /// Please note, errors have the potential to corrupt the accumulator since it mutates
  /// TODO: Make this fold_mut. fold should take a builder style
  pub fn fold<Item, Err, Func>(
    accumulator: T,
    list: impl Iterator<Item = Item>,
    func: Func,
  ) -> BatchResult<T>
  where
    Func: Fn(&mut T, Item) -> Result<(), Err>,
    Err: Display + Debug + Send + Sync + 'static,
  {
    list.fold(BatchResult::new(accumulator), |mut acc, item| {
      acc.count += 1;
      let res = func(&mut acc.value, item);
      if let Err(err) = res {
        acc.append(err);
      }
      acc
    })
  }

  /// Equivalent of a for loop, capturing all errors into a single batch
  ///
  /// If a value is expected to be returned, SplitResult should be used
  pub fn foreach<Item, Err, Func>(
    list: impl Iterator<Item = Item>,
    func: &mut Func,
  ) -> BatchResult<()>
  where
    Func: FnMut(Item) -> Result<(), Err>,
    Err: Display + Debug + Send + Sync + 'static,
  {
    let mut result = BatchResult {
      count: 0,
      value: (),
      errors: ErrorGroup::new(Some("ForEach loop result".to_string())),
    };

    for item in list {
      result.count += 1;
      match func(item) {
        Ok(_) => (),
        Err(err) => result.append(err),
      }
    }
    result
  }
}

impl<T> Grouper for BatchResult<T> {
  type Result = T;

  fn context(self, ctx: String) -> BatchResult<T> {
    BatchResult {
      errors: self.errors.set_label(ctx),
      ..self
    }
  }

  /// Convert this to a result, Ok(values) if errors is None and Err(errors) if not
  fn as_result<E: From<ErrorGroup>>(self) -> Result<Self::Result, E> {
    match self.errors.len() {
      0 => Ok(self.value),
      _ => Err(self.errors.into()),
    }
  }
}

/*

----  Found a nightly workaround for try_insert for hashmap, so I'm going to wait to implement
      this

/// Attempt to fold a set of values into the accumulator, rolling back errors
///
/// This is heavy, and requires implementing TryMut to use this. The difference is that the
/// accumulator won't be left in an unknown state after an error.
#[cfg(feature = "try_mut")]
pub fn try_fold<U, F>(
  accumulator: T,
  list: impl Iterator<Item = U>,
  action: Box<dyn Fn(U) -> T::Action>,
) -> BatchResult<T>
where
  F: Fn(&mut T, &U) -> PatchResult<T::Error, T::Action>,
  T: Revertable,
{
  let acc = BatchResult::new(accumulator);
  // for item in list {
  //   acc.count += 1;
  //   match acc.value.try_mut(|value| func(value, &item)) {
  //     PoisonErr::Ok => (),
  //     PoisonErr::Err(err) => acc.append(err),
  //     PoisonErr::Poisoned(err1, err2) => {
  //       let error = anyhow!(err2).context(err1);
  //       println!("{}", error);
  //     }
  //   };
  // }
  acc
}

// A set of functions where all need to succeed or the successes should also be rolled back.
// THINK: Maybe a macro
// pub fn transaction()
*/

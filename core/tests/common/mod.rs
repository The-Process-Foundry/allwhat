//! Common tools used by the tests

pub use anyhow::{anyhow, Error as AnyhowError, Result};

/// A testing buffer to more easily track the expected output of an error
pub struct ErrorBuffer {
  count: u8,
  value: String,
}

impl ErrorBuffer {
  pub fn new() -> ErrorBuffer {
    Default::default()
  }

  /// Get the result of the buffer, clearing the contents
  pub fn pop(&mut self) -> String {
    self.count = 0;
    let result = self.value.to_owned();
    self.value = "Bulk Try Aggregation:\n".to_string();
    result
  }

  /// Add a new message and convert it to an anyhow error
  pub fn add(&mut self, msg: String) -> AnyhowError {
    self.count += 1;
    self.value = format!("{}\t{}) {}\n", self.value, self.count, msg);
    anyhow!(msg)
  }
}

impl Default for ErrorBuffer {
  fn default() -> ErrorBuffer {
    ErrorBuffer {
      count: 0,
      value: "Bulk Try Aggregation:\n".to_string(),
    }
  }
}

impl std::fmt::Display for ErrorBuffer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

// A global buffer that can aggregate the expected bulk result
lazy_static::lazy_static! {
  pub static ref ERR: std::sync::Mutex<ErrorBuffer> = {
    std::sync::Mutex::new(ErrorBuffer::new())
  };
}

#[macro_export]
macro_rules! err {
  ($msg:expr) => {
    crate::ERR.lock().unwrap().add($msg)
  };
  ($($msg:expr),+) => {
    err!(format!($($msg),+))
  };
}

/// Retrieve the value from the error buffer for comparison and clear it
#[macro_export]
macro_rules! pop_err {
  () => {
    crate::ERR.lock().unwrap().pop()
  };
}

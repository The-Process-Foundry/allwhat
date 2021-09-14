//! Common tools used by the tests

pub use allwhat::prelude::*;

pub use anyhow::{anyhow, Error as AnyhowError, Result};

use std::error::Error;

/// Create a simple string error to simulate how an outside user would use it
#[derive(Debug)]
pub struct TestErr(pub String);

impl std::fmt::Display for TestErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self.0)
  }
}

impl Error for TestErr {}

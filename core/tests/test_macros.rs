//! Test the map struct macro

// mod common;

#[derive(Debug)]
pub struct NestedStruct {
  pub str_maybe: String,
}

#[derive(Debug)]
pub struct TestMap {
  pub int_maybe: i32,
  pub int_sure: i32,
  pub float_maybe: f32,
  pub float_sure: f32,
  pub nested: NestedStruct,
}

// Test Enum

// Test Tuples

// Test Struct

// Test Nested

// Test func with try params
///
/// let value = DemoStruct {
///   value1: value1?,
///   value_from_func: func(param1?, param2?)?,
/// })

#[test]
fn test_try_assign() {
  use allwhat::prelude::ErrorGroup;
  use allwhat::try_assign;
  use anyhow::{anyhow, Error as AnyhowError};

  // TODO: Check if type annotations are still needed if this is properly generated
  let raw_float: Result<f32, AnyhowError> = Ok(3.14159);
  let raw_int: Result<i32, AnyhowError> = Err(anyhow!("An Error for Int"));
  let raw_str: Result<String, AnyhowError> = Ok("Hi, I'm nested".to_string());

  let map: Result<TestMap, ErrorGroup> = try_assign! {
    TestMap {
      float_maybe: raw_float?,
      int_maybe: raw_int?,
      float_sure: 1.602,
      int_sure: 82,
      nested: NestedStruct {
        str_maybe: raw_str?,
      }
    }
  };

  println!("The test result: {:#?}", map);
  assert!(false)
}

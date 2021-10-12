//! Test Expr Struct
//!
//! This is making use of the ExprField

use allwhat::{bulk_try, prelude::ErrorGroup};
use anyhow::{anyhow, Error as AnyhowError, Result};

#[derive(Debug, PartialEq, Eq)]
struct TestStruct {
  string: String,
  boolean: bool,
  vector: Vec<i32>,
  next: Option<Box<TestStruct>>,
}

// Wrap a value in Ok so we can annotate the error type
macro_rules! ok {
  ($val:expr) => {{
    let x: Result<_, AnyhowError> = Ok($val);
    x
  }};
}

#[test]
fn test_struct_assignment() {
  // Test 1
  //   Create a new struct with good values
  let ok_string = ok!("Test_String".to_string());
  let ok_bool = ok!(true);
  let ok_vec = ok!(vec![1, 2, -5]);

  let test1 = bulk_try! {
    TestStruct {
      string: ok_string?,
      boolean: ok_bool?,
      vector: ok_vec?,
      next: None,
    }
  };

  match test1 {
    Err(err) => panic!("Test 1 was supposed to be Ok. Instead returned: {:#?}", err),
    Ok(val) => assert_eq!(
      val,
      TestStruct {
        string: "Test_String".to_string(),
        boolean: true,
        vector: vec![1, 2, -5],
        next: None
      }
    ),
  };

  let mut error_count = 0;
  let mut expected_error = "Bulk Try Aggregation:\n".to_string();

  // Wrap a value in Ok so we can annotate the error type.
  // TODO: Move this to a common result class where we can reuse this macro
  macro_rules! err {
    ($val:expr) => {{
      error_count += 1;
      expected_error = format!("{}\t{}) {}\n", expected_error, error_count, $val);
      let x: Result<_, AnyhowError> = Err(anyhow!($val));
      x
    }};
  }

  // Test 2
  //   Create a new struct with multiple errors
  let err_string = err!("String Error 1");
  let err_bool = err!("Boolean Error 2");
  let err_vec = err!("Vector Error 3");

  let test2 = bulk_try! {
    TestStruct {
      string: err_string?,
      boolean: err_bool?,
      vector: err_vec?,
      next: None,
    }
  };

  match test2 {
    Ok(val) => panic!(
      "Test 2 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(expected_error, err.to_string()),
  }

  error_count = 0;
  expected_error = "Bulk Try Aggregation:\n".to_string();

  let err_string = err!("String Error 4");
  let err_bool = err!("Boolean Error 5");
  let err_vec = err!("Vector Error 6");

  let err_string2 = err!("String Error 7");
  let err_bool2 = err!("Boolean Error 8");
  let err_vec2 = err!("Vector Error 9");

  // Test 3
  //   Create a nested struct with multiple errors
  let test3 = bulk_try! {
    TestStruct {
      string: err_string?,
      boolean: err_bool?,
      vector: err_vec?,
      next: Some(
        Box::new(
          TestStruct {
            string: err_string2?,
            boolean: err_bool2?,
            vector: err_vec2?,
            next: None,
          }
        )
      ),
    }
  };

  match test3 {
    Ok(val) => panic!(
      "Test 3 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(expected_error, err.to_string()),
  }

  error_count = 0;
  expected_error = "Bulk Try Aggregation:\n".to_string();

  let ok_string = ok!("Test4_String".to_string());
  let ok_bool = ok!(false);
  let ok_vec = ok!(vec![4, 8, -50]);

  let ok_string2 = ok!("Test_String".to_string());
  let err_bool2 = err!("Boolean Error 10");
  let ok_vec2 = ok!(vec![6, -200, 38]);

  // Test 4
  //   Create a nested struct with only one error
  let test4 = bulk_try! {
    TestStruct {
    string: ok_string?,
    boolean: ok_bool?,
    vector: ok_vec?,
    next: Some(
      Box::new(
        TestStruct {
          string: ok_string2?,
          boolean: err_bool2?,
          vector: ok_vec2?,
          next: None,
        }
      )
    ),
  }
  };

  match test4 {
    Ok(val) => panic!(
      "Test 4 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(expected_error, err.to_string()),
  }
}

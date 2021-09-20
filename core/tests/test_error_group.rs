//! Testing for the ErrorGroup

mod common;
use common::*;

/// Test that the internals of errors are ok
fn cmp<T: Eq>(left: Result<T>, right: Result<T, &str>) -> bool {
  match (left, right) {
    (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
    (Ok(l), Ok(r)) => l == r,
    (Err(l), Err(r)) => match l.to_string().eq(&r.to_string()) {
      true => true,
      false => {
        println!("No Match:\n\tEval:     '{}'\n\tExpected: '{}'", l, r);
        false
      }
    },
  }
}

#[test]
fn test_error_group() -> () {
  let mut group: ErrorGroup = ErrorGroup::new(None);

  let value1: Result<&str, TestErr> = Ok("Ok does nothing");
  assert!(cmp(group.extract(value1), Ok("Ok does nothing")));

  let value2: Result<(), TestErr> = Err(TestErr("Value2 Error".to_string()));
  assert!(cmp(
    group.extract(value2),
    Err("(Extracted) - \"Value2 Error\"")
  ));

  let value3: Result<(), TestErr> = Err(TestErr("Value3 Error".to_string()));
  assert!(cmp(
    group.extract(value3),
    Err("(Extracted) - \"Value3 Error\"")
  ));
}

#[test]
#[allow(unused_assignments)]
fn test_extract_errors() -> () {
  use allwhat::extract_errors;
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
  let display = "Extracted Errors:\n\t0) Forced Error for val 2\n\t1) Forced 3 with a context\n\t2) Invalid: String 4\n\t3) String 5 errored with context\n".to_string();
  assert_eq!(err_res.to_string(), display);

  let debug = "ErrorGroup { label: Some(\"Extracted Errors\"), errors: [Forced Error for val 2, Forced 3 with a context\n\nCaused by:\n    Forced Error for val 3, Invalid: String 4, String 5 errored with context\n\nCaused by:\n    Invalid: String 5] }".to_string();
  assert_eq!(format!("{:?}", err_res), debug);
}

//! Test the ExprField path

use anyhow::{anyhow, Error as AnyhowError, Result};
use std::boxed::Box;

use allwhat::{bulk_try, prelude::ErrorGroup};

struct TestField {
  bare_int: i32,
  ok_int: Result<i32>,
  err_int: Result<i32>,
  next: Result<Box<TestField>>,
}

#[test]
fn test_expr_field() {
  // Bypass cloning and just generate some values
  // TODO: Add randomization to these tests
  fn new_tester() -> Result<TestField, AnyhowError> {
    Ok(TestField {
      bare_int: 1,
      ok_int: Ok(2),
      err_int: Err(anyhow!("Int error 3")),
      next: Ok(Box::new(TestField {
        bare_int: 4,
        ok_int: Ok(5),
        err_int: Err(anyhow!("Int error 6")),
        next: Ok(Box::new(TestField {
          bare_int: 7,
          ok_int: Ok(8),
          err_int: Err(anyhow!("Int error 9")),
          next: Err(anyhow!("Next error 10")),
        })),
      })),
    })
  }

  // Test 1:
  //   Unwrap the a basic struct
  let value = new_tester();
  let test1: Result<TestField, ErrorGroup> = bulk_try! { value? };
  assert_eq!(test1.unwrap().bare_int, 1);

  // Test 2:
  //   Unwrap the base level Ok
  let value = new_tester();
  let test2: Result<i32, ErrorGroup> = bulk_try! { value?.ok_int? };
  match test2 {
    Ok(val) => assert_eq!(val, 2),
    Err(err) => panic!("Test 2 was supposed to be Ok. Instead returned: {:#?}", err),
  }

  // Test 3:
  //   Make sure the base level error bubbles up
  let value = new_tester();
  let test3: Result<i32, ErrorGroup> = bulk_try! { value?.err_int? };
  match test3 {
    Ok(val) => panic!(
      "Test 3 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(
      "Bulk Try Aggregation:\n\t1) Int error 3\n".to_string(),
      err.to_string()
    ),
  }

  // Test 3:
  //   Make sure the base level error bubbles up
  let value = new_tester();
  let test3: Result<i32, ErrorGroup> = bulk_try! { value?.err_int? };
  match test3 {
    Ok(val) => panic!(
      "Test 3 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(
      "Bulk Try Aggregation:\n\t1) Int error 3\n".to_string(),
      err.to_string()
    ),
  }

  // Test 4:
  //   Check the deeply nested values works
  let value = new_tester();
  let test4: Result<i32, ErrorGroup> = bulk_try! { value?.next?.next?.bare_int };
  match test4 {
    Ok(val) => assert_eq!(val, 7),
    Err(err) => panic!("Test 4 was supposed to be Ok. Instead returned: {:#?}", err),
  }

  // Test 5:
  //   Check the nesting throws an error when it runs out of depth
  let value = new_tester();
  let test5: Result<i32, ErrorGroup> = bulk_try! { value?.next?.next?.next?.next?.next?.err_int? };
  match test5 {
    Ok(val) => panic!(
      "Test 5 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => assert_eq!(
      "Bulk Try Aggregation:\n\t1) Next error 10\n".to_string(),
      err.to_string()
    ),
  }
}

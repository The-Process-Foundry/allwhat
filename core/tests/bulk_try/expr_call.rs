//! Test Expr Call

use allwhat::{bulk_try, prelude::ErrorGroup};

/*
// A very simple function that also determines the type of error
fn to_ok<T>(val: T) -> Result<T, String> {
  Ok(val)
}

fn to_err<T>(msg: &'static str) -> Result<T, String> {
  Err(anyhow!(msg))
}

#[test]
fn test_expr_call_result() {
  // The simplest case: one parameter and nothing is broken
  let test1: Result<bool, ErrorGroup> = bulk_try! {to_ok(true)?};
  assert!(test1.unwrap());

  let test2: Result<bool, ErrorGroup> = bulk_try! {to_err("Test2 is an error")?};
  match test2 {
    Ok(_) => panic!("Test 2 should not be OK"),
    Err(err) => {
      assert_eq!(err.len(), 1);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t0) Test2 is an error\n".to_string()
      );
    }
  }
}
*/

#[test]

fn test_expr_call_params() {
  fn one_param(a: i32) -> Result<i32, String> {
    if a < 64 {
      Ok(a)
    } else {
      Err(format!("one_param received a large value: {}", a))
    }
  }

  // Test 1:
  //   The most basic, unwrap a valid parameter and return the Ok result
  let mut param: Result<i32, String> = Ok(1);
  let test1 = bulk_try! { one_param(param?) };
  assert_eq!(test1.unwrap().unwrap(), 1);

  // Test 2:
  //   Have the try find an error, so we end up with an error group containing it
  param = Err(anyhow!("Now the param is an error"));
  let test2 = bulk_try! { one_param(param?) };
  match test2 {
    Ok(val) => panic!(
      "Test 2 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => {
      assert_eq!(err.len(), 1);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t1) Now the param is an error\n".to_string()
      );
    }
  }

  // Test 3
  //   Add a trailing try and ensure the error group still only contains the parameter error
  param = Err(anyhow!("The param is still an error"));
  let test3: Result<i32, ErrorGroup> = bulk_try! { one_param(param?)? };
  match test3 {
    Ok(val) => panic!("Test 3 was supposed to be an error. Received: {:#?}", val),
    Err(err) => {
      assert_eq!(err.len(), 1);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t1) The param is still an error\n".to_string()
      );
    }
  }

  // Test 4:
  //   The paramater is good, but causes an error. It should return an Ok wrapping an error
  param = Ok(100);
  let test4: Result<Result<i32, String>, ErrorGroup> = bulk_try! { one_param(param?) };

  match &test4 {
    Ok(Err(err)) => assert_eq!(
      err.to_string(),
      "one_param received a large value: 100".to_string()
    ),
    Ok(Ok(_)) => panic!("Bug in test 4. The test function returned an Ok instead of an error"),
    Err(err) => panic!(
      "Test 4 should be an OK wrapping an error. Instead, the internal is:\n{:#?}\n\n",
      err
    ),
  }

  // Test 5:
  //   Same as Test 4, but now we add another try operator to unwrap the output of the call
  param = Ok(100);
  let test5 = bulk_try! { one_param(param?)? };

  match &test5 {
    Err(err) => assert_eq!(
      err.to_string(),
      "Bulk Try Aggregation:\n\t1) one_param received a large value: 100\n".to_string()
    ),
    Ok(err) => panic!(
      "Test 5 should have returned an Error. Instead it returned:\n{:#?}",
      err
    ),
  }

  fn multi_param(a1: i32, b2: i32, c3: i32, d4: i32, e5: i32, f6: i32) -> Result<i32, String> {
    one_param(a1 + b2 + c3 + d4 + e5 + f6)
  }

  // Test 6
  //   Work with multiple parameters for the same function, some of them errors.
  param = Ok(1);
  let mut param2: Result<i32, String> = Ok(2);
  let mut param3 = 4;
  let mut param4: Result<i32, String> = Err(anyhow!("Param4 Error"));
  let mut param5: Result<i32, String> = Ok(16);
  let mut param6: Result<i32, String> = Err(anyhow!("Param6 Error"));

  let test6 = bulk_try! {multi_param(param?, param2?, param3, param4?, param5?, param6?)};
  match &test6 {
    Ok(val) => panic!(
      "Test 4 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => {
      assert_eq!(err.len(), 2);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t1) Param4 Error\n\t2) Param6 Error\n".to_string()
      );
    }
  }

  // Test 7
  //   Same as test 6, but add the trailing try operator again
  param = Ok(1);
  param2 = Ok(2);
  param3 = 4;
  param4 = Err(anyhow!("Param4 Error"));
  param5 = Ok(16);
  param6 = Err(anyhow!("Param6 Error"));
  let test7 = bulk_try! {multi_param(param?, param2?, param3, param4?, param5?, param6?)?};
  match &test7 {
    Ok(val) => panic!(
      "Test 7 was supposed to be an error. Instead returned: {:#?}",
      val
    ),
    Err(err) => {
      assert_eq!(err.len(), 2);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t1) Param4 Error\n\t2) Param6 Error\n".to_string()
      );
    }
  }

  // Test 8
  //   Make the result a success, and check the result
  param = Ok(1);
  param2 = Ok(2);
  param3 = 4;
  param4 = Ok(8);
  param5 = Ok(16);
  param6 = Ok(32);
  let test8 = bulk_try! {multi_param(param?, param2?, param3, param4?, param5?, param6?)?};
  match test8 {
    Err(err) => panic!("Test 8 was supposed to be Ok. Instead returned: {:#?}", err),
    Ok(val) => assert_eq!(val, 63),
  }

  // Test 9
  //   All the parameters are correct, but the function causes an error
  param = Ok(1);
  param2 = Ok(2);
  param3 = 4;
  param4 = Ok(8);
  param5 = Ok(16);
  param6 = Ok(200);
  let test9 = bulk_try! {multi_param(param?, param2?, param3, param4?, param5?, param6?)};
  match &test9 {
    Ok(Err(err)) => {
      assert_eq!(
        err.to_string(),
        "one_param received a large value: 231".to_string()
      );
    }
    Ok(Ok(_)) => panic!("Bug in test 10. The test function returned an Ok instead of an error"),
    Err(err) => panic!(
      "Test 10 should be an OK wrapping an error. Instead, the internal is:\n{:#?}\n\n",
      err
    ),
  }

  // Test 10
  //   All the parameters are correct, but the function causes an error
  param = Ok(1);
  param2 = Ok(2);
  param3 = 4;
  param4 = Ok(8);
  param5 = Ok(16);
  param6 = Ok(200);
  // Add in a trailing try when the params already failed. This should return the same as before
  let test10 = bulk_try! {multi_param(param?, param2?, param3, param4?, param5?, param6?)?};
  match &test10 {
    Err(err) => {
      assert_eq!(err.len(), 1);
      assert_eq!(
        err.to_string(),
        "Bulk Try Aggregation:\n\t1) one_param received a large value: 231\n".to_string()
      )
    }
    Ok(err) => panic!(
      "Test 10 should have returned an valid error. Instead it returned Ok:\n{:#?}",
      err
    ),
  }
}

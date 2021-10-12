//! Test using anyhow as the grouped error type

mod common;

// use anyhow::Context;

/*

#[test]
/// Test try_mut with a fancy hashmap
fn test_try_mut() {
  use common::*;
  use std::collections::HashMap;
  use std::boxed::Box;

  // #[derive(TestMut)]
  #[derive(Debug)]
  struct TestAcc {
    value: std::collections::HashMap<i32, i32>,

    // added by the derive
    /// This holds pointers to the derived functions needed to
    _try_mut_actions:

  }

  enum TestAccTryAction {
    Insert(
      //patch
      Box<dyn Fn(&self) -> i32>,
      // Action
    )
  }

  // Created by derive
  // Items from the context that needs
  pub enum TestAccTryPatch {
    Insert(i32),
  }


  // Created by derive
  // Actions that can be rolled back on error
  pub enum TestAccTryAction {
    Insert(i32, i32),
  }


  impl TryMutAction for TestAccTryAction {
    type Item = TestAcc;
    type Patch = TestAccTryPatch;
    type Error = TestAcc::Error;


  }


  impl TestAcc {
    fn new() -> TestAcc {
      TestAcc {
        value: HashMap::new(),
      }
    }


    // Select functions to wrap in a rollback
    // $[try_mut(value.insert(i32, i32))]
    fn try_insert(&mut self, act: TestAccAction) -> {

    }
  }



  impl TryMut for TestAcc {
    type Error = anyhow::Error;

    // fn try_mut<F>(&mut self, func: F) -> PoisonErr<Self::Error>
    // where
    //   F: FnMut(&mut Self) -> Result<{}, Self::Error>,
    // {
    //   match func(self) {
    //     Some(value) => anyhow!("Duplicate Key"),
    //   }
      // match func(self) {

      //   PoisonErr::Ok => (),
      //   PoisonErr::Err(err) => acc.append(err),
      //   PoisonErr::Poisoned(err1, err2) => {
      //     let error = anyhow!(err2).context(err1);
      //     println!("{}", error);
      //   }
    }
  }


// Try to insert a value
let adder : Box<dyn Fn(&mut TestAcc, i32, i32) -> PoisonErr<AnyhowError>> =
  |map: &mut TestAcc, key, value|
    match map.insert(key, value) {
      Some(old) => {}
      None => PoisonErr::Ok(()),
    };

  let remove: dyn Fn(TestAcc, i32) -> PoisonErr<AnyhowError> {
  {
    unimplemented!("'' still needs to be implemented")
  }

  fn func() -> Result<i32, anyhow::Error> {
    let res = BatchResult::try_fold(TestAcc::new(), 1..6, |_, i| {
      PoisonErr::Err(anyhow!("Error #{}", i))
    });

    println!("{:?}", res);
    assert_eq!(res.count(), 5);
    assert_eq!(res.count_error(), 5);
    assert_eq!(res.count_valid(), 0);

    let err: ErrorGroup = res.as_result().unwrap_err();
    println!("Finish: {}", err);
    Ok(Err(err)?)
  }

  let res: Result<i32, anyhow::Error> = func();
  assert!(res.is_err())
}

*/

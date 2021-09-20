//! Test using anyhow as the grouped error type

mod common;

// use anyhow::Context;

/*
/// There is a strange bug where anyhow would blow up when using a try_from from using the ?.
/// Level 0 of the stack points to a macro in anyhow. It was definitely an error going in and I
/// cannot find where it is actually trying the unwrap, let alone where an Ok would be expected.
/// Modifying my code to be more generic so it accepts items other than anyhow items

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Could not validate the headers for LABdivus

Caused by:
    0: The headers are invalid because:
    1: ErrorGroup:
         0) Validate Headers is not implemented
        1) Validate Headers is not implemented
        2) Validate Headers is not implemented

Stack backtrace:
   0: anyhow::error::<impl core::convert::From<E> for anyhow::Error>::from
             at /home/dfogelson/Foundry/anyhow/src/error.rs:524:25
   1: <T as core::convert::Into<U>>::into
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/core/src/convert/mod.rs:540:9
   2: allwhat::annotation::BatchResult<T>::as_result
             at /home/dfogelson/Foundry/allwhat/src/annotation.rs:18:22
   3: subpar::base::row::RowTemplate::validate_headers
             at /home/dfogelson/Foundry/Subpar/subpar/src/base/row.rs:159:5
   4: subpar::csv::io::reader::CsvReader::new
             at /home/dfogelson/Foundry/Subpar/subpar/src/csv/io/reader.rs:150:31
   5: fhl_server::test_to_row
             at ./fhl-server/src/main.rs:78:20
   6: fhl_server::main
             at ./fhl-server/src/main.rs:98:14
   7: core::ops::function::FnOnce::call_once
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/core/src/ops/function.rs:227:5
   8: std::sys_common::backtrace::__rust_begin_short_backtrace
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys_common/backtrace.rs:125:18
   9: std::rt::lang_start::{{closure}}
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/rt.rs:63:18
  10: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/core/src/ops/function.rs:259:13
  11: std::panicking::try::do_call
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:403:40
  12: std::panicking::try
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:367:19
  13: std::panic::catch_unwind
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panic.rs:129:14
  14: std::rt::lang_start_internal::{{closure}}
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/rt.rs:45:48
  15: std::panicking::try::do_call
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:403:40
  16: std::panicking::try
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:367:19
  17: std::panic::catch_unwind
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panic.rs:129:14
  18: std::rt::lang_start_internal
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/rt.rs:45:20
  19: std::rt::lang_start
             at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/rt.rs:62:5
  20: main
  21: __libc_start_main
  22: _start', fhl-server/src/main.rs:78:82

*/

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

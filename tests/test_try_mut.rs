//! Test the mutability implementation

mod common;

/// Basic test of mutability of a primitive.
#[test]
fn test_i32() {
  use common::*;

  let mut action = i32TryMut::Op(Box::new(|item: &mut i32| {
    // Convert it regardless
    *item += 10;

    if *item % 2 == 0 {
      // Evens are Ok
      Ok(())
    } else {
      // Odds throw an error
      Err(anyhow!("Eew, an odd"))
    }
  }));

  for i in 0..10 {
    println!("Running i: {}", i);
    let mut j = i.clone();
    match j.try_mut(&mut action) {
      PoisonedMut::Ok => {
        assert_eq!(j, i + 10);
      }
      PoisonedMut::Err(err) => {
        assert_eq!(j, i);
        assert_eq!(format!("{}", err), "Eew, an odd");
      }
      PoisonedMut::Poisoned(_, _) => panic!("This should never be poisoned"),
    }
  }
}

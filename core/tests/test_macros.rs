//! Test the map struct macro
//!
//! Each type of expression that can contain a question mark is tested in its own file. Cleaner
//! just to import the directory

pub mod common;
pub use common::ERR;

pub mod bulk_try;

// Test Enum

// Test ExprField

// Test Tuples

// Test Struct

// Test Nested

// Test func with try params
//
// let value = DemoStruct {
//   value1: value1?,
//   value_from_func: func(param1?, param2?)?,
// })

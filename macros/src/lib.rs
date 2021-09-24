#![feature(proc_macro_diagnostic)]
//! Support macros that implement uses of the allwhat aggregators

// Dumps messages to the screen using the env_logger
use std::sync::Mutex;

#[macro_use]
pub(crate) mod logger;
lazy_static::lazy_static! {
  pub(crate) static ref LOG: Mutex<logger::ScreenLogger> = {
    use std::io::Write;
    let mut builder = env_logger::Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{}", record.args()));
    builder.init();
    Mutex::new(logger::ScreenLogger::new())};
}

pub(crate) mod try_assign;

#[proc_macro]
/// Evaluate an expression used for an assignment and aggregate all the errors captures by the try
/// operator
///
/// Fields that have results should be annotated with question marks, like a standard result, but
/// instead of returning the first it will gather them all. The output of this macro is
/// Result<T, ErrorGroup>.
pub fn try_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  try_assign::run_macro(proc_macro2::TokenStream::from(item)).into()
}

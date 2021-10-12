#![feature(proc_macro_diagnostic)]
#![feature(trace_macros)]
//! Support macros that implement uses of the allwhat aggregators

lazy_static::lazy_static! {
  pub(crate) static ref LOG: std::sync::Mutex<ymlog::YmLog<std::fs::File>> = {
    let file = std::fs::OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open("./output_log.yml")
      .unwrap();
    let mut logger = ymlog::YmLog::new();
    logger.set_output(file);
    std::sync::Mutex::new(logger)
  };
}

pub(crate) mod bulk_try;

#[proc_macro]
/// Evaluate an expression used for an assignment and aggregate all the errors captures by the try
/// operator
///
/// Fields that have results should be annotated with question marks, like a standard result, but
/// instead of returning the first it will gather them all. The output of this macro is
/// Result<T, ErrorGroup>.
pub fn bulk_try(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    {
        let _x = crate::LOG.lock();
    }
    bulk_try::run_macro(proc_macro2::TokenStream::from(item)).into()
}

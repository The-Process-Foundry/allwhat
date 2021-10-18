//! The custom structures with parses needed to build the macros
//!
//! This is going to work recursively, so we're going to keep going down into the tree
//! even though the try operator should only appear on leaves. The recursion handles odd edge cases
//! such as:
//!
//! let value = DemoStruct {
//!   value1: value1?,
//!   value_from_func: func(param1?, param2?)?,
//! })
//!
//!
//! This is a pretty simple recursion used to aggregate all the errors that can be thrown by the
//! root expression. It is done using the following recursive steps:
//!
//!   - Create a new error aggregator
//!   - Match the expression
//!   - If it is a try, send the wrapped expression to step two
//!   - Iterate all parameters and other locations for potential tries
//!   - If any tries are found
//!      - Make a temporary variable assignment, matching the expression from inside the try
//!      - Add any errors matched to the aggregator
//!      - Replace found errors with a unit error
//!      - Update "found_tries" flag to true
//! parse all the code that can handle the question mark
//! operator, extract them to temporary variables,
//!
//!
//! FIXME: Document the error "cannot infer type"
/*
error[E0282]: type annotations needed
--> /home/dfogelson/Foundry/anyhow/src/macros.rs:177:31
 |
168 | / macro_rules! anyhow {
169 | |     ($msg:literal $(,)?) => {
170 | |         // Handle $:literal as a special case to make cargo-expanded code more
171 | |         // concise in the common case.
...   |
177 | |             error => (&error).anyhow_kind().new(error),
 | |                               ^^^^^^^^^^^ cannot infer type
...   |
182 | |     };
183 | | }
 | |_- in this expansion of `anyhow::anyhow!` (#2)
 |
::: /home/dfogelson/Foundry/allwhat/macros/src/lib.rs:27:1
 |
27  |   pub fn bulk_try(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
 |   ------------------------------------------------------------------------- in this expansion of `bulk_try!` (#1)
 |
::: core/tests/expr_call.rs:13:41
 |
13  |     let test1: Result<bool, ErrorGroup> = bulk_try! {Ok(true)?};
 |                                           ---------------------
 |                                           |
 |                                           in this macro invocation (#1)
 |                                           in this macro invocation (#2)
 |
 = note: type must be known at this point
*/

use proc_macro2::{Span, TokenStream};
use quote::quote;

pub use syn::{
  parse::{Error, Result},
  punctuated::Punctuated,
  Expr, ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBox,
  ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField, ExprForLoop, ExprGroup,
  ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen,
  ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock,
  ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprWhile, ExprYield, FieldValue, Ident, Index,
  Member, Token,
};

use ymlog::ymlog;

/// Tells a parent whether a descendent was unwrapped at any point and (un)modified expression
pub struct Unwrapped {
  /// The number of tries found in sub-expressions of the unwrapped item
  pub try_count: i16,
  pub expr: TokenStream,
}

impl std::fmt::Debug for Unwrapped {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Unwrapped")
      .field("try_count", &self.try_count)
      .field("expr", &{
        let expr = &self.expr;
        quote! {#expr}
      })
      .finish()
  }
}

impl std::fmt::Display for Unwrapped {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Unwrapped")
      .field("try_count", &self.try_count)
      .field("expr", &{
        let expr = &self.expr;
        format!("{}", quote! {#expr})
      })
      .finish()
  }
}

impl Unwrapped {
  /// Print the debug to the default logger
  pub fn _log(&self) {
    ymlog!("_" => format!("{:#?}", self));
  }

  /// Printing the contents of the unwrapped to the log, dedent the logger, and wrap in Ok
  ///
  /// The primary purpose of this is to reduce noise in the code created by debugging statements
  pub fn log_ok(self) -> Result<Unwrapped> {
    let expr = &self.expr;
    ymlog!("_+" => "Unwrap result:");
    ymlog!("_" => "Try Count = {}", self.try_count);
    ymlog!("_--" => "Final Expression:\n{}", quote!{#expr});
    // ymlog!("Finalizing {}", self);
    Ok(self)
  }
}

/// A trait to find where results need to be unwrapped
pub trait Questionable {
  /// Recursively check to unwrap try expressions and return the inner expression if so.
  ///
  /// If there is a viable question mark in the scope of the group, it adds a temporary variable
  /// to check for an error and add it to the "global" error group.
  fn unwrap_tries(self) -> Result<Unwrapped>;

  /// Get a pretty name from the item, in case the parent doesn't know it.
  fn get_ident(&self) -> Ident {
    unimplemented!("get_ident has not been implemented for the current type")
  }

  /// Add a warning to the item
  fn warn(&self, msg: &str) {
    Span::call_site().unwrap().warning(msg).emit();
  }

  /// Add an error to the given item
  fn error(&self, msg: &str) {
    Span::call_site().unwrap().error(msg).emit();
  }
}

impl Questionable for Expr {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    match self {
      Expr::Array(expr_array) => expr_array.unwrap_tries(),
      Expr::Assign(expr_assign) => expr_assign.unwrap_tries(),
      Expr::AssignOp(expr_assign_op) => expr_assign_op.unwrap_tries(),
      Expr::Async(expr_async) => expr_async.unwrap_tries(),
      Expr::Await(expr_await) => expr_await.unwrap_tries(),
      Expr::Binary(expr_binary) => expr_binary.unwrap_tries(),
      Expr::Block(expr_block) => expr_block.unwrap_tries(),
      Expr::Box(expr_box) => expr_box.unwrap_tries(),
      Expr::Break(expr_break) => expr_break.unwrap_tries(),
      Expr::Call(expr_call) => expr_call.unwrap_tries(),
      Expr::Cast(expr_cast) => expr_cast.unwrap_tries(),
      Expr::Closure(expr_closure) => expr_closure.unwrap_tries(),
      Expr::Continue(expr_continue) => expr_continue.unwrap_tries(),
      Expr::Field(expr_field) => expr_field.unwrap_tries(),
      Expr::ForLoop(expr_for_loop) => expr_for_loop.unwrap_tries(),
      Expr::Group(expr_group) => expr_group.unwrap_tries(),
      Expr::If(expr_if) => expr_if.unwrap_tries(),
      Expr::Index(expr_index) => expr_index.unwrap_tries(),
      Expr::Let(expr_let) => expr_let.unwrap_tries(),
      Expr::Lit(expr_lit) => expr_lit.unwrap_tries(),
      Expr::Loop(expr_loop) => expr_loop.unwrap_tries(),
      Expr::Macro(expr_macro) => expr_macro.unwrap_tries(),
      Expr::Match(expr_match) => expr_match.unwrap_tries(),
      Expr::MethodCall(expr_method_call) => expr_method_call.unwrap_tries(),
      Expr::Paren(expr_paren) => expr_paren.unwrap_tries(),
      Expr::Path(expr_path) => expr_path.unwrap_tries(),
      Expr::Range(expr_range) => expr_range.unwrap_tries(),
      Expr::Reference(expr_reference) => expr_reference.unwrap_tries(),
      Expr::Repeat(expr_repeat) => expr_repeat.unwrap_tries(),
      Expr::Return(expr_return) => expr_return.unwrap_tries(),
      Expr::Struct(expr_struct) => expr_struct.unwrap_tries(),
      Expr::Try(expr_try) => expr_try.unwrap_tries(),
      Expr::TryBlock(expr_try_block) => expr_try_block.unwrap_tries(),
      Expr::Tuple(expr_tuple) => expr_tuple.unwrap_tries(),
      Expr::Type(expr_type) => expr_type.unwrap_tries(),
      Expr::Unary(expr_unary) => expr_unary.unwrap_tries(),
      Expr::Unsafe(expr_unsafe) => expr_unsafe.unwrap_tries(),
      Expr::Verbatim(token_stream) => token_stream.unwrap_tries(),
      Expr::While(expr_while) => expr_while.unwrap_tries(),
      Expr::Yield(expr_yield) => expr_yield.unwrap_tries(),
      Expr::__TestExhaustive(_) => panic!("Attempting to process a private expression"),
    }
  }
}

/// The explicit question mark operator call
///
/// It can be used in a multitude of places, but it wraps only a single expression. The most common
/// use case in the bulk situation is as a simple unwrap, where the underlying expression is not
/// a result.
impl Questionable for ExprTry {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let expr = &self.expr;
    ymlog! {"_+" => "Processing TryExpr: ({}) ?", quote!{ #expr }};

    let Unwrapped { try_count, expr } = self.expr.unwrap_tries()?;

    // If there was a nested try, errors may have already been extracted, which causes a type
    // mismatch on the error grouping
    ymlog!("TryExpr's inner value contained {} tries", try_count);
    let expr_value = match try_count == 0 {
      true => quote! { try_expr },
      false => quote! { try_expr.unwrap() },
    };

    // Get a count of errors found before and after evaluating the internal expression. If it is
    // more than zero, it means the try should not be executed.
    let unwrap = quote! {{
      let pre_try_error_count = __error_group.len();
      let try_expr = { #expr };

      match __error_group.len() == pre_try_error_count {
        false => Err(()),
        true =>
          match #expr_value {
            Ok(val) => Ok(val),
            Err(err) => {
              __error_group.append(anyhow::anyhow!(err));
              Err(())
            },
          },
      }
    }};

    Unwrapped {
      try_count: try_count + 1,
      expr: unwrap,
    }
    .log_ok()
  }
}

/// A stand-alone function call, both functions and closures
impl Questionable for ExprCall {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let cloned_call = quote! {#self};
    ymlog!("_+" =>  "Processing a call: {}", cloned_call);

    let mut count = 0;
    let mut unwrap = TokenStream::new();
    let mut args: Punctuated<Expr, Token![,]> = Punctuated::new();

    for (i, param) in self.args.into_iter().enumerate() {
      let param_clone = quote! { #param };
      let name = Ident::new(&format!("param_{}", i), Span::call_site());

      let Unwrapped { try_count, expr } = param.unwrap_tries()?;
      if try_count > 0 {
        count += try_count;
        ymlog!("_" => "Processing Param {}", quote! {#name});
        unwrap.extend(quote! {
          let #name = { #expr };
        });

        args.push(syn::parse2(quote! { #name.unwrap() })?);
      } else {
        args.push(syn::parse2(param_clone)?);
      }
    }

    if unwrap.is_empty() {
      ymlog! {"_" => "Returning the unchanged call"};
      Unwrapped {
        try_count: count,
        expr: cloned_call,
      }
    } else {
      let updated_call = ExprCall {
        args,
        ..{ syn::parse2(cloned_call)? }
      };
      ymlog! {"T" => "Returning an updated call: {}", quote!{#updated_call}};

      Unwrapped {
        try_count: count + 1,
        expr: quote! {
          let pre_try_err_count = __error_group.len();
          #unwrap

          match __error_group.len() == pre_try_err_count {
            true => Ok(#updated_call),
            false => Err(()),
          }
        },
      }
    }
    .log_ok()
  }
}

impl Questionable for ExprField {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let member = &self.member;
    let base = &self.base;
    // bulk_try! { tester.err_int? };
    ymlog!("_+" => "Processing ExprField");
    ymlog!("base: {}", quote!(#base));

    let Unwrapped { try_count, expr } = self.base.unwrap_tries()?;

    match try_count == 0 {
      true => {
        ymlog! {"_" => "No tries found in the FieldExpr"};
        Unwrapped {
          try_count,
          expr: quote! { #expr.#member},
        }
      }
      false => {
        ymlog!("Found {} tries in the field", try_count);
        Unwrapped {
          try_count,
          expr: quote! {{
            let pre_try_err_count = __error_group.len();
            let base = #expr;

            match __error_group.len() == pre_try_err_count {
              true => Ok(base.unwrap().#member),
              false => Err(()),
            }
          }},
        }
      }
    }
    .log_ok()
  }
}

impl Questionable for ExprStruct {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let path = &self.path;
    ymlog!( "_+" => "Processing ExprStruct");
    ymlog!("Path: {}", quote! {#path});

    // Frontload quoting self, since it will get consumed while looking for tries
    let cloned_struct = quote! {#self};

    let mut count = 0;
    let mut unwrap = TokenStream::new();
    let mut fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();

    for field in self.fields.into_iter() {
      // Make a copy of the token stream since we cannot copy and unwrap_tries consumes
      let cloned_field = quote! { #field };
      let name = field.get_ident();

      let Unwrapped { try_count, expr } = field.unwrap_tries()?;
      if try_count > 0 {
        // Keep a running total of the number of tries found
        count += try_count;
        // TODO: Is the annotation correct here? I believe it should stick with the FieldValue and
        //       not the expression, but I'm not sure how to test it or the edge cases
        unwrap.extend(quote! {
          let #name = { #expr };
        });
        fields.push(FieldValue {
          expr: syn::parse2(quote! { #name.unwrap() })?,
          ..{ syn::parse2(cloned_field)? }
        });
      } else {
        ymlog!("T" => "No try on the field {}", quote!(#name));
        fields.push(syn::parse2(cloned_field)?);
      }
    }

    if unwrap.is_empty() {
      Unwrapped {
        try_count: count,
        expr: cloned_struct,
      }
    } else {
      let struct_ident = self.path;
      Unwrapped {
        try_count: count,
        expr: quote! {
          let error_count = __error_group.len();
          #unwrap

          match __error_group.len() > error_count {
            true => Err(()),
            false => Ok(#struct_ident { #fields }),
          }
        },
      }
    }
    .log_ok()
  }
}

impl Questionable for FieldValue {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    match &self.member {
      Member::Named(ident) => {
        ymlog!("_" => "Processing the FieldValue named {}", ident.to_string());
        self.expr.unwrap_tries()?
      }
      Member::Unnamed(index) => {
        ymlog!("_" => "Processing the unnamed FieldValue at index {}", index.index);
        self.expr.unwrap_tries()?
      }
    }
    .log_ok()
  }

  fn get_ident(&self) -> Ident {
    match &self.member {
      Member::Named(ident) => ident.clone(),
      Member::Unnamed(index) => Ident::new(&index.index.to_string(), index.span),
    }
  }
}

/*
impl Questionable for ExprMethodCall {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let cloned_call = quote! {#self};
    ymlog!("_+" => "Processing Method Call: {}", cloned_call);

    let mut unwrap = TokenStream::new();
    // let mut assign = TokenStream::new();

    // for param in self.args.into_iter() {
    //   let name = param.get_ident();
    //   let Unwrapped { try_count, expr } = param.unwrap_tries()?;
    //   if try_count {
    //     ymlog!("  - Processing Param {}", quote! {#name});
    //     unwrap.extend(quote! {
    //       let #name = { #expr };
    //     });

    //     let updated_expr: Expr = syn::parse2(quote! { #name.unwrap() })?;
    //     assign.extend(quote! {#updated_expr});
    //   }
    // }

    let mut count = 0;
    let mut args: Punctuated<Expr, Token![,]> = Punctuated::new();

    for (i, param) in self.args.into_iter().enumerate() {
      let param_clone = quote! { #param };
      let name = Ident::new(&format!("param_{}", i), Span::call_site());

      let Unwrapped { try_count, expr } = param.unwrap_tries()?;
      if try_count > 0 {
        count += 0;
        ymlog!("Processing Param {}", quote! {#name});
        unwrap.extend(quote! {
          let #name = { #expr };
        });

        args.push(syn::parse2(quote! { #name.unwrap() })?);
      } else {
        args.push(syn::parse2(param_clone)?);
      }
    }

    if unwrap.is_empty() {
      ymlog! {"Returning the unchanged method call"};
      Unwrapped {
        try_count: count,
        expr: cloned_call,
      }
    } else {
      let updated_call = ExprCall {
        args,
        ..{ syn::parse2(cloned_call)? }
      };
      ymlog! {"Returning an updated method call: {}", quote!{#updated_call}};

      Unwrapped {
        try_count: count,
        expr: quote! {
          #unwrap

          match __error_found {
            true => Err(()),
            false => Ok(#updated_call),
          }
        },
      }
    }
    .log_ok()
  }
}*/

/// A stand-alone function call, both functions and closures
impl Questionable for ExprMethodCall {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let cloned_call = quote! {#self};
    ymlog!("_+" =>  "Processing a method call: {}", cloned_call);

    // A running total of try expressions encountered
    let mut count = 0;
    let mut unwrap = TokenStream::new();
    let mut args: Punctuated<Expr, Token![,]> = Punctuated::new();

    // The receiver is the object before the dot. This is allowed to have a try, so we process it.
    let Unwrapped { try_count, expr } = self.receiver.unwrap_tries()?;
    count += try_count;
    let receiver = match try_count == 0 {
      true => None,
      false => Some(expr),
    };

    for (i, param) in self.args.into_iter().enumerate() {
      let param_clone = quote! { #param };
      let name = Ident::new(&format!("param_{}", i), Span::call_site());

      let Unwrapped { try_count, expr } = param.unwrap_tries()?;
      if try_count > 0 {
        count += try_count;
        ymlog!("_" => "Extracting try from param {}", quote! {#name});
        unwrap.extend(quote! {
          let #name = { #expr };
        });

        args.push(syn::parse2(quote! { #name.unwrap() })?);
      } else {
        args.push(syn::parse2(param_clone)?);
      }
    }

    let expr = match (&receiver, unwrap.is_empty()) {
      (Some(receiver), _) => {
        ymlog!("T_+" => "updated_call");
        ymlog!("T_" => "Receiver: {}", quote! {#receiver});
        ymlog!("T_" => "Args: {}", quote! {#args});
        ymlog!("T_" => "Updating full method call (receiver and args)");

        let updated_call = ExprMethodCall {
          receiver: Box::new(syn::parse2(quote!(receiver.unwrap())).unwrap()),
          args,
          ..{ syn::parse2(cloned_call)? }
        };

        ymlog!("T_-" => "{}", quote!(#updated_call));

        quote! {{
          let error_count = __error_group.len();
          // Checking the receiver
          let receiver = #receiver;

          // Check all the parameters
          #unwrap

          match error_count == __error_group.len() {
            true => {
              Ok(#updated_call)
            }
            false => Err(()),
          }
        }}
      }
      (None, false) => {
        ymlog!("Updating arguments only");
        let updated_call = ExprMethodCall {
          args,
          ..{ syn::parse2(cloned_call)? }
        };

        quote! {
          let error_count = __error_group.len();
          #unwrap

          match error_count == __error_group.len() {
            true => {
              Ok(#updated_call)
            }
            false => Err(()),
          }

        }
      }
      (None, true) => {
        ymlog!("Returning the unchanged call");
        cloned_call
      }
    };

    ymlog!("Updated method call expr");
    ymlog!("{}", expr);
    Unwrapped {
      try_count: count,
      expr,
    }
    .log_ok()
    /*
    } else {
      let updated_call = ExprCall {
        args,
        ..{ syn::parse2(cloned_call)? }
      };
      ymlog! {"T" => "Returning an updated call: {}", quote!{#updated_call}};

      Unwrapped {
        try_count: count + 1,
        expr: quote! {
          let pre_try_err_count = __error_group.len();
          #unwrap

          match __error_group.len() == pre_try_err_count {
            true => Ok(#updated_call),
            false => Err(()),
          }
        },
      }
    }
    */
  }
}

/// These are items which have no more nested expressions to be parsed. Closures and macros are
/// included because digging into them can have unintended consequences.
macro_rules! expressionless {
  ($($name:ident),+) => {
    $(
    impl Questionable for $name {
      fn unwrap_tries(self) -> Result<Unwrapped> {
        ymlog!("_" => "Expressionless Value {}: {}", stringify!($name),  quote!(#self));

        Ok(Unwrapped {
          try_count: 0,
          expr: quote! {#self},
        })
      }
    }
    )+
  }
}

expressionless!(ExprPath, ExprMacro, ExprLit, ExprCast, ExprClosure);

/// An Expr that I haven't had time to process yet, but may contain nested try blocks
macro_rules! unexamined_expr {
  ($($name:ident),+) => {
    $(
    impl Questionable for $name {
      fn unwrap_tries(self) -> Result<Unwrapped> {
        unimplemented!("Questionable not implemented for {} yet", stringify!{$name})
      }
    }
  )+
  }
}

unexamined_expr!(
  ExprBlock,
  ExprMatch,
  ExprArray,
  ExprAssign,
  ExprAssignOp,
  ExprAsync,
  ExprAwait,
  ExprBinary,
  ExprBox,
  ExprBreak,
  ExprContinue,
  ExprForLoop,
  ExprGroup,
  ExprIf,
  ExprIndex,
  ExprLet,
  ExprLoop,
  ExprParen,
  ExprRange,
  ExprReference,
  ExprRepeat,
  ExprReturn,
  ExprTryBlock,
  ExprTuple,
  ExprType,
  ExprUnary,
  ExprUnsafe,
  TokenStream,
  ExprWhile,
  ExprYield
);

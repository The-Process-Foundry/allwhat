//! The custom structures with parses needed to build the macros
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

use crate::LOG;

/// Tells a parent whether a descendent was unwrapped at any point and (un)modified expression
pub struct Unwrapped {
  pub had_try: bool,
  pub expr: TokenStream,
}

impl std::fmt::Debug for Unwrapped {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Unwrapped")
      .field("had_try", &self.had_try)
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
      .field("had_try", &self.had_try)
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
    print!(format!("{:#?}", self));
  }

  /// Printing the contents of the unwrapped to the log, dedent the logger, and wrap in Ok
  ///
  /// The primary purpose of this is to reduce noise in the code created by debugging statements
  pub fn log_ok(self) -> Result<Unwrapped> {
    print!("_-" => format!("Finalizing {}", self));
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

impl Questionable for ExprStruct {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let path = &self.path;
    print!(
      "_+" =>
      "Processing the struct expression of type '{}'",
      quote! {#path}
    );

    // Frontload quoting self, since it will get consumed while looking for tries
    let cloned_struct = quote! {#self};

    let mut unwrap = TokenStream::new();
    let mut fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();
    LOG.lock().unwrap().indent();

    for field in self.fields.into_iter() {
      // Since we cannot copy and unwrap_tries consumes, we want to make a copy first
      let cloned_field = quote! { #field };
      let name = field.get_ident();

      let Unwrapped { had_try, expr } = field.unwrap_tries()?;
      if had_try {
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
        print!("No try on the field {}", quote!(#name));
        fields.push(syn::parse2(cloned_field)?);
      }
    }

    print!("Finalizing Struct {}", quote! {#path});
    if unwrap.is_empty() {
      Unwrapped {
        had_try: false,
        expr: cloned_struct,
      }
    } else {
      let struct_ident = self.path;
      Unwrapped {
        had_try: true,
        expr: quote! {
          let mut __error_found = false;
          #unwrap

          match __error_found {
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
        print!("Processing the FieldValue named {}", ident.to_string());
        self.expr.unwrap_tries()?
      }
      Member::Unnamed(index) => {
        print!("Processing the unnamed FieldValue at index {}", index.index);
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

/// This is the searched for question mark operator
///
/// This is going to work recursively, so we're going to keep going down into the tree
/// even though the try operator should only appear on leaves. The recursion handles odd edge cases
/// such as:
///
/// let value = DemoStruct {
///   value1: value1?,
///   value_from_func: func(param1?, param2?)?,
/// })
impl Questionable for ExprTry {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    print! {"_+" => "Found a try: {}", quote!{#self}};

    let Unwrapped { had_try: _, expr } = self.expr.unwrap_tries()?;

    Unwrapped {
      had_try: true,
      expr: quote! {
        match { #expr } {
          Ok(val) => Ok(val),
          Err(err) => {
            __error_group.append(err);
            __error_found = true;
            Err(())
          },
        }
      },
    }
    .log_ok()
  }
}

/// All function calls, both functions and closures
impl Questionable for ExprCall {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let cloned_call = quote! {#self};
    print!("_+" =>  "Processing a call: {}", cloned_call);

    let mut unwrap = TokenStream::new();
    let mut args: Punctuated<Expr, Token![,]> = Punctuated::new();

    for (i, param) in self.args.into_iter().enumerate() {
      let param_clone = quote! { #param };
      let name = Ident::new(&format!("param_{}", i), Span::call_site());

      let Unwrapped { had_try, expr } = param.unwrap_tries()?;
      if had_try {
        print!("Processing Param {}", quote! {#name});
        unwrap.extend(quote! {
          let #name = { #expr };
        });

        args.push(syn::parse2(quote! { #name.unwrap() })?);
      } else {
        args.push(syn::parse2(param_clone)?);
      }
    }

    if unwrap.is_empty() {
      print! {"Returning the unchanged call"};
      Unwrapped {
        had_try: false,
        expr: cloned_call,
      }
    } else {
      let updated_call = ExprCall {
        args,
        ..{ syn::parse2(cloned_call)? }
      };
      print! {"Returning an updated call: {}", quote!{#updated_call}};

      Unwrapped {
        had_try: true,
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
}

/// Nothing to be done with a lit except return it
impl Questionable for ExprLit {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    print!("_+" =>  "Processing ExprLit: {}", quote! {self});
    Unwrapped {
      had_try: false,
      expr: quote! {#self},
    }
    .log_ok()
  }
}

/// Return as is, as we can never assume anything about the internals of a macro
impl Questionable for ExprMacro {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    print!("_+" => "Processing exprMacro: {}", quote!(#self));

    Unwrapped {
      had_try: false,
      expr: quote! {#self},
    }
    .log_ok()
  }
}

impl Questionable for ExprMethodCall {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    let cloned_call = quote! {#self};
    print!("_+" => "Processing Method Call: {}", cloned_call);

    let mut unwrap = TokenStream::new();
    // let mut assign = TokenStream::new();

    // for param in self.args.into_iter() {
    //   let name = param.get_ident();
    //   let Unwrapped { had_try, expr } = param.unwrap_tries()?;
    //   if had_try {
    //     print!("  - Processing Param {}", quote! {#name});
    //     unwrap.extend(quote! {
    //       let #name = { #expr };
    //     });

    //     let updated_expr: Expr = syn::parse2(quote! { #name.unwrap() })?;
    //     assign.extend(quote! {#updated_expr});
    //   }
    // }

    let mut args: Punctuated<Expr, Token![,]> = Punctuated::new();

    for (i, param) in self.args.into_iter().enumerate() {
      let param_clone = quote! { #param };
      let name = Ident::new(&format!("param_{}", i), Span::call_site());

      let Unwrapped { had_try, expr } = param.unwrap_tries()?;
      if had_try {
        print!("Processing Param {}", quote! {#name});
        unwrap.extend(quote! {
          let #name = { #expr };
        });

        args.push(syn::parse2(quote! { #name.unwrap() })?);
      } else {
        args.push(syn::parse2(param_clone)?);
      }
    }

    if unwrap.is_empty() {
      print! {"Returning the unchanged method call"};
      LOG.lock().unwrap().dedent();
      Unwrapped {
        had_try: false,
        expr: cloned_call,
      }
    } else {
      let updated_call = ExprCall {
        args,
        ..{ syn::parse2(cloned_call)? }
      };
      print! {"Returning an updated method call: {}", quote!{#updated_call}};
      LOG.lock().unwrap().dedent();

      Unwrapped {
        had_try: true,
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
}

impl Questionable for ExprPath {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    print!("_+" => "Processing ExprPath: {}", quote!(#self));

    Unwrapped {
      had_try: false,
      expr: quote! {#self},
    }
    .log_ok()
  }
}

//--- Unused/Unimplemented types of expression

impl Questionable for ExprArray {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprArray yet")
  }
}

impl Questionable for ExprAssign {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprAssign yet")
  }
}

impl Questionable for ExprAssignOp {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprAssignOp yet")
  }
}

impl Questionable for ExprAsync {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprAsync yet")
  }
}

impl Questionable for ExprAwait {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprAwait yet")
  }
}

impl Questionable for ExprBinary {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprBinary yet")
  }
}

impl Questionable for ExprBlock {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprBlock yet")
  }
}

impl Questionable for ExprBox {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprBox yet")
  }
}

impl Questionable for ExprBreak {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprBreak yet")
  }
}

impl Questionable for ExprCast {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprCast yet")
  }
}

impl Questionable for ExprClosure {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprClosure yet")
  }
}

impl Questionable for ExprContinue {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprContinue yet")
  }
}

impl Questionable for ExprField {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprField yet")
  }
}

impl Questionable for ExprForLoop {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprForLoop yet")
  }
}

impl Questionable for ExprGroup {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprGroup yet")
  }
}

impl Questionable for ExprIf {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprIf yet")
  }
}

impl Questionable for ExprIndex {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprIndex yet")
  }
}

impl Questionable for ExprLet {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprLet yet")
  }
}

impl Questionable for ExprLoop {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprLoop yet")
  }
}

impl Questionable for ExprMatch {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprMatch yet")
  }
}

impl Questionable for ExprParen {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprParen yet")
  }
}

impl Questionable for ExprRange {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprRange yet")
  }
}

impl Questionable for ExprReference {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprReference yet")
  }
}

impl Questionable for ExprRepeat {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprRepeat yet")
  }
}

impl Questionable for ExprReturn {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprReturn yet")
  }
}

impl Questionable for ExprTryBlock {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprTryBlock yet")
  }
}

impl Questionable for ExprTuple {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprTuple yet")
  }
}

impl Questionable for ExprType {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprType yet")
  }
}

impl Questionable for ExprUnary {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprUnary yet")
  }
}

impl Questionable for ExprUnsafe {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprUnsafe yet")
  }
}

impl Questionable for TokenStream {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for TokenStream yet")
  }
}

impl Questionable for ExprWhile {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprWhile yet")
  }
}

impl Questionable for ExprYield {
  fn unwrap_tries(self) -> Result<Unwrapped> {
    unimplemented!("Questionable not implemented for ExprYield yet")
  }
}

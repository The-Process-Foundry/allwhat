//! Map a set of result values into a struct initialization
//!
//! Create each field individually, gather all the errors, and if none of the errors are fatal
//! assign the fields directly to a struct. This is fairly useful when one has a diverse set of
//! independent fields to assign. The two big examples that come to mind is command line interfaces
//! and deserialization. Generally these are done with "fail-first" mentality, where only the first
//! problem found is reported, even though more than one field can fail validation simultaneously.
//! This allows us to return all the errors found instead of the painful fix one and test again
//! method of repairing the inputs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, FieldValue, Ident, Member, Result, Token};

/// Run the map_struct
pub fn run_macro(input: TokenStream) -> TokenStream {
  let input = match syn::parse2::<MapStruct>(input) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return TokenStream::from(err.to_compile_error()),
  };

  TokenStream::from(input.expand())
}

/********  The Structure's AST  *******/

/// The structure to be assigned
struct MapStruct {
  name: Ident,
  brace_token: token::Brace,
  fields: Punctuated<FieldValue, Token![,]>,
}

impl MapStruct {
  pub fn expand(self) -> TokenStream {
    for field in self.fields.iter() {
      match &field.member {
        Member::Named(ident) => println!("Field Named: {:#?}", ident),
        Member::Unnamed(i) => println!("Unnamed field index: {:#?}", i.index),
      }
    }

    quote! {
      Ok(#{self.name} {
        hello: "Not correct"
      });
    }
  }
}

impl Parse for MapStruct {
  fn parse(input: ParseStream) -> Result<Self> {
    let content;
    Ok(MapStruct {
      name: input.parse()?,
      brace_token: braced!(content in input),
      fields: content.parse_terminated(FieldValue::parse)?,
    })
  }
}

// /// The individual field assignment
// struct MapStructField {}

// impl Parse for MapStructField {
//   fn parse(input: ParseStream) -> Result<Self> {
//     Ok(Field {})
//   }
// }

// enum FieldAssignment {
//   Closure,
//   Function,
//   Result,
//   Direct,
// }

// impl Parse for FieldAssignment {
//   fn parse(input: ParseStream) -> Result<Self> {
//     unimplemented!("'parse' still needs to be implemented")
//   }
// }

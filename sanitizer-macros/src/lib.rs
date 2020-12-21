#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! Macros that allows seamless sanitizing
//! on struct fields
use crate::codegen::methods_layout;
use crate::sanitizer::parse_sanitizers;
use crate::type_ident::TypeOrNested;
use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::export::TokenStream2;
use syn::{parse_macro_input, DeriveInput};

// argument parsing and storing
mod arg;
// code gen here
mod codegen;
// parsing for struct fields
mod sanitizer;
// sanitizers are here
mod sanitizers;
// types and stuff
mod type_ident;

/// The Sanitize derive macro implements the Sanitize trait for you.
/// The trait only has a single associated function called `sanitize`
/// which edits the fields based on the sanitizer you specified in
/// the helper attributes
///
/// # Example
///
/// ```
/// use sanitizer::prelude::*;
///
/// #[derive(Sanitize)]
/// struct User {
///    #[sanitize(trim)]
///    name: String
/// }
///
/// fn main() {
/// 	let mut instance = User {
/// 		name: String::from("John, Doe ")
/// 	};
/// 	instance.sanitize();
/// 	assert_eq!(instance.name, "John, Doe");
/// }
/// ```
///
/// # Available sanitizers
///
/// - **trim**: Trims the string.
/// - **numeric**: Remove numeric items from the string.
/// - **alphanumeric**: Remove alphanumeric items from the string.
/// - **lower_case**: Convert input to lower case.
/// - **upper_case**: Convert input to upper case.
/// - **camel_case**: Convert input to camel case.
/// - **snake_case**: Convert input to snake case.
/// - **e164**: Convert a valid phone number to the e164 international standard, panic if invalid phone number.
/// - **clamp(min, max)**: Limit an integer input to this region of min to max.
/// - **clamp(max)**: Cut the string if it exceeds max.
/// - **screaming_snake_case**: Convert input to screaming snake case.
#[proc_macro_derive(Sanitize, attributes(sanitize))]
pub fn sanitize(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let name = input_parsed.ident;
    let mut inner_body: TokenStream2 = Default::default();
    let parsed = parse_sanitizers(input_parsed.data);
    if let Ok(ref val) = parsed {
        inner_body.append_all(val.iter().map(|r| {
            let field = r.0;
            let mut call = quote! {};
            let mut init = quote! {};
            let mut layout = quote! {};
            match field {
                TypeOrNested::Type(x, y) => {
                    layout = methods_layout(r.1, y.clone());
                    if !field.is_int() {
                        init.append_all(quote! {
                            let mut instance = StringSanitizer::from(self.#x.as_str());
                        })
                    } else {
                        init.append_all(quote! {
                            let mut instance = IntSanitizer::new(self.#x);
                        })
                    }
                    call.append_all(quote! {
                        self.#x = instance.get();
                    });
                }
                TypeOrNested::Nested(x, y) => call.append_all({
                    quote! {
                        <#y as Sanitize>::sanitize(&mut self.#x);
                    }
                }),
            }
            quote! {
                #init
                #layout
                #call
            }
        }));
    } else {
        let err = parsed.err().unwrap().to_string();
        inner_body = quote! { compile_error!(#err) };
    }
    let final_body = quote! {

        impl sanitizer::Sanitize for #name {
            fn sanitize(&mut self) {
                #inner_body
            }
        }
    };

    TokenStream::from(final_body)
}

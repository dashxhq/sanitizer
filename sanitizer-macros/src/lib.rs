#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! Macros that allows seamless sanitizing
//! on struct fields
use crate::codegen::enums::EnumGen;
use crate::codegen::sanitizers as sanitizer_gen;
use crate::codegen::structs::StructGen;
use crate::sanitizer::parse_sanitizers;
use crate::type_ident::TypeOrNested;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, TokenStreamExt};
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
///    name: String,
///    #[sanitize(custom(eight))]
///    acc_no: u8
/// }
///
/// fn eight(mut acc_no: u8) -> u8 {
///     if acc_no != 8 {
///         acc_no = 8;
///
///     }
///     acc_no
/// }
///
/// fn main() {
/// 	let mut instance = User {
/// 		name: String::from("John, Doe "),
///         acc_no: 10
/// 	};
/// 	instance.sanitize();
/// 	assert_eq!(instance.name, "John, Doe");
///     assert_eq!(instance.acc_no, 8);
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
/// - **custom(function)**: A custom function that is called to sanitize a field
/// according to any other way.
#[proc_macro_derive(Sanitize, attributes(sanitize))]
#[allow(unused_assignments)]
pub fn sanitize(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let name = input_parsed.ident;
    let mut inner_body: TokenStream2 = Default::default();
    let parsed = parse_sanitizers(input_parsed.data);
    if let Ok(ref val) = parsed {
        inner_body.append_all(val.get_map().iter().map(|r| {
            let field = r.0;
            let mut body = quote! {};
            match field {
                TypeOrNested::Type(field, type_ident) => {
                    let sanitizer_calls = sanitizer_gen::methods_layout(r.1, type_ident.clone());
                    let mut stream = Default::default();
                    if val.is_enum() {
                        stream = EnumGen::new(field.clone(), type_ident).body(sanitizer_calls);
                    } else {
                        stream = StructGen::new(field.clone(), type_ident).body(sanitizer_calls);
                    }
                    body.append_all(stream)
                }
                TypeOrNested::Nested(field, type_ident) => {
                    let call = quote! { <#type_ident as Sanitize>::sanitize };
                    if val.is_enum() {
                        body.append_all({
                            quote! {
                                if let Self::#field(x) = self {
                                    #call(x);
                                }
                            }
                        });
                    } else {
                        body.append_all({
                            quote! {
                                #call(&mut self.#field);
                            }
                        });
                    }
                }
            }
            quote! {
                #body
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

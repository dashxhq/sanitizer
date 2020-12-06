#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! Macros that allows seamless sanitizing
//! on struct fields
use crate::codegen::methods_layout;
use crate::sanitizer::parse_sanitizers;
use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::export::TokenStream2;
use syn::{parse_macro_input, DeriveInput};

mod codegen;
mod sanitizer;

/// The Sanitize derive macro implements the Sanitize trait for you.
/// The trait only has a single associated function called `sanitize`
/// which edits the fields based on the sanitizer you specified in
/// the helper attributes
///
/// # Example
///
/// ```
/// use sanitizer_macros::Sanitize;
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
/// - **Trim**: Trims the string.
/// - **numeric**: Remove numeric items from the string.
/// - **alphanumeric**: Remove alphanumeric items from the string.
/// - **lower_case**: Convert input to lower case.
/// - **upper_case**: Convert input to upper case.
/// - **camel_case**: Convert input to camel case.
/// - **snake_case**: Convert input to snake case.
/// - **screaming_snake_case**: Convert input to screaming snake case.
///
/// Right now, the macro only supports fields that have the type `String`
#[proc_macro_derive(Sanitize, attributes(sanitize))]
pub fn sanitize(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let name = input_parsed.ident;
    let mut inner_body: TokenStream2 = Default::default();
    let parsed = parse_sanitizers(input_parsed.data);
    if let Ok(ref val) = parsed {
        inner_body.append_all(val.iter().map(|r| {
            let field = r.0;
            let layout = methods_layout(r.1);
            quote! {
                let mut instance = Sanitizer::from(self.#field.as_str());
                #layout
                self.#field = instance.get();
            }
        }));
    } else {
        let err = parsed.err().unwrap().to_string();
        inner_body = quote! { compile_error!(#err) };
    }
    let final_body = quote! {
        use sanitizer::{Sanitize, Sanitizer};

        impl Sanitize for #name {
            fn sanitize(&mut self) {
                #inner_body
            }
        }
    };

    TokenStream::from(final_body)
}

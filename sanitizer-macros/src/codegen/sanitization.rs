use proc_macro2::TokenStream;
use quote::quote;

pub struct Sanitization {
    is_int: bool,
}

impl Sanitization {
    pub fn new(is_int: bool) -> Self {
        Self { is_int }
    }

    pub fn literal(&self) -> TokenStream {
        if self.is_int {
            quote! { 0 }
        } else {
            quote! { String::new() }
        }
    }

    pub fn method_calls(&self, field: TokenStream) -> TokenStream {
        if self.is_int {
            quote! {
                IntSanitizer::from(#field);
            }
        } else {
            quote! {
                StringSanitizer::from(#field);
            }
        }
    }

    pub fn field(&self, field: &TokenStream) -> TokenStream {
        if self.is_int {
            quote! {
                #field
            }
        } else {
            quote! {
                #field.as_str()
            }
        }
    }
}

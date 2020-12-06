use crate::sanitizer::{meta_list, SanitizerError};
use quote::{quote, TokenStreamExt};
use syn::export::TokenStream2;
use syn::{Ident, NestedMeta};

// helper function to get the sanitizer function body
pub fn sanitizer_function_body(sanitizer: &Ident) -> Result<TokenStream2, SanitizerError> {
    match sanitizer.to_string().as_str() {
        "trim" => Ok(quote! { trim() }),
        "numeric" => Ok(quote! { strip_numeric() }),
        "alphanumeric" => Ok(quote! { strip_alphanumeric() }),
        "lower_case" => Ok(quote! { to_lowercase() }),
        "upper_case" => Ok(quote! { to_uppercase() }),
        "camel_case" => Ok(quote! { to_camelcase() }),
        "snake_case" => Ok(quote! { to_snakecase() }),
        "screaming_snake_case" => Ok(quote! { to_screaming_snakecase() }),
        _ => Err(SanitizerError::new(5)),
    }
}

pub fn methods_layout(list: &Vec<NestedMeta>) -> TokenStream2 {
    let mut methods = quote! {};

    methods.append_all(list.iter().map(|e| {
        let err_msg = SanitizerError::new(5).to_string();

        if let Ok(meta) = meta_list(e) {
            if let Ok(body) = sanitizer_function_body(&meta) {
                quote! {
                    instance.#body;
                }
            } else {
                let meta_ident = format!("{}: {}", err_msg, meta.to_string());
                quote! {
                    compile_error!(#meta_ident);
                }
            }
        } else {
            quote! {
                compile_error!(#err_msg);
            }
        }
    }));
    methods
}

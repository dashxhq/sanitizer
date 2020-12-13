use crate::sanitizer::{check_if_valid_int, meta_list, PathOrList, SanitizerError};
use quote::{quote, TokenStreamExt};
use syn::export::{Span, TokenStream2};
use syn::{Ident, LitInt, NestedMeta};

#[macro_use]
macro_rules! sanitizer_with_arg {
    ( $sanitizer : expr, $body : expr ) => {
        if $sanitizer.has_args() {
            $body
        } else {
            Err(SanitizerError::new(7))
        }
    };
}

// helper function to get the sanitizer function body
pub fn sanitizer_function_body(
    sanitizer: &PathOrList,
    type_of_field: Ident,
) -> Result<TokenStream2, SanitizerError> {
    if check_if_valid_int(type_of_field.to_string()) {
        match sanitizer.to_string().as_str() {
            "clamp" => {
                sanitizer_with_arg!(sanitizer, {
                    if sanitizer.get_args().len() == 2 {
                        let arg_one = LitInt::new(&sanitizer.get_args().args[0], Span::call_site());
                        let arg_two = LitInt::new(&sanitizer.get_args().args[1], Span::call_site());
                        Ok(quote! {
                            clamp(#arg_one, #arg_two)
                        })
                    } else {
                        Err(SanitizerError::new(6))
                    }
                })
            }
            _ => Err(SanitizerError::new(5)),
        }
    } else {
        match sanitizer.to_string().as_str() {
            "trim" => Ok(quote! { trim() }),
            "numeric" => Ok(quote! { numeric() }),
            "alphanumeric" => Ok(quote! { alphanumeric() }),
            "lower_case" => Ok(quote! { to_lowercase() }),
            "upper_case" => Ok(quote! { to_uppercase() }),
            "camel_case" => Ok(quote! { to_camelcase() }),
            "snake_case" => Ok(quote! { to_snakecase() }),
            "screaming_snake_case" => Ok(quote! { to_screaming_snakecase() }),
            "e164" => Ok(quote! { e164() }),
            "clamp" => {
                sanitizer_with_arg!(sanitizer, {
                    if sanitizer.get_args().len() == 1 {
                        let arg_one = LitInt::new(&sanitizer.get_args().args[0], Span::call_site());
                        Ok(quote! {
                            cut(#arg_one)
                        })
                    } else {
                        Err(SanitizerError::new(6))
                    }
                })
            }
            _ => Err(SanitizerError::new(5)),
        }
    }
}

pub fn methods_layout(list: &Vec<NestedMeta>, type_of_field: Ident) -> TokenStream2 {
    let mut methods = quote! {};

    methods.append_all(list.iter().map(|e| {
        let err_msg = SanitizerError::new(5).to_string();

        if let Ok(meta) = meta_list(e) {
            let res_body = sanitizer_function_body(&meta, type_of_field.clone());
            if let Ok(body) = res_body {
                quote! {
                    instance.#body;
                }
            } else {
                let meta_ident = format!(
                    "{}: {}",
                    res_body.err().unwrap().to_string(),
                    meta.to_string()
                );
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

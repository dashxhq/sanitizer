use crate::arg::ArgBuilder;
use crate::codegen::PathOrList;
use crate::sanitizer::SanitizerError;
use quote::quote;
use syn::export::TokenStream2;

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

pub fn get_string_sanitizers(sanitizer: &PathOrList) -> Result<TokenStream2, SanitizerError> {
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
                    let arg_one = ArgBuilder::int(&sanitizer.get_args().args[0]);
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
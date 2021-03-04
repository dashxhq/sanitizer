use crate::arg::ArgBuilder;
use crate::codegen::PathOrList;
use crate::sanitizer::SanitizerError;
use proc_macro2::TokenStream;
use quote::quote;

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

pub fn get_string_sanitizers(sanitizer: &PathOrList) -> Result<TokenStream, SanitizerError> {
    match sanitizer.to_string().as_str() {
        "trim" => Ok(quote! { trim() }),
        "numeric" => Ok(quote! { numeric() }),
        "alphanumeric" => Ok(quote! { alphanumeric() }),
        "lower_case" => Ok(quote! { to_lowercase() }),
        "upper_case" => Ok(quote! { to_uppercase() }),
        "camel_case" => Ok(quote! { to_camel_case() }),
        "snake_case" => Ok(quote! { to_snake_case() }),
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
        "custom" => {
            sanitizer_with_arg!(sanitizer, {
                if sanitizer.get_args().len() == 1 {
                    let arg_one = ArgBuilder::ident(sanitizer.get_args().args[0].as_str());
                    Ok(quote! {
                        call(#arg_one)
                    })
                } else {
                    Err(SanitizerError::new(6))
                }
            })
        }
        _ => Err(SanitizerError::new(5)),
    }
}

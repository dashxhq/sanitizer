use crate::arg::ArgBuilder;
use crate::codegen::PathOrList;
use crate::sanitizer::SanitizerError;
use proc_macro2::TokenStream;
use quote::quote;

#[macro_use]
macro_rules! sanitizer_with_arg {
    ( $sanitizer : expr, $method_name : ident, $arg : expr  ) => {
        if $sanitizer.has_args() {
            if $sanitizer.get_args().len() == 1 {
                let arg_one = ArgBuilder::$method_name($arg);
                Ok(quote! {
                    cut(#arg_one)
                })
            } else {
                Err(SanitizerError::new(6))
            }
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
            sanitizer_with_arg!(sanitizer, int, &sanitizer.get_args().args[0])
        }
        "custom" => {
            sanitizer_with_arg!(sanitizer, ident, sanitizer.get_args().args[0].as_str())
        }
        _ => Err(SanitizerError::new(5)),
    }
}

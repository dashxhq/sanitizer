use crate::arg::ArgBuilder;
use crate::codegen::sanitizers::PathOrList;
use crate::sanitizer::SanitizerError;
use proc_macro2::TokenStream;
use quote::quote;

#[macro_use]
macro_rules! sanitizer_with_arg {
    ( $sanitizer : expr, $body : expr ) => {
        if $sanitizer.has_args() {
            $body
        } else {
            Err(SanitizerError::Only64BitInt)
        }
    };
}

pub fn get_int_sanitizers(sanitizer: &PathOrList) -> Result<TokenStream, SanitizerError> {
    match sanitizer.to_string().as_str() {
        "clamp" => {
            sanitizer_with_arg!(sanitizer, {
                if sanitizer.get_args().len() == 2 {
                    let arg_one = ArgBuilder::int(sanitizer.get_args().args[0].as_str());
                    let arg_two = ArgBuilder::int(sanitizer.get_args().args[1].as_str());
                    Ok(quote! {
                        clamp(#arg_one, #arg_two)
                    })
                } else {
                    Err(SanitizerError::WrongArguments)
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
                    Err(SanitizerError::WrongArguments)
                }
            })
        }
        _ => Err(SanitizerError::InvalidSanitizer),
    }
}

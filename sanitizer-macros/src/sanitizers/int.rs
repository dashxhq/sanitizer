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

pub fn get_int_sanitizers(sanitizer: &PathOrList) -> Result<TokenStream2, SanitizerError> {
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

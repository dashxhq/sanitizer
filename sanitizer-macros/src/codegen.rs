use crate::sanitizer::SanitizerError;
use crate::type_ident::TypeIdent;
use quote::{quote, TokenStreamExt};
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::export::{Span, TokenStream2};
use syn::{Ident, Lit, LitInt, Meta, NestedMeta};

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

pub enum PathOrList {
    Path(Ident),
    List(Ident, Args),
}

pub struct Args {
    pub args: Vec<String>,
}

// helper function to get the sanitizer function body
pub fn sanitizer_function_body(
    sanitizer: &PathOrList,
    type_of_field: TypeIdent,
) -> Result<TokenStream2, SanitizerError> {
    if type_of_field.is_int() {
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
    } else if type_of_field.is_string() {
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
    } else {
        Err(SanitizerError::new(0))
    }
}

pub fn methods_layout(list: &Vec<NestedMeta>, type_of_field: TypeIdent) -> TokenStream2 {
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

// helper function to get the list item as ident
pub fn meta_list(meta: &NestedMeta) -> Result<PathOrList, SanitizerError> {
    match meta {
        NestedMeta::Meta(x) => match x {
            Meta::Path(y) => {
                if let Some(x) = y.get_ident() {
                    Ok(PathOrList::Path(x.clone()))
                } else {
                    Err(SanitizerError::new(4))
                }
            }
            Meta::List(y) => {
                if let Some(x) = y.path.get_ident() {
                    let mut vec = Vec::new();
                    for args in y.nested.clone() {
                        if let Some(x) = get_int_arg(&args) {
                            vec.push(x);
                        } else {
                            return Err(SanitizerError::new(7));
                        }
                    }
                    return Ok(PathOrList::List(x.clone(), Args::new(vec)));
                } else {
                    Err(SanitizerError::new(4))
                }
            }
            _ => Err(SanitizerError::new(4)),
        },
        _ => Err(SanitizerError::new(4)),
    }
}

pub fn get_int_arg(meta: &NestedMeta) -> Option<String> {
    match meta {
        NestedMeta::Lit(x) => match x {
            Lit::Int(y) => Some(y.to_string()),
            _ => None,
        },
        _ => None,
    }
}

impl PathOrList {
    pub fn has_args(&self) -> bool {
        if let Self::List(_, _) = self {
            true
        } else {
            false
        }
    }
    pub fn get_args(&self) -> &Args {
        if let Self::List(_, x) = self {
            x
        } else {
            panic!("{:?}", "Arugment not found");
        }
    }
}

impl Args {
    pub fn len(&self) -> usize {
        self.args.len()
    }
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Display for PathOrList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let y = match self {
            Self::Path(x) => x.to_string(),
            Self::List(x, _) => x.to_string(),
        };
        write!(f, "{}", y)
    }
}

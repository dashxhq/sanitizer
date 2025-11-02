use crate::arg::Args;
use crate::sanitizer::SanitizerError;
use crate::sanitizers::*;
use crate::type_ident::TypeIdent;
use proc_macro2::{Ident, TokenStream};
use quote::{TokenStreamExt, quote};
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::{Lit, Meta, NestedMeta};

pub enum PathOrList {
    Path(Ident),
    List(Ident, Args),
}

// helper function to get the sanitizer function body
pub fn sanitizer_function_body(
    sanitizer: &PathOrList,
    type_of_field: TypeIdent,
) -> Result<TokenStream, SanitizerError> {
    if type_of_field.is_int() {
        int::get_int_sanitizers(sanitizer)
    } else if type_of_field.is_string() {
        string::get_string_sanitizers(sanitizer)
    } else {
        Err(SanitizerError::InvalidFieldType)
    }
}

pub fn methods_layout(list: &Vec<NestedMeta>, type_of_field: TypeIdent) -> TokenStream {
    let mut methods = quote! {};

    methods.append_all(list.iter().map(|meta| {
        let list = meta_list(meta);
        if let Ok(meta) = list {
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
            let err = list.err().unwrap().to_string();
            quote! {
                compile_error!(#err);
            }
        }
    }));
    methods
}

// helper function to get the list item as ident
pub fn meta_list(meta: &NestedMeta) -> Result<PathOrList, SanitizerError> {
    match meta {
        NestedMeta::Meta(meta) => match meta {
            Meta::Path(type_path) => {
                if let Some(type_path_ident) = type_path.get_ident() {
                    Ok(PathOrList::Path(type_path_ident.clone()))
                } else {
                    Err(SanitizerError::MacrosWithListOnly)
                }
            }
            Meta::List(list) => {
                if let Some(list_ident) = list.path.get_ident() {
                    let mut vec = Vec::new();
                    for args in list.nested.clone() {
                        if let Some(list_ident) = get_first_arg(&args) {
                            vec.push(list_ident);
                        } else {
                            return Err(SanitizerError::Only64BitInt);
                        }
                    }
                    return Ok(PathOrList::List(list_ident.clone(), Args::new(vec)));
                } else {
                    Err(SanitizerError::MacrosWithListOnly)
                }
            }
            _ => Err(SanitizerError::MacrosWithListOnly),
        },
        _ => Err(SanitizerError::MacrosWithListOnly),
    }
}

pub fn get_first_arg(meta: &NestedMeta) -> Option<String> {
    match meta {
        NestedMeta::Lit(literal) => match literal {
            Lit::Int(integer) => Some(integer.to_string()),
            _ => None,
        },
        NestedMeta::Meta(meta) => match meta {
            Meta::Path(path) => Some(path.segments.last().unwrap().ident.to_string()),
            _ => None,
        },
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
        if let Self::List(_, args) = self {
            args
        } else {
            panic!("{:?}", "Arugment not found");
        }
    }
}

impl Display for PathOrList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let string_path_list = match self {
            Self::Path(path) => path.to_string(),
            Self::List(path, _) => path.to_string(),
        };
        write!(f, "{}", string_path_list)
    }
}

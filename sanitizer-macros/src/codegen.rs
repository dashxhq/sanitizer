use crate::arg::Args;
use crate::sanitizer::SanitizerError;
use crate::sanitizers::*;
use crate::type_ident::TypeIdent;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
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
        Err(SanitizerError::new(0))
    }
}

pub fn methods_layout(list: &Vec<NestedMeta>, type_of_field: TypeIdent) -> TokenStream {
    let mut methods = quote! {};

    methods.append_all(list.iter().map(|e| {
        let list = meta_list(e);
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
                        if let Some(x) = get_first_arg(&args) {
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

pub fn init_struct(
    init: &mut TokenStream,
    type_ident: &TypeIdent,
    field: &Ident,
    call: &mut TokenStream,
) {
    if type_ident.is_option {
        if type_ident.is_int {
            init.append_all(quote! {
                let mut instance = IntSanitizer::from(0);
                if let Some(x) = self.#field {
                    instance = IntSanitizer::from(x);
                }
            })
        } else {
            init.append_all(quote! {
                let mut instance = StringSanitizer::from(String::new());
                if let Some(x) = &self.#field {
                    instance = StringSanitizer::from(x.as_str());
                }
            })
        }
        call.append_all(quote! {
            self.#field = Some(instance.get());
        });
    } else {
        if type_ident.is_int {
            init.append_all(quote! {
                let mut instance = IntSanitizer::from(self.#field);
            })
        } else {
            init.append_all(quote! {
                let mut instance = StringSanitizer::from(self.#field.as_str());
            })
        }
        call.append_all(quote! {
            self.#field = instance.get();
        });
    }
}

pub fn init_enum(
    init: &mut TokenStream,
    type_ident: &TypeIdent,
    field_name: &Ident,
    call: &mut TokenStream,
) {
    if type_ident.is_int {
        init.append_all(quote! {
            let mut instance = IntSanitizer::from(0);
            if let Self::#field_name(x) = self {
                instance = IntSanitizer::from(*x);
            }
        })
    } else {
        init.append_all(quote! {
            let mut instance = StringSanitizer::from(String::new());
            if let Self::#field_name(x) = self {
                instance = StringSanitizer::from(x.clone());
            }
        })
    }
    call.append_all(quote! {
        if let Self::#field_name(x) = self {
            *x = instance.get();
        };
    });
}

pub fn get_first_arg(meta: &NestedMeta) -> Option<String> {
    match meta {
        NestedMeta::Lit(x) => match x {
            Lit::Int(y) => Some(y.to_string()),
            _ => None,
        },
        NestedMeta::Meta(x) => match x {
            Meta::Path(y) => Some(y.segments.last().unwrap().ident.to_string()),
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
        if let Self::List(_, x) = self {
            x
        } else {
            panic!("{:?}", "Arugment not found");
        }
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

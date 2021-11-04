use crate::codegen::sanitization::Sanitization;
use crate::codegen::Entity;
use crate::type_ident::TypeIdent;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub struct StructGen {
    field_name: Entity,
    is_int: bool,
    is_option: bool,
    is_option_nested: bool,
}

impl StructGen {
    pub fn new(field_name: Entity, type_ident: &TypeIdent) -> Self {
        Self {
            field_name,
            is_int: type_ident.is_int,
            is_option: type_ident.is_option,
            is_option_nested: type_ident.is_nested,
        }
    }

    fn field_or_value(&self) -> TokenStream {
        let struct_instance = Sanitization::new(self.is_int);
        if self.is_option {
            let literal = struct_instance.literal();
            quote! {
                #literal
            }
        } else {
            let ident = &self.field_name;
            let field = struct_instance.field(&quote! { #ident });
            quote! {
                self.#field
            }
        }
    }

    fn operand(&self) -> TokenStream {
        let field = &self.field_name;
        if self.is_int {
            quote! {
                self.#field
            }
        } else {
            quote! {
                &self.#field
            }
        }
    }

    fn new_value(&self) -> TokenStream {
        let field_value = Sanitization::new(self.is_int);
        let val: TokenStream;
        if self.is_option_nested {
            val = field_value.field(&quote! { y });
        } else {
            val = field_value.field(&quote! { x });
        }
        field_value.method_calls(val)
    }

    fn call(&self) -> TokenStream {
        if self.is_option {
            if self.is_option_nested {
                quote! {
                    Some(Some(instance.get()));
                }
            } else {
                quote! {
                    Some(instance.get());
                }
            }
        } else {
            quote! {
                instance.get();
            }
        }
    }

    pub fn body(&self, sanitizers: TokenStream) -> TokenStream {
        let init = self.init();
        let operand = self.operand();
        let call = self.call();
        let mut rest = quote! {};
        let field = &self.field_name;
        if self.is_option {
            let new_value = self.new_value();
            if self.is_option_nested {
                rest.append_all(quote! {
                    if let Some(Some(x)) = #operand {
                        instance = #new_value
                        #sanitizers
                        self.#field = #call
                    };
                })
            } else {
                rest.append_all(quote! {
                    if let Some(x) = #operand {
                        instance = #new_value
                        #sanitizers
                        self.#field = #call
                    };
                })
            };
            quote! {
                let mut instance = #init
                #rest
            }
        } else {
            quote! {
                let mut instance = #init
                #sanitizers
                self.#field = #call
            }
        }
    }

    fn init(&self) -> TokenStream {
        let init = Sanitization::new(self.is_int);
        let value = self.field_or_value();
        init.method_calls(value)
    }
}

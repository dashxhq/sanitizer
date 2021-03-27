use crate::codegen::sanitization::Sanitization;
use crate::codegen::Entity;
use crate::type_ident::TypeIdent;
use proc_macro2::TokenStream;
use quote::quote;

pub struct EnumGen {
    field_name: Entity,
    is_int: bool,
}

impl EnumGen {
    pub fn new(field_name: Entity, type_ident: &TypeIdent) -> Self {
        Self {
            field_name,
            is_int: type_ident.is_int,
        }
    }

    pub fn body(&self, sanitizers: TokenStream) -> TokenStream {
        let field_name = &self.field_name;
        let operand = Sanitization::new(self.is_int);
        let literal = operand.literal();
        let field = operand.field(&literal);
        let call = operand.method_calls(field);
        let call_final = self.call();
        let body = if self.is_int {
            let re_assign = operand.method_calls(quote! {
                *x
            });
            quote! {
                let mut instance = #call;
                if let Self::#field_name(x) = self {
                    instance = #re_assign
                    #sanitizers
                    #call_final
                }
            }
        } else {
            let re_assign = operand.method_calls(quote! {
                x.clone()
            });
            quote! {
                let mut instance = #call
                if let Self::#field_name(x) = self {
                    instance = #re_assign
                    #sanitizers
                    #call_final
                }
            }
        };
        quote! {
            #body

        }
    }

    fn call(&self) -> TokenStream {
        quote! {
            *x = instance.get();
        }
    }
}

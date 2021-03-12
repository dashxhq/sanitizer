use proc_macro2::Span;
use syn::{Ident, LitInt};

pub struct Args {
    pub args: Vec<String>,
}
impl Args {
    pub fn len(&self) -> usize {
        self.args.len()
    }
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

pub struct ArgBuilder;

impl ArgBuilder {
    // hacky whitespace removal, should be fixed in future versions
    pub fn int(int: &str) -> LitInt {
        let mut owned = int.to_owned();
        owned.retain(|character| !character.is_whitespace());
        LitInt::new(&owned, Span::call_site())
    }
    pub fn ident(ident: &str) -> Ident {
        let mut owned = ident.to_owned();
        owned.retain(|character| !character.is_whitespace());
        Ident::new(&owned, Span::call_site())
    }
}

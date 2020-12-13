use syn::{export::Span, LitInt};

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
    pub fn int(int: &str) -> LitInt {
        let mut owned = int.to_owned();
        owned.retain(|c| !c.is_whitespace());
        LitInt::new(&owned, Span::call_site())
    }
}

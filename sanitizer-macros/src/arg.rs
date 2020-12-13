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
        LitInt::new(int, Span::call_site())
    }
}

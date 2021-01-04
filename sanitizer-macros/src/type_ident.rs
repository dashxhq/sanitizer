use crate::sanitizer::SanitizerError;
use std::convert::TryFrom;
use syn::{Ident, Type};

static INT_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i63", "isize", "usize",
];

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum TypeOrNested {
    // field, type
    Type(Ident, TypeIdent),
    Nested(Ident, Ident),
}

impl TypeOrNested {
    pub fn is_int(&self) -> bool {
        if let Self::Type(_, x) = self {
            x.is_int()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeIdent {
    pub ident: Ident,
    pub is_int: bool,
}

impl TypeIdent {
    pub fn new(ident: Ident, is_int: bool) -> Self {
        Self { ident, is_int }
    }
    pub fn is_string(&self) -> bool {
        self.ident == "String"
    }
    pub fn is_int(&self) -> bool {
        self.is_int
    }
    pub fn is_string_or_int(&self) -> bool {
        self.is_int || self.ident == "String"
    }
    pub fn ident(&self) -> Ident {
        self.ident.clone()
    }
}

impl TryFrom<Type> for TypeIdent {
    type Error = SanitizerError;
    fn try_from(type_ident: Type) -> Result<Self, Self::Error> {
        match type_ident {
            Type::Path(x) => {
                // the last entry is hopefully the type in a path
                // I think this is volatile and can change with future updates
                if let Some(y) = x.path.segments.last() {
                    let ident = y.clone().ident;
                    Ok(TypeIdent::new(
                        ident.clone(),
                        INT_TYPES.contains(&ident.to_string().as_str()),
                    ))
                } else {
                    Err(SanitizerError::new(0))
                }
            }
            _ => Err(SanitizerError::new(0)),
        }
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::{Data, Fields, FieldsNamed, Ident, Lit, Meta, NestedMeta, Type};

// SanitizerError is a custom error type that includes
// info on why proc macro parsing for Sanitizer crate failed
#[derive(Debug)]
pub struct SanitizerError(u8);

impl SanitizerError {
    pub fn new(code: u8) -> Self {
        Self(code)
    }
}

pub enum PathOrList {
    Path(Ident),
    List(Ident, usize),
}

impl PathOrList {
    pub fn has_args(&self) -> bool {
        if let Self::List(_, _) = self {
            true
        } else {
            false
        }
    }
    pub fn get_args(&self) -> usize {
        if let Self::List(_, x) = self {
            *x
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

impl Display for SanitizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let case = match self.0 {
            0 => "Invalid field type, only std::string::String is allowed",
            1 => "Struct cannot contain unnamed fields",
            2 => "Please specify at least a single sanitizer",
            3 => "Macro can be only applied on structs",
            4 => "Macros that contain a structured meta list are allowed only",
            5 => "Invalid sanitizer",
            6 => "This sanitizer takes a single argument",
            7 => "The argument can be only 64 bit int",
            _ => "",
        };
        write!(f, "{}", case)
    }
}

impl Error for SanitizerError {}

// the type of map where we store the fields with the lints
type FieldMap = HashMap<Ident, Vec<NestedMeta>>;

pub fn parse_sanitizers(data: Data) -> Result<FieldMap, SanitizerError> {
    let mut map: FieldMap = Default::default();
    match data {
        // macro is only for structs
        Data::Struct(structure) => {
            match structure.fields {
                // applied on named fields of the structs
                Fields::Named(named_fields) => populate_map(named_fields, &mut map),
                _ => Err(SanitizerError(1)),
            }
        }
        _ => Err(SanitizerError(3)),
    }
}
pub fn populate_map(
    named_fields: FieldsNamed,
    map: &mut FieldMap,
) -> Result<FieldMap, SanitizerError> {
    // iterate over each field
    for field in named_fields.named.iter() {
        let mut sanitizers = Vec::new();
        let field_type = field_type(field.clone().ty)?;
        // make sure the field type is string only
        if field_type == "String" {
            // get the attributes over the field
            for attr in field.attrs.iter() {
                // parse the attribute
                let meta = attr.parse_meta().unwrap();

                match meta {
                    // the attribute should be a list. for eg. sanitise(options)
                    Meta::List(ref list) => {
                        if let Some(x) = list.path.get_ident() {
                            if x == "sanitize" {
                                // get the sanitizers
                                sanitizers.extend(list.nested.iter().cloned())
                            }
                        }
                    }
                    _ => return Err(SanitizerError(4)),
                }
            }
            map.insert(field.clone().ident.unwrap(), sanitizers);
        } else {
            return Err(SanitizerError(0));
        }
    }
    Ok(map.clone())
}
// helper function to get the field type
pub fn field_type(field_type: Type) -> Result<Ident, SanitizerError> {
    match field_type {
        Type::Path(x) => {
            // the last entry is hopefully the type in a path
            // I think this is volatile and can change with future updates
            if let Some(y) = x.path.segments.last() {
                Ok(y.clone().ident)
            } else {
                Err(SanitizerError(0))
            }
        }
        _ => Err(SanitizerError(0)),
    }
}

// helper function to get the list item as ident
pub fn meta_list(meta: &NestedMeta) -> Result<PathOrList, SanitizerError> {
    match meta {
        NestedMeta::Meta(x) => match x {
            Meta::Path(y) => {
                if let Some(x) = y.get_ident() {
                    Ok(PathOrList::Path(x.clone()))
                } else {
                    Err(SanitizerError(4))
                }
            }
            Meta::List(y) => {
                if let Some(x) = y.path.get_ident() {
                    if y.nested.len() > 1 {
                        return Err(SanitizerError(6));
                    } else {
                        if let Some(arg) = y.nested.last() {
                            if let Some(y) = get_int_arg(arg) {
                                return Ok(PathOrList::List(x.clone(), y));
                            } else {
                                return Err(SanitizerError(7));
                            }
                        } else {
                            return Err(SanitizerError(6));
                        }
                    }
                } else {
                    Err(SanitizerError(4))
                }
            }
            _ => Err(SanitizerError(4)),
        },
        _ => Err(SanitizerError(4)),
    }
}

pub fn get_int_arg(meta: &NestedMeta) -> Option<usize> {
    match meta {
        NestedMeta::Lit(x) => match x {
            Lit::Int(y) => Some(y.base10_parse::<usize>().unwrap()),
            _ => None,
        },
        _ => None,
    }
}

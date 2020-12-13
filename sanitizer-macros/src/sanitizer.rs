use crate::type_ident::{TypeIdent, TypeOrNested};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::{Data, Fields, FieldsNamed, Meta, NestedMeta};

// SanitizerError is a custom error type that includes
// info on why proc macro parsing for Sanitizer crate failed
#[derive(Debug)]
pub struct SanitizerError {
    pub code: u8,
}

// the type of map where we store the fields with the lints
type FieldMap = BTreeMap<TypeOrNested, Vec<NestedMeta>>;

pub fn parse_sanitizers(data: Data) -> Result<FieldMap, SanitizerError> {
    let mut map: FieldMap = Default::default();
    match data {
        // macro is only for structs
        Data::Struct(structure) => {
            match structure.fields {
                // applied on named fields of the structs
                Fields::Named(named_fields) => populate_map(named_fields, &mut map),
                _ => Err(SanitizerError::new(1)),
            }
        }
        _ => Err(SanitizerError::new(3)),
    }
}
pub fn populate_map(
    named_fields: FieldsNamed,
    map: &mut FieldMap,
) -> Result<FieldMap, SanitizerError> {
    // iterate over each field
    for field in named_fields.named.iter() {
        let mut sanitizers = Vec::new();
        let field_type = TypeIdent::try_from(field.clone().ty)?;
        let mut type_field = TypeOrNested::Type(field.clone().ident.unwrap(), field_type.clone());
        // get the attributes over the field
        for attr in field.attrs.iter() {
            // parse the attribute
            let meta = attr.parse_meta().unwrap();
            match meta {
                // the attribute should be a list. for eg. sanitise(options)
                Meta::List(ref list) => {
                    // make sure the field type is string only
                    if field_type.is_string() || field_type.is_int() {
                        if let Some(x) = list.path.get_ident() {
                            if x == "sanitize" {
                                // get the sanitizers
                                sanitizers.extend(list.nested.iter().cloned())
                            }
                        }
                    } else {
                        return Err(SanitizerError::new(0));
                    }
                }
                Meta::Path(_) => {
                    if field_type.is_string() || field_type.is_int() {
                        return Err(SanitizerError::new(2));
                    } else {
                        type_field =
                            TypeOrNested::Nested(field.clone().ident.unwrap(), field_type.ident())
                    }
                }
                _ => return Err(SanitizerError::new(4)),
            }
        }
        map.insert(type_field, sanitizers);
    }
    Ok(map.clone())
}

impl SanitizerError {
    pub fn new(code: u8) -> Self {
        Self { code }
    }
}

impl Display for SanitizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let case = match self.code {
            0 => "Invalid field type",
            1 => "Struct cannot contain unnamed fields",
            2 => "Please specify at least a single sanitizer",
            3 => "Macro can be only applied on structs",
            4 => "Macros that contain a structured meta list are allowed only",
            5 => "Invalid sanitizer",
            6 => "Wrong number of arguments",
            7 => "The argument can be only 64 bit int",
            _ => "",
        };
        write!(f, "{}", case)
    }
}

impl Error for SanitizerError {}

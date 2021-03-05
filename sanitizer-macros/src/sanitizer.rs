use crate::type_ident::{TypeIdent, TypeOrNested};
use proc_macro2::Span;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::Ident;
use syn::{Data, DataEnum, Fields, FieldsNamed, Meta, NestedMeta};

// SanitizerError is a custom error type that includes
// info on why proc macro parsing for Sanitizer crate failed
#[derive(Debug)]
pub struct SanitizerError {
    pub code: u8,
}

// the type of map where we store the fields with the lints
type FieldMap = BTreeMap<TypeOrNested, Vec<NestedMeta>>;

pub enum StructOrEnum {
    Enum(FieldMap),
    Struct(FieldMap),
}

pub fn parse_sanitizers(data: Data) -> Result<StructOrEnum, SanitizerError> {
    let mut map: FieldMap = Default::default();
    match data {
        Data::Struct(structure) => {
            match structure.fields {
                // applied on named fields of the structs
                Fields::Named(named_fields) => populate_map_struct(named_fields, &mut map),
                _ => Err(SanitizerError::new(1)),
            }
        }
        Data::Enum(x) => populate_map_enum(x, &mut map),
        _ => Err(SanitizerError::new(3)),
    }
}

pub fn populate_map_struct(
    named_fields: FieldsNamed,
    map: &mut FieldMap,
) -> Result<StructOrEnum, SanitizerError> {
    // iterate over each field
    for field in named_fields.named.iter() {
        let mut push = false;
        let mut sanitizers = Vec::new();
        let field_type = TypeIdent::try_from(field.clone().ty)?;
        let mut type_field = TypeOrNested::Type(field.clone().ident.unwrap(), field_type.clone());
        // get the attributes over the field
        for attr in field.attrs.iter() {
            // parse the attribute
            if attr.path.is_ident("sanitize") {
                push = true;
                let meta = attr.parse_meta().unwrap();
                match meta {
                    // the attribute should be a list. for eg. sanitise(options)
                    Meta::List(ref list) => {
                        // make sure the field type is string only
                        if field_type.is_string_or_int() {
                            // get the sanitizers
                            sanitizers.extend(list.nested.iter().cloned())
                        } else {
                            return Err(SanitizerError::new(0));
                        }
                    }
                    Meta::Path(_) => {
                        if field_type.is_string_or_int() {
                            return Err(SanitizerError::new(2));
                        } else {
                            type_field = TypeOrNested::Nested(
                                field.clone().ident.unwrap(),
                                field_type.ident(),
                            )
                        }
                    }
                    _ => return Err(SanitizerError::new(4)),
                }
            }
        }
        if push {
            map.insert(type_field, sanitizers);
        }
    }
    Ok(StructOrEnum::Struct(map.clone()))
}

pub fn populate_map_enum(
    named_fields: DataEnum,
    map: &mut FieldMap,
) -> Result<StructOrEnum, SanitizerError> {
    // iterate over each field
    for variant in named_fields.variants.iter() {
        let mut sanitizers = Vec::new();
        let mut field_type = TypeIdent::new(Ident::new("_", Span::call_site()), false);
        let mut type_field = TypeOrNested::Type(variant.clone().ident, field_type);
        let mut push = false;
        for attr in variant.attrs.iter() {
            let meta = attr.parse_meta().unwrap();
            if attr.path.is_ident("sanitize") {
                push = true;
                match meta {
                    // the attribute should be a list. for eg. sanitise(options)
                    Meta::List(ref list) => {
                        match &variant.fields {
                            Fields::Unnamed(x) => {
                                for unnamed in x.unnamed.iter() {
                                    field_type = TypeIdent::try_from(unnamed.ty.clone())?;
                                    type_field.set_type(field_type.clone());
                                    if field_type.is_string_or_int() {
                                        // get the sanitizers
                                        sanitizers.extend(list.nested.iter().cloned())
                                    } else {
                                        return Err(SanitizerError::new(0));
                                    }
                                }
                            }
                            _ => return Err(SanitizerError::new(8)),
                        }
                    }
                    Meta::Path(_) => match &variant.fields {
                        Fields::Unnamed(x) => {
                            for unnamed in x.unnamed.iter() {
                                field_type = TypeIdent::try_from(unnamed.ty.clone())?;
                                type_field.set_type(field_type.clone());
                                if field_type.is_string_or_int() {
                                    return Err(SanitizerError::new(2));
                                } else {
                                    type_field = TypeOrNested::Nested(
                                        variant.clone().ident,
                                        field_type.ident(),
                                    )
                                }
                            }
                        }
                        _ => return Err(SanitizerError::new(8)),
                    },
                    _ => return Err(SanitizerError::new(4)),
                }
            }
        }
        if push {
            map.insert(type_field, sanitizers);
        }
    }
    Ok(StructOrEnum::Enum(map.clone()))
}

impl StructOrEnum {
    pub fn is_enum(&self) -> bool {
        if let Self::Enum(_) = self {
            true
        } else {
            false
        }
    }
    pub fn get_map(&self) -> &FieldMap {
        match self {
            Self::Enum(x) => x,
            Self::Struct(x) => x,
        }
    }
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
            8 => "Enums can contain only unnamed field",
            _ => "",
        };
        write!(f, "{}", case)
    }
}

impl Error for SanitizerError {}

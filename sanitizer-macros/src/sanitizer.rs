use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::{Data, Fields, FieldsNamed, Ident, Lit, Meta, NestedMeta, Type};

static INT_TYPES: [&str; 10] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i63", "isize", "usize",
];
// SanitizerError is a custom error type that includes
// info on why proc macro parsing for Sanitizer crate failed
#[derive(Debug)]
pub struct SanitizerError(u8);

pub enum PathOrList {
    Path(Ident),
    List(Ident, Args),
}

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

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum TypeOrNested {
    // field, type
    Type(Ident, Ident),
    Nested(Ident, Ident),
}

impl TypeOrNested {
    pub fn is_int(&self) -> bool {
        if let Self::Type(_, x) = self {
            check_if_valid_int(x.to_string())
        } else {
            false
        }
    }
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
        let mut type_field = TypeOrNested::Type(field.clone().ident.unwrap(), field_type.clone());
        // get the attributes over the field
        for attr in field.attrs.iter() {
            // parse the attribute
            let meta = attr.parse_meta().unwrap();
            let int = check_if_valid_int(field_type.to_string());
            match meta {
                // the attribute should be a list. for eg. sanitise(options)
                Meta::List(ref list) => {
                    // make sure the field type is string only
                    if field_type == "String" || int {
                        if let Some(x) = list.path.get_ident() {
                            if x == "sanitize" {
                                // get the sanitizers
                                sanitizers.extend(list.nested.iter().cloned())
                            }
                        }
                    } else {
                        return Err(SanitizerError(0));
                    }
                }
                Meta::Path(_) => {
                    if field_type == "String" || int {
                        return Err(SanitizerError(2));
                    } else {
                        type_field =
                            TypeOrNested::Nested(field.clone().ident.unwrap(), field_type.clone())
                    }
                }
                _ => return Err(SanitizerError(4)),
            }
        }
        map.insert(type_field, sanitizers);
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
                    let mut vec = Vec::new();
                    for args in y.nested.clone() {
                        if let Some(x) = get_int_arg(&args) {
                            vec.push(x);
                        } else {
                            return Err(SanitizerError(7));
                        }
                    }
                    return Ok(PathOrList::List(x.clone(), Args::new(vec)));
                } else {
                    Err(SanitizerError(4))
                }
            }
            _ => Err(SanitizerError(4)),
        },
        _ => Err(SanitizerError(4)),
    }
}

pub fn get_int_arg(meta: &NestedMeta) -> Option<String> {
    match meta {
        NestedMeta::Lit(x) => match x {
            Lit::Int(y) => Some(y.to_string()),
            _ => None,
        },
        _ => None,
    }
}

pub fn check_if_valid_int(int_type: String) -> bool {
    INT_TYPES.contains(&int_type.as_str())
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

impl SanitizerError {
    pub fn new(code: u8) -> Self {
        Self(code)
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
            6 => "Wrong number of arguments",
            7 => "The argument can be only 64 bit int",
            _ => "",
        };
        write!(f, "{}", case)
    }
}

impl Error for SanitizerError {}

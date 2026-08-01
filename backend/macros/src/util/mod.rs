//! Utility methods for Butterfly procedural macros.

use core::panic;
use std::{format, matches};

use quote::ToTokens;
use syn::{Attribute, GenericArgument, PathArguments, Type};

/// Is true if `ty` is an Option. In that case, SQL`NULL` will map to Rust [None].
///
/// # Example
///
/// ```ignore
/// use butterfly_macro::util::sql_type_nullale;
/// use syn::Type;
///
/// let option_str: Type = syn::parse_str("::std::option::Option<String>").unwrap();
/// let string: Type = syn::parse_str("String").unwrap();
///
/// assert_eq!(sql_type_nullale(option_str), true)
/// assert_eq!(sql_type_nullale(string), false)
/// ```
pub fn sql_type_nullale(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            // unwrap is safe here: every type path has at least one element
            let segment = type_path.path.segments.last().unwrap();
            // match segment.ident.to_string().as_str() {
            //     "Option" => true,
            //     _ => false,
            // }
            matches!(segment.ident.to_string().as_str(), "Option")
        }
        _ => false,
    }
}

/// Returns the SQL type name for a given Rust type.
///
/// # Example
///
/// ```ignore
/// use butterfly_macro::util::sql_type;
/// use syn::Type;
///
/// let option_str: Type = syn::parse_str("::std::option::Option<String>").unwrap();
/// let string: Type = syn::parse_str("String").unwrap();
/// let integer: Type = syn::parse_str("i8").unwrap();
///
/// assert_eq!(sql_type(option_str).as_str(), "TEXT")
/// assert_eq!(sql_type(string).as_str(), "TEXT")
/// assert_eq!(sql_type(integer).as_str(), "INTEGER")
/// ```
pub fn sql_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            // unwrap is safe here: every type path has at least one element
            let segment = type_path.path.segments.last().unwrap();
            match segment.ident.to_string().as_str() {
                // Numeric
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                    "INTEGER".to_string()
                }
                "f32" | "f64" => "REAL".to_string(),

                // String-like
                "String" | "str" | "Uuid" => "TEXT".to_string(),

                // Boleans
                "bool" => "BOOLEAN".to_string(),

                // Other
                "Option" if !segment.arguments.is_empty() => {
                    let generic = match &segment.arguments {
                        PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                            match &args.args[0] {
                                GenericArgument::Type(inner_ty) => inner_ty,
                                _ => panic!(
                                    "Unsupported generic argument in Option: {:?}",
                                    args.args[0]
                                ),
                            }
                        }
                        _ => panic!("Unsupported arguments in Option: {:?}", segment.arguments),
                    };

                    sql_type(generic)
                }
                "Vec" if segment.arguments.is_empty() => "BLOB".to_string(),

                // Unknown
                _ => "BLOB".to_string(),
            }
        }
        Type::Array(_) => "BLOB".to_string(),
        Type::Group(type_group) => sql_type(&type_group.elem),
        Type::Paren(type_paren) => sql_type(&type_paren.elem),
        Type::Reference(type_reference) => sql_type(&type_reference.elem),
        Type::Slice(_) => "BLOB".to_string(),
        Type::Tuple(_) => panic!(
            "Tuples in a #[collection] are reserved for future use and currently not supported."
        ),
        ty => {
            panic!(
                "#[collection] macro could not determine SQL for the following type: {}",
                ty.to_token_stream()
            )
        }
    }
}

/// Returns the full SQL type name for a given Rust type, including wether it is
/// optional.
///
/// # Example
///
/// ```ignore
/// use butterfly_macro::util::sql_type;
/// use syn::Type;
///
/// let option_str: Type = syn::parse_str("::std::option::Option<String>").unwrap();
/// let string: Type = syn::parse_str("String").unwrap();
/// let integer: Type = syn::parse_str("i8").unwrap();
///
/// assert_eq!(sql_type(option_str).as_str(), "TEXT")
/// assert_eq!(sql_type(string).as_str(), "TEXT NOT NULL")
/// assert_eq!(sql_type(integer).as_str(), "INTEGER NOT NULL")
/// ```
pub fn _full_sql_type(ty: &Type) -> String {
    let nullable = sql_type_nullale(ty);
    let type_name = sql_type(ty);

    if nullable {
        type_name
    } else {
        format!("{type_name} NOT NULL")
    }
}

/// For a given attribute vector, consumes a specific attribute and
/// return it if it exists, removing it from the vector.
pub fn consume_attrs<K: AsRef<str>>(name: K, args: &mut Vec<Attribute>) -> Vec<Attribute> {
    let result = args
        .iter()
        .filter(|attr: &&Attribute| attr.meta.path().is_ident(name.as_ref()))
        .cloned()
        .collect::<Vec<_>>();
    args.retain(|attr: &Attribute| !attr.meta.path().is_ident(name.as_ref()));
    result
}

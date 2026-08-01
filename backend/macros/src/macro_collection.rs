//! The [collection] attribute marks a struct as an SQL table mapping.

use crate::util;
use proc_macro2::TokenStream;
use quote::quote;
use std::format;
use syn::{ItemStruct, LitStr};

/// Parses the key value pairs in the [collection][super::collection] macro.
pub struct CollectionMacroMeta {
    /// `#[collection(table = "hello")]`
    table: LitStr,
}

impl ::syn::parse::Parse for CollectionMacroMeta {
    fn parse(attr: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
        // Parse the attr as a hashmap of literal keys and tokens in the first stage,
        // parse proper types in the second stage.
        // E.g.: db = ..., table = ...
        let mut table = None;

        while !attr.is_empty() {
            let key: syn::Ident = attr.parse()?;
            attr.parse::<::syn::Token![=]>()?;

            match key.to_string().as_str() {
                "table" => {
                    if table.is_some() {
                        return Err(::syn::Error::new_spanned(
                            key,
                            "duplicate field `table` in #[collection(...)]".to_string(),
                        ));
                    }

                    table = Some(attr.parse::<LitStr>()?);
                }
                _ => {
                    return Err(::syn::Error::new_spanned(
                        key.clone(),
                        format!(
                            "unknown key in #[collection(...)]: `{key}`. expected any of: `table`"
                        ),
                    ));
                }
            }

            if attr.is_empty() {
                break;
            }

            attr.parse::<::syn::Token![,]>()?;
        }

        let table = table.ok_or_else(|| {
            ::syn::Error::new(
                ::proc_macro2::Span::call_site(),
                format!("missing `{}` in #[collection(...)]", stringify!($field)),
            )
        })?;

        Ok(Self { table })
    }
}

/// The [collection] attribute marks a struct as an SQL table mapping. The
/// struct will be extended with database access methods.
pub(crate) fn collection(meta: CollectionMacroMeta, mut item: ItemStruct) -> TokenStream {
    // The name of the collections' rust struct.
    let struct_name = &item.ident;
    let table_name = meta.table;

    // Retrieves the item attributes. This is done so macro generated doc comments
    // can be appended instead of preceeding other arguments.
    let strucc_attr = item.attrs;
    item.attrs = Vec::new();

    // For all fields, consumes the `#[primary]` attribute and return identifiers
    // for primary fields.
    let primary_fields = item
        .fields
        .iter_mut()
        .filter_map(|field| {
            let is_primary = !util::consume_attrs("primary", &mut field.attrs).is_empty();
            match is_primary {
                true => field.ident.clone(),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    // The schema doc comment.
    //
    // This generates and appends schema documentation onto a collection-marked
    // struct.
    let schema_doc_comment = {
        let mut header = quote! {
            /// # Schema
            ///
            /// | CID | Name | Type | Required | PK |
            /// | --- | ---- | ---- | :------: | -- |
        };

        for (cid, field) in item.fields.iter().enumerate() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let sql_field_type = util::sql_type(&field.ty);
            let is_optional = util::sql_type_nullale(&field.ty);
            let is_primary = primary_fields
                .iter()
                .any(|i1| field.ident.as_ref().is_some_and(|i2| i1 == i2));

            let required_text = if is_optional { "FALSE" } else { "TRUE" };
            let primary_text = if is_primary { "PRIMARY" } else { "" };
            let comment = format!(
                " | {cid} | `{field_name}` | {sql_field_type} | {required_text} | {primary_text} |"
            );
            header.extend(quote! { #[doc = #comment] });
        }

        header
    };

    quote! {
        #(#strucc_attr)*
        #schema_doc_comment
        #item

        impl #struct_name {
            /// The name of the collections' table in the database. This is used
            /// for generating SQL queries and must be unique within the database.
            ///
            /// # SQL Example
            ///
            /// ```sql
            #[doc = concat!("CREATE TABLE ", #table_name, " (...);")]
            #[doc = concat!("SELECT * FROM ", #table_name, ";")]
            /// ```
            pub const TABLE_NAME: &'static str = #table_name;

            /// Saves the collection
            pub async fn save(&self, pool: &::sqlx::Pool<::sqlx::Sqlite>) -> Result<(), ::sqlx::Error> {
                // sqlx::query!("INSERT INTO tables (id, username) VALUES (?, ?)", self.id, self.username).execute(pool).await
                todo!("collection save method");

                #[allow(unreachable_code)] // there were rust-analyzer problems with todo! in macro
                Ok(())
            }
        }
    }
}

//! The [collection] attribute marks a struct as an SQL table mapping.

use std::format;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::util;

/// The [collection] attribute marks a struct as an SQL table mapping. The
/// struct will be extended with database access methods.
pub(crate) fn collection(_meta: TokenStream, mut item: ItemStruct) -> TokenStream {
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
            let is_required = util::sql_type_nullale(&field.ty);
            let is_primary = primary_fields
                .iter()
                .any(|i1| field.ident.as_ref().is_some_and(|i2| i1 == i2));

            let required_text = if is_required { "" } else { "TRUE" };
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
    }
}

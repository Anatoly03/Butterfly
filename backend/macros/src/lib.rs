//! The macro utility of the Butterfly backend application.

mod macro_collection;
mod util;

use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

/// The [collection] attribute marks a struct as an SQL table mapping. The
/// struct will be extended with database access methods.
#[proc_macro_attribute]
pub fn collection(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    macro_collection::collection(args.into(), item).into()
}

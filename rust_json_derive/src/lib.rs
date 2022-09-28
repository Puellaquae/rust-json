use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod from_json;
mod to_json;
mod attrs;

#[proc_macro_derive(ToJson, attributes(rename))]
pub fn derive_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_json::expand_serialize(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(FromJson, attributes(rename))]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    from_json::expand_deserialize(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

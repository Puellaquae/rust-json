use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod to_json;
mod from_json;

#[proc_macro_derive(ToJson)]
pub fn derive_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    match input.data {
        Data::Struct(struct_data) => to_json::serialize_struct(ident, struct_data),
        Data::Enum(enum_data) => to_json::serialize_enum(ident, enum_data),
        Data::Union(_) => panic!("rust_json_derive not support union!")
    }
}

#[proc_macro_derive(FromJson)]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    match input.data {
        Data::Struct(struct_data) => from_json::deserialize_struct(ident, struct_data),
        Data::Enum(enum_data) => from_json::deserialize_enum(ident, enum_data),
        Data::Union(_) => panic!("rust_json_derive not support union!")
    }
}

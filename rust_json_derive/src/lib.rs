use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod to_json;

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
        Data::Struct(struct_data) => {
            let fields = struct_data.fields;
            match fields {
                Fields::Named(_) => {
                    unimplemented!();
                }
                Fields::Unnamed(_) => {
                    unimplemented!();
                }
                Fields::Unit => quote!(
                    impl rust_json::FromJson for #ident {
                        fn from_json(json: rust_json::JsonElem) -> Option<#ident> {
                            if rust_json::JsonElem::Str(stringify!(#ident).to_string()) == json {
                                Some(#ident)
                            } else {
                                None
                            }
                        }
                    }
                )
                .into(),
            }
        }
        Data::Enum(_enum_data) => {
            unimplemented!();
        }
        Data::Union(_) => {
            panic!("rust_json_derive not support union!");
        }
    }
}

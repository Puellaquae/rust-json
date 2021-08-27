use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(ToJson)]
pub fn derive_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    match input.data {
        Data::Struct(struct_data) => {
            let fields = struct_data.fields;
            match fields {
                Fields::Named(_) => {
                    let s: Vec<_> = fields
                        .iter()
                        .map(|f| f.ident.as_ref())
                        .map(
                            |i| quote!(hm.insert(stringify!(#i).to_string(), self.#i.to_json());),
                        )
                        .collect();
                    quote!(
                        impl ToJson for #ident {
                            fn to_json(&self) -> rust_json::JsonElem {
                                let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                                #(#s)*
                                rust_json::JsonElem::Object(hm)
                            }
                        }
                    ).into()
                }
                Fields::Unnamed(_) => {
                    let s: Vec<_> = fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| Index::from(i))
                        .map(|i| quote!(vec.push(self.#i.to_json());))
                        .collect();
                    quote!(
                        impl ToJson for #ident {
                            fn to_json(&self) -> rust_json::JsonElem {
                                let mut vec = Vec::<rust_json::JsonElem>::new();
                                #(#s)*
                                rust_json::JsonElem::Array(vec)
                            }
                        }
                    )
                    .into()
                }
                Fields::Unit => {
                    quote!(
                        impl ToJson for #ident {
                            fn to_json(&self) -> rust_json::JsonElem {
                                rust_json::JsonElem::Str(stringify!(#ident).to_string())
                            }
                        }
                    )
                    .into()
                }
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

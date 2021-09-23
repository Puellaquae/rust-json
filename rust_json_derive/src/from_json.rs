use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, Fields, Ident};

pub fn deserialize_struct(ident: Ident, struct_data: DataStruct) -> TokenStream {
    let s = deserialize_fields(&ident, struct_data.fields);
    quote!(
        impl rust_json::FromJson for #ident {
            fn from_json(json: rust_json::JsonElem) -> Option<#ident> {
                #s
            }
        }
    )
    .into()
}

pub fn deserialize_enum(_ident: Ident, _enum_data: DataEnum) -> TokenStream {
    quote!().into()
}

fn deserialize_fields(ident: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(_) => {
            let s = fields
                .iter()
                .map(|f| f.ident.as_ref())
                .map(|i| quote!(#i: obj.get(&stringify!(#i).to_string()).unwrap().get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Object(obj) = json {
                    Some(#ident{#(#s,)*})
                } else {
                    None
                }
            )
        }
        Fields::Unnamed(_) => {
            let s = (0..fields.len()).map(|i| quote!(vec[#i].get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Array(vec) = json {
                    Some(#ident(#(#s,)*))
                } else {
                    None
                }
            )
        }
        Fields::Unit => {
            quote!(
                if rust_json::JsonElem::Str(stringify!(#ident).to_string()) == json {
                    Some(#ident)
                } else {
                    None
                }
            )
        }
    }
}

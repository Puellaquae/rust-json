use proc_macro::TokenStream;

use quote::quote;
use syn::{DataEnum, DataStruct, Fields, Ident, Index, Variant};
use proc_macro2::Span;

pub fn serialize_struct(ident: Ident, struct_data: DataStruct) -> TokenStream {
    let s = serialize_fields(&ident, struct_data.fields);
    quote!(
        impl rust_json::ToJson for #ident {
            fn to_json(&self) -> rust_json::JsonElem {
                #s
            }
        }
    )
    .into()
}

pub fn serialize_enum(ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants;
    let s = variants
        .iter()
        .map(|v| serialize_varient(&ident, v));
    quote!(
        impl rust_json::ToJson for #ident {
            fn to_json(&self) -> rust_json::JsonElem {
                match self {
                    #(#s),*
                }
            }
        }
    )
    .into()
}

fn serialize_fields(ident: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(_) => {
            let s: Vec<_> = fields
                .iter()
                .map(|f| f.ident.as_ref())
                .map(|i| quote!(hm.insert(stringify!(#i).to_string(), self.#i.to_json());))
                .collect();
            quote!(
                {
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    #(#s)*
                    rust_json::JsonElem::Object(hm)
                }
            )
        }
        Fields::Unnamed(_) => {
            let s: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(i, _)| Index::from(i))
                .map(|i| quote!(vec.push(self.#i.to_json());))
                .collect();
            quote!(
                {
                    let mut vec = Vec::<rust_json::JsonElem>::new();
                    #(#s)*
                    rust_json::JsonElem::Array(vec)
                }
            )
        }
        Fields::Unit => {
            quote!({ rust_json::JsonElem::Str(stringify!(#ident).to_string()) })
        }
    }
}

fn serialize_varient(ident: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let fields = &variant.fields;
    let var_ident = &variant.ident;
    match fields {
        Fields::Unit => {
            quote!(#ident::#var_ident => rust_json::JsonElem::Str(stringify!(#var_ident).to_string()))
        }
        Fields::Unnamed(_) if fields.len() == 1 => {
            //let field = Ident::new("__field", Span::call_site());
            quote!(#ident::#var_ident(field) => {
                let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                hm.insert(stringify!(#var_ident).to_string(), field.to_json());
                rust_json::JsonElem::Object(hm)
            })
        }
        Fields::Unnamed(_) => {
            let field_names =
                (0..fields.len()).map(|i| Ident::new(&format!("field{}", i), Span::call_site()));
            let s = (0..fields.len())
                .map(|i| Ident::new(&format!("field{}", i), Span::call_site()))
                .map(|i| quote!(vec.push(#i.to_json());));
            quote! {
                #ident::#var_ident(#(#field_names),*) => {
                    let mut vec = Vec::<rust_json::JsonElem>::new();
                    #(#s)*
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    hm.insert(stringify!(#var_ident).to_string(), rust_json::JsonElem::Array(vec));
                    rust_json::JsonElem::Object(hm)
                }
            }
        }
        Fields::Named(_) => {
            let field_names = fields.iter().map(|f| f.ident.as_ref());
            let s = fields
                .iter()
                .map(|f| f.ident.as_ref())
                .map(|i| quote!(hm.insert(stringify!(#i).to_string(), #i.to_json());));
            quote! {
                #ident::#var_ident{#(#field_names),*} => {
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    #(#s)*
                    let mut hm_warp = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    hm_warp.insert(stringify!(#var_ident).to_string(), rust_json::JsonElem::Object(hm));
                    rust_json::JsonElem::Object(hm_warp)
                }
            }
        }
    }
}

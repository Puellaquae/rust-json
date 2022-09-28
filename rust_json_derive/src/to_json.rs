use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Index, Result, Variant};

use crate::attrs;

pub fn expand_serialize(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    match input.data {
        Data::Struct(struct_data) => Ok(serialize_struct(ident, struct_data)),
        Data::Enum(enum_data) => Ok(serialize_enum(ident, enum_data)),
        Data::Union(_) => Err(Error::new(
            ident.span(),
            "rust_json_derive not support union!",
        )),
    }
}

fn serialize_struct(ident: &Ident, struct_data: DataStruct) -> proc_macro2::TokenStream {
    let s = serialize_fields(ident, struct_data.fields);
    quote!(
        impl rust_json::ToJson for #ident {
            fn to_json(&self) -> rust_json::JsonElem {
                #s
            }
        }
    )
}

fn serialize_enum(ident: &Ident, enum_data: DataEnum) -> proc_macro2::TokenStream {
    let variants = enum_data.variants;
    let s = variants.iter().map(|v| serialize_varient(ident, v));
    quote!(
        impl rust_json::ToJson for #ident {
            fn to_json(&self) -> rust_json::JsonElem {
                match self {
                    #(#s),*
                }
            }
        }
    )
}

fn serialize_fields(ident: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(_) => {
            let s = fields
                .iter()
                .map(|f| {
                    (
                        attrs::get_name_from_attrs(&f.attrs, f.ident.as_ref().unwrap().to_string()),
                        f.ident.as_ref(),
                    )
                })
                .map(|(name, i)| quote!(hm.insert(#name.to_string(), self.#i.to_json());));
            quote!(
                {
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    #(#s)*
                    rust_json::JsonElem::Object(hm)
                }
            )
        }
        Fields::Unnamed(_) if fields.len() == 1 => {
            quote!(self.0.to_json())
        }
        Fields::Unnamed(_) => {
            let s = fields
                .iter()
                .enumerate()
                .map(|(i, _)| Index::from(i))
                .map(|i| quote!(vec.push(self.#i.to_json());));
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
    let var_name = attrs::get_name_from_attrs(&variant.attrs, var_ident.to_string());
    match fields {
        Fields::Unit => {
            quote!(#ident::#var_ident => rust_json::JsonElem::Str(#var_name.to_string()))
        }
        Fields::Unnamed(_) if fields.len() == 1 => {
            quote!(#ident::#var_ident(__field) => {
                let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                hm.insert(#var_name.to_string(), __field.to_json());
                rust_json::JsonElem::Object(hm)
            })
        }
        Fields::Unnamed(_) => {
            let field_names =
                (0..fields.len()).map(|i| Ident::new(&format!("__field{}", i), Span::call_site()));
            let s = (0..fields.len())
                .map(|i| Ident::new(&format!("__field{}", i), Span::call_site()))
                .map(|i| quote!(vec.push(#i.to_json());));
            quote! {
                #ident::#var_ident(#(#field_names),*) => {
                    let mut vec = Vec::<rust_json::JsonElem>::new();
                    #(#s)*
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    hm.insert(#var_name.to_string(), rust_json::JsonElem::Array(vec));
                    rust_json::JsonElem::Object(hm)
                }
            }
        }
        Fields::Named(_) => {
            let field_names = fields.iter().map(|f| f.ident.as_ref());
            let s = fields
                .iter()
                .map(|f| {
                    (
                        attrs::get_name_from_attrs(&f.attrs, f.ident.as_ref().unwrap().to_string()),
                        f.ident.as_ref(),
                    )
                })
                .map(|(name, i)| quote!(hm.insert(#name.to_string(), #i.to_json());));
            quote! {
                #ident::#var_ident{#(#field_names),*} => {
                    let mut hm = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    #(#s)*
                    let mut hm_wrap = std::collections::HashMap::<String, rust_json::JsonElem>::new();
                    hm_wrap.insert(#var_name.to_string(), rust_json::JsonElem::Object(hm));
                    rust_json::JsonElem::Object(hm_wrap)
                }
            }
        }
    }
}

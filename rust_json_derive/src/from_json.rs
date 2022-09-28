use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Result, Variant};

use crate::attrs;

pub fn expand_deserialize(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ident = input.ident;
    match input.data {
        Data::Struct(struct_data) => Ok(deserialize_struct(ident, struct_data)),
        Data::Enum(enum_data) => Ok(deserialize_enum(ident, enum_data)),
        Data::Union(_) => Err(Error::new(
            ident.span(),
            "rust_json_derive not support union!",
        )),
    }
}

fn deserialize_struct(ident: Ident, struct_data: DataStruct) -> proc_macro2::TokenStream {
    let s = deserialize_fields(&ident, struct_data.fields);
    quote!(
        impl rust_json::FromJson for #ident {
            fn from_json(json: rust_json::JsonElem) -> Option<#ident> {
                #s
            }
        }
    )
}

fn deserialize_enum(ident: Ident, enum_data: DataEnum) -> proc_macro2::TokenStream {
    let units = deserialize_enum_units(&ident, &enum_data);
    let tags = deserialize_enum_tags(&ident, &enum_data);
    quote!(
        impl rust_json::FromJson for #ident {
            fn from_json(json: rust_json::JsonElem) -> Option<#ident> {
                match json {
                    rust_json::JsonElem::Str(__str) => {
                        match __str.as_str() {
                            #units
                            _ => None
                        }
                    },
                    rust_json::JsonElem::Object(mut __obj) => {
                        if __obj.len() != 1 {
                            None
                        } else {
                            let __key = __obj.keys().next().unwrap().clone();
                            let __val = __obj.remove(&__key).unwrap();
                            match __key.as_str() {
                                #tags
                                _ => None
                            }
                        }
                    },
                    _ => None
                }
            }
        }
    )
}

fn deserialize_fields(ident: &Ident, fields: Fields) -> proc_macro2::TokenStream {
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
                .map(|(name, i)| quote!(#i: obj.remove(#name).unwrap().get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Object(mut obj) = json {
                    Some(#ident{#(#s,)*})
                } else {
                    None
                }
            )
        }
        Fields::Unnamed(_) if fields.len() == 1 => {
            let ty = &fields.iter().next().unwrap().ty;
            quote!(
                if let Some(__field) = json.get::<#ty>() {
                    Some(#ident(__field))
                } else {
                    None
                }
            )
        }
        Fields::Unnamed(_) => {
            let l = fields.len();
            let field_tmp = (0..l)
                .map(|i| Ident::new(&format!("__field{}", i), Span::call_site()))
                .rev()
                .map(|v| quote!(let #v = vec.pop().unwrap();));
            let s = (0..l)
                .map(|i| Ident::new(&format!("__field{}", i), Span::call_site()))
                .map(|v| quote!(#v.get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Array(mut vec) = json {
                    #(#field_tmp)*
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

fn deserialize_enum_units(ident: &Ident, enum_data: &DataEnum) -> proc_macro2::TokenStream {
    let s = enum_data
        .variants
        .iter()
        .filter(|v| matches!(v.fields, Fields::Unit))
        .map(|v| deserialize_enum_unit(ident, v));
    quote!(#(#s)*)
}

fn deserialize_enum_unit(ident: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let var_ident = &variant.ident;
    let name = attrs::get_name_from_attrs(&variant.attrs, var_ident.to_string());
    quote!(#name => Some(#ident::#var_ident),)
}

fn deserialize_enum_tags(ident: &Ident, enum_data: &DataEnum) -> proc_macro2::TokenStream {
    let s = enum_data
        .variants
        .iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
        .map(|v| deserialize_enum_tag(ident, v));
    quote!(#(#s)*)
}

fn deserialize_enum_tag(ident: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let var_ident = &variant.ident;
    let fields = &variant.fields;
    let name = attrs::get_name_from_attrs(&variant.attrs, var_ident.to_string());
    let val = match fields {
        Fields::Unnamed(_) if fields.len() == 1 => {
            let ty = &fields.iter().next().unwrap().ty;
            quote!(
                if let Some(__field) = __val.get::<#ty>() {
                    Some(#ident::#var_ident(__field))
                } else {
                    None
                }
            )
        }
        Fields::Unnamed(_) => {
            let l = fields.len();
            let field_tmp = (0..l)
                .map(|i| Ident::new(&format!("__field{}", i), Span::call_site()))
                .rev()
                .map(|v| quote!(let #v = vec.pop().unwrap();));
            let s = (0..l)
                .map(|i| Ident::new(&format!("__field{}", i), Span::call_site()))
                .map(|v| quote!(#v.get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Array(mut vec) = __val {
                    #(#field_tmp)*
                    Some(#ident::#var_ident(#(#s,)*))
                } else {
                    None
                }
            )
        }
        Fields::Named(_) => {
            let s = fields
                .iter()
                .map(|f| {
                    (
                        attrs::get_name_from_attrs(&f.attrs, f.ident.as_ref().unwrap().to_string()),
                        f.ident.as_ref(),
                    )
                })
                .map(|(name, i)| quote!(#i: obj.remove(#name).unwrap().get().unwrap()));
            quote!(
                if let rust_json::JsonElem::Object(mut obj) = __val {
                    Some(#ident::#var_ident{#(#s,)*})
                } else {
                    None
                }
            )
        }
        Fields::Unit => unreachable!(),
    };
    quote!(#name => #val,)
}

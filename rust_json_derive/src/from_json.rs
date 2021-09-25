use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DataEnum, DataStruct, Fields, Ident, Variant};

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

pub fn deserialize_enum(ident: Ident, enum_data: DataEnum) -> TokenStream {
    let units = deserialize_enum_units(&ident, &enum_data);
    let tags = deserialize_enum_tags(&ident, &enum_data);
    quote!(
        impl rust_json::FromJson for #ident {
            fn from_json(json: rust_json::JsonElem) -> Option<#ident> {
                match json {
                    rust_json::JsonElem::Str(__str) => {
                        match __str.as_str() {
                            #units
                        }
                    }
                    rust_json::JsonElem::Object(mut __obj) => {
                        if __obj.len() != 1 {
                            None
                        } else {
                            let __key = __obj.keys().next().unwrap();
                            let __val = __obj.remove(__key);
                            match __key.as_str() {
                                #tags
                            }
                        }
                    }
                    _ => {
                        None
                    }
                }
            }
        }
    )
    .into()
}

fn deserialize_fields(ident: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(_) => {
            let s = fields.iter().map(|f| f.ident.as_ref()).map(
                |i| quote!(#i: obj.remove(stringify!(#i)).unwrap().get().unwrap()),
            );
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
    let s = enum_data.variants.iter().map(|v| deserialize_enum_unit(ident, v));
    quote!(#(#s)*)
}

fn deserialize_enum_unit(ident: &Ident, variant: &Variant) -> proc_macro2::TokenStream {

}

fn deserialize_enum_tags(ident: &Ident, enum_data: &DataEnum) -> proc_macro2::TokenStream {

}

fn deserialize_varient(ident: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let fields = &variant.fields;
    let var_ident = &variant.ident;
    match fields {
        Fields::Unit => {
            quote!(rust_json::JsonElem::Str(stringify!(#var_ident).to_string()) => Some(#ident::#var_ident))
        }
        _ => {
            let ty = &fields.iter().next().unwrap().ty;
            quote!(rust_json::JsonElem::Object(mut obj) => {
                let json = obj.remove(stringify!(#var_ident));
                match json {
                    Some(rust_json::JsonElem::Array(__vec)) => {

                    }
                    Some(rust_json::JsonElem::Object(__obj)) => {

                    }
                    Some(__var) => {

                    }
                }
                if let Some(__field) = .get::<#ty>() {
                    Some(#ident::#var_ident(__field))
                } else {
                    None
                }
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
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToJson)]
pub fn derive_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    if let Data::Struct(struct_data) = input.data {
        let fields = struct_data.fields;
        if let Fields::Named(_) = fields {
            let s: Vec<_> = fields
                .iter()
                .map(|f| f.ident.as_ref())
                .map(|i| quote!(hm.insert(String::from(stringify!(#i)), self.#i.to_json());))
                .collect();
            return quote!(
                impl ToJson for #ident {
                    fn to_json(&self) -> ::rust_json::JsonElem {
                        let mut hm = std::collections::HashMap::<String, ::rust_json::JsonElem>::new();
                        #(#s)*
                        ::rust_json::JsonElem::Object(hm)
                    }
                }
            )
            .into();
        }
    }
    quote!().into()
}

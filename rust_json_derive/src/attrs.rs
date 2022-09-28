use syn::Attribute;

pub fn get_name_from_attrs(attrs: &[Attribute], fallback: String) -> String {
    let rename = attrs.iter().flat_map(|a| a.parse_meta()).find_map(|m| {
        if let syn::Meta::NameValue(nv) = m {
            if nv.path.is_ident("rename") {
                Some(nv.lit)
            } else {
                None
            }
        } else {
            None
        }
    });
    rename
        .and_then(|l| match l {
            syn::Lit::Str(s) => Some(s.value()),
            syn::Lit::Verbatim(s) => Some(s.to_string()),
            _ => None,
        })
        .unwrap_or(fallback)
}

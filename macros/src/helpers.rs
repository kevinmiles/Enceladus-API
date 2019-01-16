use proc_macro2::TokenTree;
use syn::{Attribute, Field, Fields};

pub fn fields_without_attribute<'a>(fields: &'a Fields, attr: &str) -> Vec<&'a Field> {
    fields
        .iter()
        .filter(|field| !attribute_is_present(&field.attrs, attr))
        .collect()
}

pub fn attribute_is_present(attrs: &Vec<Attribute>, name: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.parse_meta().unwrap().name() == name)
}

pub fn get_attribute_equals(attrs: &Vec<Attribute>, name: &str) -> Option<Option<TokenTree>> {
    let tts: Option<Vec<_>> = attrs
        .iter()
        .find(|attr| attr.parse_meta().unwrap().name() == name)
        .map(|attr| attr.tts.clone().into_iter().collect());

    match tts {
        Some(tts) => {
            if tts.len() != 2 || format!("{}", tts[0]) != "=" {
                Some(None)
            } else {
                Some(Some(tts[1].clone()))
            }
        }
        None => None,
    }
}

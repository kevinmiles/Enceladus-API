#![feature(proc_macro_hygiene, crate_visibility_modifier)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Meta};

#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("Cannot derive `Display` on non-enum types"),
    };

    let fields: Vec<_> = variants
        .into_iter()
        .map(|variant| {
            let field = variant.ident;
            let attrs = variant.attrs;

            let mut disp_value = None;

            for attr in attrs.iter() {
                let name_value = match attr.parse_meta().expect("Invalid attribute") {
                    Meta::NameValue(name_value) => name_value,
                    _ => panic!(r#"Expected format `#[display = "foo"]"#),
                };

                if name_value.ident == Ident::new("display", Span::call_site()) {
                    // We have the attribute we want, let's get the value to display.
                    disp_value = Some(name_value.lit);
                }
            }

            let disp_value = disp_value.expect("Value must be set");

            quote!(#name::#field => #disp_value)
        })
        .collect();

    TokenStream::from(quote! {
        impl std::fmt::Display for #name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        #(#fields),*
                    }
                )
            }
        }
    })
}

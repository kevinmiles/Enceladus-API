#![feature(proc_macro_hygiene)]

extern crate proc_macro;
mod helpers;

use crate::helpers::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitStr};

#[proc_macro_derive(InsertStruct, attributes(no_insert, table_name, insert_default))]
pub fn generate_insert_struct(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let (visibility, generics) = (input.vis, input.generics);
    let ident = Ident::new(&format!("Insert{}", &input.ident), Span::call_site());
    let table_name = get_attribute_equals(&input.attrs, "table_name");

    let mut generated_functions = vec![];

    let fields: Vec<_> = fields_without_attribute(&input.fields, "no_insert")
        .iter()
        .map(
            |field| match get_attribute_equals(&field.attrs, "insert_default") {
                None => quote!(#field),
                Some(default) => {
                    let field_attrs = &field.attrs;
                    let field_vis = &field.vis;
                    let field_ident = &field.ident;
                    let field_type = &field.ty;

                    match default {
                        None => quote! {
                            #(#field_attrs)*
                            #[serde(default)]
                            #field_vis #field_ident: #field_type
                        },
                        Some(default) => {
                            let fn_name = Ident::new(
                                &format!("_ENC_MACRO_DEFAULT_{}", field_ident.as_ref().unwrap()),
                                Span::call_site(),
                            );
                            let fn_name_str = LitStr::new(
                                &format!("_ENC_MACRO_DEFAULT_{}", field_ident.as_ref().unwrap()),
                                Span::call_site(),
                            );

                            let function = quote! {
                                #[inline(always)]
                                fn #fn_name() -> #field_type {
                                    #default.into()
                                }
                            };

                            generated_functions.push(function);

                            quote! {
                                #(#field_attrs)*
                                #[serde(default = #fn_name_str)]
                                #field_vis #field_ident: #field_type
                            }
                        }
                    }
                }
            },
        )
        .collect();

    TokenStream::from(quote! {
        #(#generated_functions)*
        #[derive(serde::Deserialize, rocket_contrib::databases::diesel::Insertable)]
        #[serde(deny_unknown_fields)]
        #[table_name = #table_name]
        #visibility struct #generics #ident {
            #(#fields),*
        }
    })
}

#[proc_macro_derive(UpdateStruct, attributes(no_update, table_name))]
pub fn generate_update_struct(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let (visibility, generics) = (input.vis, input.generics);
    let ident = Ident::new(&format!("Update{}", &input.ident), Span::call_site());
    let table_name = get_attribute_equals(&input.attrs, "table_name");
    let fields = fields_without_attribute(&input.fields, "no_update");

    let fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_attrs = &field.attrs;
            let field_vis = &field.vis;
            let field_ident = &field.ident;
            let field_type = &field.ty;

            quote! {
                #(#field_attrs)*
                #field_vis #field_ident: Option<#field_type>
            }
        })
        .collect();

    TokenStream::from(quote! {
        #[derive(serde::Deserialize, rocket_contrib::databases::diesel::AsChangeset)]
        #[serde(deny_unknown_fields)]
        #[table_name = #table_name]
        #visibility struct #generics #ident {
            #(#fields),*
        }
    })
}

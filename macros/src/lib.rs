#![feature(proc_macro_hygiene)]
#![allow(clippy::eval_order_dependence)]

extern crate proc_macro;

mod keyword;
use crate::keyword::{kw, Keyword};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Expr, Ident, Result, Token, Type,
};

struct Declaration {
    name: Ident,
    _paren: token::Paren,
    table_name: Expr,
    _brace: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let paren_content;
        let brace_content;

        Ok(Declaration {
            name: input.parse()?,
            _paren: parenthesized!(paren_content in input),
            table_name: paren_content.parse()?,
            _brace: braced!(brace_content in input),
            fields: brace_content.parse_terminated(Field::parse)?,
        })
    }
}

struct Field {
    attribute: Option<Keyword>,
    name: Ident,
    _colon: Token![:],
    typ: Type,
    default: Option<Expr>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            attribute: if input.peek(kw::auto)
                || input.peek(kw::readonly)
                || input.peek(kw::private)
            {
                Some(input.parse()?)
            } else {
                None
            },
            name: input.parse()?,
            _colon: input.parse()?,
            typ: input.parse()?,
            default: if input.peek(Token![=]) {
                // throw away the `=` token
                input.parse::<Token![=]>()?;
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

#[proc_macro]
pub fn generate_structs(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Declaration);

    let name = input.name;
    let insert_name = Ident::new(&format!("Insert{}", name), name.span());
    let update_name = Ident::new(&format!("Update{}", name), name.span());
    let table = input.table_name;

    let mut general_fields = vec![];
    let mut insert_fields = vec![];
    let mut update_fields = vec![];
    let mut generated_fns = vec![];

    for field in input.fields {
        let attribute = field.attribute;
        let name = field.name;
        let typ = field.typ;
        let default = field.default;

        // may or may not need this in any given iteration
        let fn_name: String = {
            let mut rng = thread_rng();
            let rand_value: String = iter::repeat(())
                .map(|_| rng.sample(Alphanumeric))
                .take(20)
                .collect();

            // prefix with an underscore the prevent an identifier with an initial numeric
            format!("_{}", rand_value)
        };
        let fn_name_ident = Ident::new(&fn_name, Span::call_site());

        // set attributes indicating what actions are performed for a given field
        let mut insertable = true;
        let mut updateable = true;
        let mut serializable = true;
        match attribute {
            Some(Keyword::Auto) => {
                insertable = false;
                updateable = false;
            }
            Some(Keyword::Readonly) => updateable = false,
            Some(Keyword::Private) => serializable = false,
            None => {}
        };

        // add the field to the general struct,
        // skipping serialization if private
        general_fields.push(if serializable {
            quote!(pub #name: #typ)
        } else {
            quote!(#[serde(skip_serializing)] pub #name: #typ)
        });

        // add the field to the insertables,
        // with an optional default
        if insertable {
            insert_fields.push(match default {
                Some(_) => quote!(#[serde(default = #fn_name)] pub #name: #typ),
                None => quote!(pub #name: #typ),
            });
        }

        // add the field to the updateables
        if updateable {
            update_fields.push(quote!(pub #name: Option<#typ>));
        }

        // create the function containing our default value
        if let Some(default) = default {
            generated_fns.push(quote! {
                #[inline(always)]
                fn #fn_name_ident() -> #typ {
                    #default.into()
                }
            });
        }
    }

    TokenStream::from(quote! {
        #(#generated_fns)*

        #[derive(serde::Serialize, serde::Deserialize, rocket_contrib::databases::diesel::Queryable)]
        #[table_name = #table]
        #[serde(deny_unknown_fields)]
        pub struct #name {
            #(#general_fields),*
        }

        #[derive(serde::Deserialize, rocket_contrib::databases::diesel::Insertable)]
        #[table_name = #table]
        #[serde(deny_unknown_fields)]
        pub struct #insert_name {
            #(#insert_fields),*
        }

        #[derive(serde::Deserialize, rocket_contrib::databases::diesel::AsChangeset)]
        #[table_name = #table]
        #[serde(deny_unknown_fields)]
        pub struct #update_name {
            #(#update_fields),*
        }
    })
}

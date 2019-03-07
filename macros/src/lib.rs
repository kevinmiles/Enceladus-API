#![feature(proc_macro_hygiene, decl_macro, crate_visibility_modifier)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)] // Don't deny, as there may be unknown bugs.
#![allow(
    intra_doc_link_resolution_failure,
    clippy::match_bool,
    clippy::eval_order_dependence
)]

extern crate proc_macro;

mod keyword;
use keyword::{kw, Keyword};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;
use syn::{
    braced,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token,
    Expr,
    Ident,
    Result,
    Token,
    Type,
};

/// A full declaration of a `struct` & its associated fields.
///
/// Should be of the following form:
///
/// ```rust,ignore
/// Foo("bar") {
///     baz: String,
///     auto qux = "default",
///     private foobar: bool,
///     readonly barbaz: Vec<String> = vec![],
/// }
/// ```
struct Declaration {
    name:       Ident,
    _paren:     token::Paren,
    table_name: Expr,
    _brace:     token::Brace,
    fields:     Punctuated<Field, Token![,]>,
}

impl Parse for Declaration {
    /// Parse a full declaration of a `struct` & its associated fields.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let paren_content;
        let brace_content;

        Ok(Declaration {
            name:       input.parse()?,
            _paren:     parenthesized!(paren_content in input),
            table_name: paren_content.parse()?,
            _brace:     braced!(brace_content in input),
            fields:     brace_content.parse_terminated(Field::parse)?,
        })
    }
}

/// A single field, along with its type, optional default value, and optional attribute.
struct Field {
    attribute: Option<Keyword>,
    name:      Ident,
    _colon:    Token![:],
    typ:       Type,
    default:   Option<Expr>,
}

impl Parse for Field {
    /// Parse a field, likely within a full `Declaration`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Field {
            attribute: if input.peek(kw::auto)
                || input.peek(kw::readonly)
                || input.peek(kw::private)
            {
                Some(input.parse()?)
            } else {
                None
            },
            name:      input.parse()?,
            _colon:    input.parse()?,
            typ:       input.parse()?,
            default:   if input.peek(Token![=]) {
                // throw away the `=` token
                input.parse::<Token![=]>()?;
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

/// Generate the regular, insert, and update `struct`s from the AST.
/// Additionally, for all fields that have a default value,
/// create a function (that is always inlined) with a random name
/// to satisfy serde's constraint on a default needing to be a function (so not literals).
/// The names are randomly generated (and not based on any sort of hash),
/// preventing any external observers from relying on them in any manner.
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

        // May or may not need this in any given iteration.
        let fn_name: String = {
            // Get a random 20 character alphanumeric string.
            let mut rng = thread_rng();
            let rand_value: String = iter::repeat(())
                .map(|_| rng.sample(Alphanumeric))
                .take(20)
                .collect();

            // Prefix with an underscore the prevent an identifier with an initial numeric.
            format!("_{}", rand_value)
        };
        let fn_name_ident = Ident::new(&fn_name, Span::call_site());

        // Set attributes indicating what actions are performed for a given field.
        // These can be modified by the various `Keyword`s.
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

        // Add the field to the general struct,
        // skipping serialization if private.
        general_fields.push(match serializable {
            true => quote!(pub #name: #typ),
            false => quote!(#[serde(skip_serializing)] pub #name: #typ),
        });

        // Add the field to the insertables,
        // with an optional default.
        if insertable {
            insert_fields.push(match default {
                Some(_) => quote!(#[serde(default = #fn_name)] pub #name: #typ),
                None => quote!(pub #name: #typ),
            });
        }

        // Add the field to the updateables.
        if updateable {
            update_fields.push(
                quote!(#[serde(skip_serializing_if="Option::is_none")] pub #name: Option<#typ>),
            );
        }

        // Create the function containing our default value.
        // These are always inlined,
        // as default literals should have minimal cost.
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

        #[derive(
            Clone,
            serde::Serialize, serde::Deserialize,
            rocket_contrib::databases::diesel::Queryable
        )]
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

        #[derive(
            Default,
            serde::Serialize,
            serde::Deserialize,
            rocket_contrib::databases::diesel::AsChangeset,
        )]
        #[table_name = #table]
        #[serde(deny_unknown_fields)]
        pub struct #update_name {
            #(#update_fields),*
        }
    })
}

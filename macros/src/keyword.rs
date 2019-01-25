use syn::{
    parse::{Parse, ParseStream},
    Result,
};

// define custom keywords,
// which alter which `struct`s the field is emitted on
pub mod kw {
    syn::custom_keyword!(auto);
    syn::custom_keyword!(readonly);
    syn::custom_keyword!(private);
}

// represent the keywords as a type
pub enum Keyword {
    /// Cannot be inserted or updated
    Auto,

    /// Cannot be updated
    Readonly,

    /// Not serialized by serde
    Private,
}

// let us expect any given keyword
impl Parse for Keyword {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::auto) {
            input.parse::<kw::auto>()?;
            Ok(Keyword::Auto)
        } else if input.peek(kw::readonly) {
            input.parse::<kw::readonly>()?;
            Ok(Keyword::Readonly)
        } else if input.peek(kw::private) {
            input.parse::<kw::private>()?;
            Ok(Keyword::Private)
        } else {
            Err(input.error("expected `auto`, `readonly`, or `private`"))
        }
    }
}

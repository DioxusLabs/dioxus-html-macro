use crate::close_tag::CloseTag;
use crate::element::closing;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

pub struct Unopened {
    option: Option<CloseTag>,
}

impl Parse for Unopened {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Unopened { option: None });
        } else {
            let tag: CloseTag = input.parse()?;
            Err(closing(&tag.tagname))
        }
    }
}

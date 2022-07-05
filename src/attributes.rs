use crate::attribute::Attribute;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
struct Attributes(Vec<Attribute>);

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = vec![];
        loop {
            if input.peek(Token![/]) || input.peek(Token![>]) {
                break Ok(Attributes(attrs));
            }
            let attr = input.parse()?;
            attrs.push(attr);
        }
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attribute in &self.0 {
            tokens.extend(quote!(#attribute));
        }
    }
}

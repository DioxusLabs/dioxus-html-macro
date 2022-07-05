use crate::element::Element;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

#[derive(Default)]
pub struct Html {
    pub elements: Vec<Element>,
}

impl Parse for Html {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut elements = vec![];
        while !input.is_empty() && !input.peek2(Token![/]) {
            elements.push(input.parse()?);
        }

        Ok(Html { elements })
    }
}

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for element in &self.elements {
            tokens.extend(quote! {
                #element
            });
        }
    }
}

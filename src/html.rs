use crate::prelude::*;

/// A recursive HTML AST.
#[derive(Default)]
pub struct Html {
    pub elements: Vec<Item>,
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

use crate::prelude::*;

pub struct Attributes(Vec<Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
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
        let Attributes(attributes) = self;
        tokens.extend(quote! {#(#attributes,)*})
    }
}

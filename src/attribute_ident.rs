use crate::prelude::*;

/// represents either an identifier or a string literal
pub enum AttributeIdent {
    Ident(Ident),
    LitStr(LitStr),
}

impl Parse for AttributeIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok({
            if input.peek(LitStr) {
                Self::LitStr(input.parse()?)
            } else {
                Self::Ident(input.parse()?)
            }
        })
    }
}

impl ToTokens for AttributeIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend({
            match self {
                Self::Ident(ident) => quote!(#ident),
                Self::LitStr(litstr) => quote!(#litstr),
            }
        });
    }
}

impl AttributeIdent {
    pub fn is_ident(&self) -> bool {
        matches!(&self, Self::Ident(_))
    }
}

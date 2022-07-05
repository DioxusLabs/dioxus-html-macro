use crate::rsx_expr::RsxExpr;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::Parse;
pub struct Attribute {
    pub name: Ident,
    pub equals: Token![=],
    pub value: RsxExpr,
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let equals = input.parse()?;
        let value = input.parse()?;

        Ok(Attribute {
            name,
            equals,
            value,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attribute { name, value, .. } = self;
        tokens.extend(quote! {
            #name: #value
        });
    }
}

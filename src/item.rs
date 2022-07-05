use crate::element::Element;
use crate::rsx_expr::RsxExpr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::token::Brace;
use syn::LitStr;
use Item::*;

pub enum Item {
    Element(Element),
    Expr(RsxExpr),
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        let expr = lookahead1.peek(Brace) || lookahead1.peek(LitStr);
        Ok(if expr {
            Expr(input.parse()?)
        } else {
            Element(input.parse()?)
        })
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Element(el) => tokens.extend(quote!(#el)),
            Expr(expr) => tokens.extend(quote!(#expr)),
        }
    }
}

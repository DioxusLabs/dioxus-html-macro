use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse::Parse, spanned::Spanned, token::Brace, Expr, LitStr};

pub enum RsxExpr {
    LitStr(LitStr),
    Expr { brace: Brace, expr: Expr },
}

impl Parse for RsxExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = match input.parse() {
            Ok(lit) => RsxExpr::LitStr(lit),
            Err(_) => {
                let expr;
                let brace = braced!(expr in input);
                RsxExpr::Expr {
                    expr: expr.parse()?,
                    brace,
                }
            }
        };
        Ok(expr)
    }
}

impl ToTokens for RsxExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RsxExpr::LitStr(lit) => tokens.extend(quote!(#lit)),
            RsxExpr::Expr { expr, .. } => tokens.extend(quote!({#expr})),
        }
    }
}

impl RsxExpr {
    pub fn span(&self) -> Span {
        match self {
            RsxExpr::Expr { expr, .. } => expr.span(),
            RsxExpr::LitStr(lit) => lit.span(),
        }
    }
    pub fn is_str(&self) -> bool {
        matches!(self, RsxExpr::LitStr(_))
    }
}

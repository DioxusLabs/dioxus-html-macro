use proc_macro2::Span;
use syn::{token::Brace, spanned::Spanned};
use crate::prelude::*;

/// An RSX expression can either be a string literal, 
/// or any rust expression inside curly braces. 
pub enum RsxExpr {
    LitStr(LitStr),
    Expr { brace: Brace, expr: Expr },
}

impl Parse for RsxExpr {
    fn parse(input: ParseStream) -> Result<Self> {
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

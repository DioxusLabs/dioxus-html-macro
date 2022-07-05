use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::Parse;

pub struct CloseTag {
  pub open_angle: Token![<],
  pub fslash: Token![/],
  pub tagname: Ident,
  pub close_angle: Token![>],
}

impl Parse for CloseTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let open_angle = input.parse()?;
        let fslash = input.parse()?;
        let tagname: Ident = input.parse()?;
        let close_angle = input.parse()?;

        Ok(CloseTag {
            open_angle,
            fslash, 
            tagname,
            close_angle,
        })
    }
}



// impl ToTokens for OpenTag {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//     }
// }

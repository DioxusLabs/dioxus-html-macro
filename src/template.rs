use proc_macro2::TokenStream; 
use quote::ToTokens;
use syn::parse::Parse;

struct Html {
    
}

impl Parse for Html {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Html {})
    }
}

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

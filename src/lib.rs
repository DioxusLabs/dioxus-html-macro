#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use assertions::AssertStreamIsEmpty;
use html::Html;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

mod assertions;
mod attribute;
mod attributes;
mod close_tag;
mod element;
mod html;
mod item;
mod open_tag;
mod rsx_expr;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let html: HtmlNonRecursive = parse_macro_input!(input);
    quote! {
        dioxus::prelude::rsx! {
            #html
        }
    }
    .into()
}
/// Unlike Html, HtmlNonRecursive attepts to consume
/// the entire stream, and errors if it fails.
struct HtmlNonRecursive {
    html: Html,
}

impl Parse for HtmlNonRecursive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html = input.parse()?;
        let _: AssertStreamIsEmpty = input.parse()?;
        Ok(HtmlNonRecursive { html })
    }
}

impl ToTokens for HtmlNonRecursive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlNonRecursive { html } = self;
        tokens.extend(if !html.elements.is_empty() {
            quote!(#html)
        } else {
            quote!("")
        });
    }
}

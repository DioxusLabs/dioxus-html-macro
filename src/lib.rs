#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use html::Html;
use proc_macro::TokenStream;
use syn::parse::Parse;
use unopened::Unopened;

mod attribute;
mod attributes;
mod close_tag;
mod element;
mod html;
mod item;
mod open_tag;
mod rsx_expr;
mod unopened;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let HtmlNonRecursive { html } = parse_macro_input!(input);
    quote!().into()
}

struct HtmlNonRecursive {
    html: Html,
}

impl Parse for HtmlNonRecursive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html = input.parse()?;
        let _: Unopened = input.parse()?;
        Ok(HtmlNonRecursive { html })
    }
}

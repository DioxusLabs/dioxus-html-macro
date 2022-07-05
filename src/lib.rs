#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use html_non_recursive::HtmlNonRecursive;

mod assertions;
mod attribute;
mod attribute_ident;
mod attributes;
mod close_tag;
mod element;
mod html;
mod html_non_recursive;
mod item;
mod open_tag;
mod prelude;
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

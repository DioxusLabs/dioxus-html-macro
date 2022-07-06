#![doc = include_str!("../README.md")]
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



/// macro for generating components using HTML syntax instead of rsx. 
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

#[cfg(test)]
#[test]
fn err_msgs() {
    let t = trybuild::TestCases::new(); 
    t.compile_fail("test/trailing_tag.rs"); 
    t.compile_fail("test/extra_close_tag.rs"); 
    t.compile_fail("test/missing_close_tag.rs"); 

}
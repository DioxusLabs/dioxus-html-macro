use crate::close_tag::{self, CloseTag};
use crate::html::Html;
use crate::open_tag::{self, OpenTag};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Error;
pub struct Element {
    open_tag: OpenTag,
    html: Html,
    close_tag: Option<CloseTag>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork(); 
        
        let open_tag: OpenTag = input.parse().map_err(|err| unmatched_msg(err, &&fork))?;
        
        let mut html = Default::default();
        let mut close_tag = None;
        if open_tag.fslash.is_none() {
            
            html = input.parse()?;
            
            
            close_tag = Some(input.parse().map_err(|_| opening(&open_tag.tagname))?);
            
        }

        let element = Element {
            open_tag,
            html,
            close_tag,
        };
        
        element.validate()?;
        
        Ok(element)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

impl Element {
    fn validate(&self) -> Result<(), syn::Error> {
        let name = &self.open_tag.tagname;
        match &self.close_tag {
            Some(tag) if tag.tagname != *name => {
                let mut error = closing(&tag.tagname);
                error.combine(opening(name));
                Err(error)
            }
            _ => Ok(()),
        }
    }
}

fn unmatched_msg(error: Error, fork: &ParseStream) -> Error {
    
    panic!("FOO"); 
    match fork.parse::<CloseTag>() {
        Ok(tag) => closing(&tag.tagname),
        Err(error) => error,
    }
}

pub fn closing(name: &Ident) -> Error {
    Error::new(
        name.span(),
        format!("closing html element \"{name}\" without a corresponding opening tag."),
    )
}

fn opening(name: &Ident) -> Error {
    Error::new(
        name.span(),
        format!("opening html element \"{name}\" without a corresponding closing tag."),
    )
}

#[test]
fn parse_str() {
    let html: Html = syn::parse_str("<html>  </html>").unwrap();
    println!("{}", html.elements.len());
}

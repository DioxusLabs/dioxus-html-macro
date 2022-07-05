use crate::close_tag::CloseTag;
use crate::open_tag::OpenTag;
use crate::html::Html;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Element { open_tag, html, .. } = self;
        let OpenTag {
            tagname,
            attributes,
            ..
        } = open_tag;

        tokens.extend(quote! {
            #tagname {
                #attributes
                #html
            }
        });
    }
}

impl Element {
    fn validate(&self) -> Result<(), syn::Error> {
        let name = &self.open_tag.tagname;
        match &self.close_tag {
            Some(tag) if tag.tagname != *name => {
                let mut error = opening(name);
                error.combine(closing(&tag.tagname));
                Err(error)
            }
            _ => Ok(()),
        }
    }
}

fn unmatched_msg(error: Error, fork: &ParseStream) -> Error {
    match fork.parse::<CloseTag>() {
        Ok(tag) => closing(&tag.tagname),
        Err(error) => error,
    }
}

pub fn closing(name: &Ident) -> Error {
    Error::new(
        name.span(),
        format!(
            "encountered a closing HTML element \"{name}\" without a corresponding opening tag."
        ),
    )
}

fn opening(name: &Ident) -> Error {
    Error::new(
        name.span(),
        format!(
            "encountered an opening HTML element \"{name}\" without a corresponding closing tag."
        ),
    )
}

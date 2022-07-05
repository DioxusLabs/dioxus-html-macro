use crate::prelude::*;

/// Unlike Html, HtmlNonRecursive attepts to consume
/// the entire stream, and errors if it fails.
pub struct HtmlNonRecursive {
    html: Html,
}

impl Parse for HtmlNonRecursive {
    fn parse(input: ParseStream) -> Result<Self> {
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

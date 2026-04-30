use crate::prelude::*;

/// represents a `<name attr="val" [/]>` part of
/// an HTML element.
pub struct OpenTag {
    pub name: Ident,
    pub attributes: Attributes,
    pub slash: Option<Token![/]>,
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![<] = input.parse()?;
        let name = input.parse()?;
        let attributes = input.parse()?;
        let slash = input.parse()?;
        let _: Token![>] = input.parse()?;

        Ok(OpenTag {
            name,
            attributes,
            slash,
        })
    }
}

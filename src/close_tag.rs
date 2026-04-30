use crate::prelude::*;

pub struct CloseTag {
    pub name: Ident,
}

impl Parse for CloseTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![<] = input.parse()?;
        let _: Token![/] = input.parse()?;
        let name: Ident = input.parse()?;
        let _: Token![>] = input.parse()?;

        Ok(CloseTag { name })
    }
}

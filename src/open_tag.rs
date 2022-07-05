use crate::prelude::*; 



pub struct OpenTag {
    pub open_angle: Token![<],
    pub name: Ident,
    pub attributes: Attributes,
    pub slash: Option<Token![/]>,
    pub close_angle: Token![>],
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let open_angle = input.parse()?;
        let name = input.parse()?;
        let attributes = input.parse()?;
        let slash = input.parse()?;
        let close_angle = input.parse()?;

        Ok(OpenTag {
            open_angle,
            name,
            attributes,
            slash,
            close_angle,
        })
    }
}

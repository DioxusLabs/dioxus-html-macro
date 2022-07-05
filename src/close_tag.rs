use crate::prelude::*; 

pub struct CloseTag {
    pub open_angle: Token![<],
    pub slash: Token![/],
    pub name: Ident,
    pub close_angle: Token![>],
}

impl Parse for CloseTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let open_angle = input.parse()?;
        let slash = input.parse()?;
        let name: Ident = input.parse()?;
        let close_angle = input.parse()?;

        Ok(CloseTag {
            open_angle,
            slash,
            name,
            close_angle,
        })
    }
}

// impl ToTokens for OpenTag {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//     }
// }

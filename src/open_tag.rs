use proc_macro2::Ident;

use syn::parse::Parse;

pub struct OpenTag {
    pub open_angle: Token![<],
    pub tagname: Ident,
    pub fslash: Option<Token![/]>,
    pub close_angle: Token![>],
}

impl Parse for OpenTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let open_angle = input.parse()?;
        let tagname = input.parse()?;
        let fslash = input.parse()?;
        let close_angle = input.parse()?;

        Ok(OpenTag {
            open_angle,
            tagname,
            fslash,
            close_angle,
        })
    }
}

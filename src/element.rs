use crate::prelude::*; 

pub struct Element {
    open_tag: OpenTag,
    html: Html,
    close_tag: Option<CloseTag>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        let open_tag: OpenTag = input.parse().map_err(|err| unmatched_msg(err, &&fork))?;

        let mut html = Default::default();
        let mut close_tag = None;
        if open_tag.slash.is_none() {
            html = input.parse()?;

            close_tag = Some(input.parse().map_err(|_| opening(&open_tag.name))?);
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
            name,
            attributes,
            ..
        } = open_tag;

        tokens.extend(quote! {
            #name {
                #attributes
                #html
            }
        });
    }
}

impl Element {
    fn validate(&self) -> Result<()> {
        let name = &self.open_tag.name;
        match &self.close_tag {
            Some(tag) if tag.name != *name => {
                let mut error = opening(name);
                error.combine(closing(&tag.name));
                Err(error)
            }
            _ => Ok(()),
        }
    }
}

fn unmatched_msg(error: Error, fork: &ParseStream) -> Error {
    match fork.parse::<CloseTag>() {
        Ok(tag) => closing(&tag.name),
        Err(_) => error,
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

use crate::prelude::*; 

pub struct Attribute {
    pub name: AttributeIdent,
    pub equals: Token![=],
    pub value: RsxExpr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let equals = input.parse()?;
        let value = input.parse()?;
        let attr = Attribute {
            name,
            equals,
            value,
        };
        attr.validate()?;
        Ok(attr)
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attribute { name, value, .. } = self;
        tokens.extend(quote! {
            #name: #value
        });
    }
}

impl Attribute {
    fn validate(&self) -> Result<()> {
        if !self.name.is_ident() && !self.value.is_str() {
            return Err(
                Error::new(
                    self.value.span(),
                "expected a string literal."
            )
        );
        }
        Ok(())
    }
}

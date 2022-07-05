use crate::close_tag::CloseTag;
use crate::element::closing;
use syn::parse::Parse;

/// This struct is used to make sure there are no trailing
/// closing html tags. Without this, the error message for
/// `<div></div></div>` wouldn't be very useful.
pub struct AssertStreamIsEmpty {
    _option: Option<CloseTag>,
}

impl Parse for AssertStreamIsEmpty {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(AssertStreamIsEmpty { _option: None });
        } else {
            let tag: CloseTag = input.parse()?;
            Err(closing(&tag.tagname))
        }
    }
}
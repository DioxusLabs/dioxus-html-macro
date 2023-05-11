use crate::close_tag::CloseTag;
use crate::element::closing;
use crate::prelude::*;

/// This struct is used to make sure there are no trailing
/// closing html tags. Without this, the error message for
/// `<div></div></div>` wouldn't be very useful.
pub struct AssertStreamIsEmpty {
    _option: Option<CloseTag>,
}

impl Parse for AssertStreamIsEmpty {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(AssertStreamIsEmpty { _option: None })
        } else {
            let tag: CloseTag = input.parse()?;
            Err(closing(&tag.name))
        }
    }
}

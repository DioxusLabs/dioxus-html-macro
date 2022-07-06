/// # dioxus html macro
/// This crate offers an `html!` like macro for
/// dioxus applications. It expands to the equivalent `rsx!` macro
/// call you would have made otherwise, so it does not rely on any
/// dioxus' internals.
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_html_macro::html;
/// fn app(cx: Scope) -> Element {
///     let mut count = use_state(&cx, || 0);
///     cx.render(html!(
///         <h1>"High-Five counter: {count}"</h1>
///         <button onclick={move |_| count += 1}>"Up high!"</button>
///         <button onclick={move |_| count -= 1}>"Down low!"</button>
///     ))
/// }
/// ```
/// Note that unlike HTML and JSX, styling of html tags is done via
/// attributes:
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_html_macro::html;
/// html!(
///     <h1 color="red">"Title"</h1>
/// );
/// ```

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use html_non_recursive::HtmlNonRecursive;
use proc_macro::TokenStream;

mod assertions;
mod attribute;
mod attribute_ident;
mod attributes;
mod close_tag;
mod element;
mod html;
mod html_non_recursive;
mod item;
mod open_tag;
mod prelude;
mod rsx_expr;

/// macro for generating components using HTML syntax instead of rsx.
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let html: HtmlNonRecursive = parse_macro_input!(input);
    quote! {
        dioxus::prelude::rsx! {
            #html
        }
    }
    .into()
}

#[cfg(test)]
#[test]
fn err_msgs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("test/tag/trailing.rs");
    t.compile_fail("test/tag/extra_close.rs");
    t.compile_fail("test/tag/missing_close.rs");


    t.compile_fail("test/attribute/non_str_custom.rs");
    t.compile_fail("test/attribute/format_str.rs");
    t.compile_fail("test/attribute/missing_equals.rs");
    t.compile_fail("test/attribute/random_expression.rs");
    t.pass("test/attribute/passes.rs");

    t.compile_fail("test/body/plain_text.rs");
        t.pass("test/body/expression.rs");
}

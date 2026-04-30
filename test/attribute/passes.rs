use dioxus::prelude::*;
use dioxus_html_macro::html;

#[allow(dead_code)]
fn app() -> Element {
    html!(
        <div "custom"="value" background_color="red" onclick={move |_| ()}>
        </div>
    )
}

fn main() {}

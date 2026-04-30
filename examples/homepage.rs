use dioxus::prelude::*;
use dioxus_html_macro::html;

fn app() -> Element {
    let mut count = use_signal(||0);

    html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={move |_| count += 1}>"Up high!"</button>
        <button onclick={move |_| count -= 1}>"Down low!"</button>
    )
}

fn main() {
    launch(app);
}

use dioxus::prelude::*;
use dioxus_html_macro::html;

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    cx.render(html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={move |_| count += 1}>"Up high!"</button>
        <button onclick={move |_| count -= 1}>"Down low!"</button>
    ))
}

fn main() {
    dioxus_desktop::launch(app);
}

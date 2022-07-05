use dioxus::prelude::*;
use dioxus_html_rsx::html;

fn app_rsx(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    ))
}

fn app_html(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    
    cx.render(html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={move |_| count += 1} color="red">"Up high!"</button>
        <button onclick={move |_| count -= 1}>"Down low!"</button>
    ))
}

fn main() {

    dioxus::desktop::launch(app_html); 
}


# dioxus html macro
This crate offers an `html!` like macro for 
dioxus applications. This macro uses `rsx!` internally.
```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    cx.render(html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={ move |_| count += 1 }>"Up high!"</button>
        <button onclick={ move |_| count -= 1 }>"Down low!"</button>
    ))
}
```
Note that unlike HTML and JSX, styling of html tags is done via 
attributes: 
```rust
html!(
    <h1 color="red">"High-Five counter: {count}"</h1>
)
```

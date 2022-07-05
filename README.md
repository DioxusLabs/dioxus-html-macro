
# dioxus html macro
This crate offers an `html!` like macro for 
dioxus applications. It expands to the equivalent `rsx!` macro
call you would have made otherwise, so it does not rely on any 
dioxus internals. 
```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    cx.render(html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={move |_| count += 1}>"Up high!"</button>
        <button onclick={move |_| count -= 1}>"Down low!"</button>
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

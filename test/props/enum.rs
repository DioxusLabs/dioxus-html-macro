use dioxus::prelude::*;
use dioxus_html_macro::html;

#[derive(Props, PartialEq, Eq, Clone)]
struct AppProps {
    number_is: Parity,
}

#[derive(PartialEq, Eq, Clone)]
enum Parity {
    Even,
    Odd,
}

#[allow(non_snake_case)]
fn App(props: AppProps) -> Element {
    let _ = props.number_is;
    html!(
        <App number_is={Parity::Even}/>
    )
}

fn main() {}

use dioxus_html_rsx::html;
use dioxus::prelude::*; 

fn foo() {
    
    let x = 10; 
    rsx! {div {
        "number: {x}"
    }};
    html!(<div>"number: {x}"</div>); 
}

fn main() {}

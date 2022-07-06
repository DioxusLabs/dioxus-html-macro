use dioxus::prelude::*; 
use dioxus_html_macro::html;

fn main() {
    struct Foo; 

    html!(
        <div "custom"="{Foo}">
        </div>
    );
}

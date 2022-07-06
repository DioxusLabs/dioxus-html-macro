use dioxus::prelude::*; 
use dioxus_html_macro::html;

fn main() {
    html!(
        <div name:"oops used a colon instead of equals">
        </div>
    );
}

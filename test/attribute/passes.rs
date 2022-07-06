use dioxus::prelude::*;
use dioxus_html_macro::html;

fn main() {
    
    html!(
        <div "custom"="value" background_color="red" onclick={move |_| ()}>
        </div>
    );
}

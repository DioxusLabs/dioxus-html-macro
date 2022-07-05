use dioxus_html_rsx::html;

fn foo() {
    html!();

    xml!(<div attr="asd" id={1}></div>); 
}

fn main() {}

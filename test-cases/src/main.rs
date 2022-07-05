use dioxus_html_rsx::html;
use dioxus::prelude::*; 

fn foo() {
    
    let color = "red"; 

    rsx! {
        div {
            h1 {
                "Lorem ipsum dolor sit"
            }
            p {
                "amet consectetur adipisicing elit."
            }
            img {
                src: "https://example.com", 
                alt: "Facilis harum sequi", 
            }
        }
    }; 
    html!(
        <div>
        <h1>"Lorem ipsum dolor sit"</h1> 
            <p>"amet consectetur adipisicing elit."</p>
            <img src="https://example.com" alt="Facilis harum sequi"/>
        </div>
    ); 
    
}

fn main() {}

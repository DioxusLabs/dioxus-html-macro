#![allow(non_upper_case_globals)]
#![allow(non_upper_case_globals)]
use dioxus::prelude::*; 
use dioxus_html_macro::html;


#[derive(Props, PartialEq, Eq)]
struct Props {
    number_is: Parity, 
}
#[derive(PartialEq, Eq)]
enum Parity {
    Even, 
    Odd
}

const App: Component<Props> = |ref s| {
    s.render(html!( 
        <App number_is={Parity::Even}/>
    ))

}; 

fn main() {

}

use dioxus_html_macro::html;

fn main() {
    html!(
        <h1>"High-Five counter: {count}"</h1>
        <button onclick={move |_| count += 1}>"Up high!"</button>
        <button onclick={move |_| count -= 1}>"Down low!"</button>
        </button> // <- trailing
    );
}

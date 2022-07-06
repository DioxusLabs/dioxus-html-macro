use dioxus_html_macro::html;

fn main() {
    html!(
        <div>
            <h1>"High-Five counter: {count}"
            <button onclick={move |_| count += 1}>"Up high!"</button>
            <button onclick={move |_| count -= 1}>"Down low!"</button>
        </div>
    );
}

use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{"page not found"}</h1>
        </div>
    }
}

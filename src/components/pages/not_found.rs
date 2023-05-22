use yew::{Html, html, function_component};

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html!(
        <h1>{"Page not found"}</h1>
    )
}

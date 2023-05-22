use yew::{Html, html, function_component};
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component(HomePage)]
pub fn home() -> Html {
    html!(
        <div>
            <h1>{"Home"}</h1>
            <p><Link<Route> to={Route::Program}>{"Program"}</Link<Route>></p>
            <p><Link<Route> to={Route::About}>{"About"}</Link<Route>></p>
        </div>
    )
}

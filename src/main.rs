extern crate console_error_panic_hook;

use std::panic;

use yew::prelude::*;
use stylist::{yew::styled_component};
use yew_router::{BrowserRouter, Switch};

use crate::router::{Route, switch};

mod components;
mod router;
mod api_functions;

fn main() {
    // Show extra debug info
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    //std::env::set_var("RUST_BACKTRACE", "1");
    yew::Renderer::<App>::new().render();
}

#[styled_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

use yew::{Html, html};
use yew_router::Routable;

use crate::components::pages::base::{Root, NotFound};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Root /> },
        Route::About => todo!(),
        Route::NotFound => html! { <NotFound /> },
    }
}

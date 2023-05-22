use yew::{Html, html};
use yew_router::{Routable, prelude::Redirect};

use crate::components::pages::{HomePage, NotFoundPage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/home")]
    Home,
    #[at("/about")]
    About,
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Root => html! { <Redirect<Route> to={Route::Home}/> },
        Route::Home => html! { <HomePage/> },
        Route::About => todo!(),
        Route::NotFound => html! { <NotFoundPage/> },
    }
}

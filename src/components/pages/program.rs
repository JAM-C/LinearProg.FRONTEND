use yew::{Html, html, function_component};

use crate::components::organelles::ProgramSpecification;

#[function_component(ProgramPage)]
pub fn program() -> Html {
    html!(
        <div>
            <h1>{"Program"}</h1>
            <ProgramSpecification/>
        </div>
    )
}

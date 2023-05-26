use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

use crate::components::atoms::AutosizeTextArea;

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {}

pub struct GPTBlock;

impl GPTBlock {
    fn text_div_style(&self) -> Style {
        style!(r#"
            width: 90%;
        "#).unwrap()
    }
}

impl Component for GPTBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                // Prompt text
                <p>{"AI prompt: type your problem here"}</p>
                // Text area
                <div class = { self.text_div_style() }>
                    <AutosizeTextArea/>
                </div>
                // Conversion button
                <button>{"Write me a problem definition!"}</button>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

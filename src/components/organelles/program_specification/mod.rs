use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

use gpt::GPTBlock;
use table::TableBlock;
use code::CodeBlock;

mod gpt;
mod table;
mod code;

#[derive(PartialEq,Properties)]
pub struct Props {
}

pub enum Msg {
    GPTRequest,
    CodeChanged,
    TableChanged,
}

pub struct ProgramSpecification;

impl Component for ProgramSpecification {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <GPTBlock/>
                <div display = "inline-block">
                    <TableBlock/>
                </div>
                <div display = "inline-block">
                    <CodeBlock/>
                </div>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

use gpt::GPTBlock;
use table::TableBlock;
use code::CodeBlock;

use crate::components::molecules::HorizontalAdjustableDiv;

mod gpt;
mod table;
mod code;

#[derive(PartialEq,Properties)]
pub struct Props {
}

pub enum Msg {}

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
                <HorizontalAdjustableDiv>
                    <TableBlock/>
                    <CodeBlock/>
                </HorizontalAdjustableDiv>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

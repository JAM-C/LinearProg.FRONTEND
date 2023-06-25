use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent, Callback};

use gpt::GPTBlock;
use table::TableBlock;
use code::CodeBlock;

use crate::components::molecules::HorizontalAdjustableDiv;

use self::table::Table;

mod gpt;
mod table;
mod code;

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {
    TableUpdated(Table),
    CodeUpdated(String),
}

pub struct ProgramSpecification;

impl ProgramSpecification {
    // Update callbacks
    fn update_table_callback(&self, ctx: &Context<Self>) -> Callback<Table>
        { ctx.link().callback( move |table : Table| { Msg::TableUpdated(table) } ) }

    fn update_code_callback(&self, ctx: &Context<Self>) -> Callback<String>
        { ctx.link().callback( move |code : String| { Msg::CodeUpdated(code) } ) }
}

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
                    <TableBlock updated_value = { None } onupdate = { self.update_table_callback(ctx) }/>
                    <CodeBlock/>
                </HorizontalAdjustableDiv>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

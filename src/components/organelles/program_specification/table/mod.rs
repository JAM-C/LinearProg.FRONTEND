use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {}

pub struct TableBlock;

impl Component for TableBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <table>
                    <tr>
                        <th>{"Equation"}</th>
                        <th>{"Var 1"}</th>
                        <th>{"Var 2"}</th>
                    </tr>
                </table>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

#[derive(PartialEq,Properties)]
pub struct Props {
    pub children: Children
}

pub enum Msg {}

pub struct HorizontalAdjustableDiv;

impl HorizontalAdjustableDiv {
    fn style(&self) -> Style {
        style!(r#"
            
        "#).unwrap()
    }
}

impl Component for HorizontalAdjustableDiv {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class = { self.style() }>
                { ctx.props().children.clone() }
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

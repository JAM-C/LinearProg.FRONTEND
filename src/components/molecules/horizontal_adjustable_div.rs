use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

#[derive(PartialEq,Properties)]
pub struct Props {
    pub children: Children
}

pub enum Msg {}

pub struct HorizontalAdjustableDiv;

impl HorizontalAdjustableDiv {
    fn container_style(&self) -> Style {
        style!(r#"
            
        "#).unwrap()
    }

    fn contained_style(&self) -> Style {
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
        let children = ctx.props().children.clone();
        html! {
            // External container
            <div class = { self.container_style() }>{
                // Place each child
                children.iter().map(|child| { html!{
                    // Into an internal container
                    <div class = { self.contained_style() }>
                        { child }
                    </div>
                }}).collect::<Html>()
            } </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

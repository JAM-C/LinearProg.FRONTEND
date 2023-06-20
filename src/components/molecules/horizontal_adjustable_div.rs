use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context};

#[derive(PartialEq,Properties)]
pub struct Props {
    pub children: Children
}

pub enum Msg {}

pub struct HorizontalAdjustableDiv;

impl HorizontalAdjustableDiv {
    fn container_style(&self) -> Style {
        style!(r#"
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;
        "#).unwrap()
    }

    fn contained_style_1(&self) -> Style {
        style!(r#"
            flex-grow: 1;
            resize: horizontal;
            overflow: auto;
            border: 2px solid;
        "#).unwrap()
    }

    fn contained_style_2(&self) -> Style {
        style!(r#"
            flex-grow: 1;
            border: 2px solid;
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
        // Get the children
        let children: Vec<_> = ctx.props().children.clone().iter().collect();
        // Map them onto a tuple containing themselves, plus whether they are the final element
        let final_index = children.len() - 1;
        let children = children.iter().enumerate().map(
                |(index, child)| { (child, index == final_index) }
        );
        // Return html containing
        html! {
            // External container
            <div class = { self.container_style() }>{
                // Iterate through children
                children    
                    // Adding them into an internal container
                    .map(|(child, final_element)| { html!{
                    <div class = {
                        // Whose style depends on whether it is the final element 
                        match final_element {
                            false => self.contained_style_1(),
                            true => self.contained_style_2(),
                        }
                    }> { child.clone() } </div>
                }}).collect::<Html>()
            } </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }
}

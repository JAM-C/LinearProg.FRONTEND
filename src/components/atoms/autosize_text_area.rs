use gloo::console::log;
use stylist::{Style, style};
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent, Callback};

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {
    TextBoxTyping((i32,u32))
}

/**
Math from https://upmostly.com/tutorials/autosizing-textarea-react
 */
#[derive(Default)]
pub struct AutosizeTextArea {
    text_area_rows: i32,
    scroll_height: i32,
    rows: u32,
}

impl AutosizeTextArea {
    // CSS
    fn textarea_style(&self) -> Style {
        style!(r#"
            resize: vertical;
            width: 100%;
            minwidth: 100%;
        "#).unwrap()
    }

    fn update_sizes(&mut self, scroll_height: i32, rows: u32) {
        self.scroll_height = scroll_height;
        self.rows = rows;
        self.adjust_height()
    }

    fn adjust_height(&mut self){
        let height = self.scroll_height; 
        let row_height = 15; 
        let div = (height as f64) / (row_height as f64);
        let trows = (div.ceil() as i32) - 1; 
        self.text_area_rows = trows;
    }

    fn adjust_height_callback(&self, ctx: &Context<Self>) -> Callback<yew::Event> {
        ctx.link().callback( |event : yew::Event| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlTextAreaElement>();
            let scroll_height = input.scroll_height();
            let rows = input.rows();
            Msg::TextBoxTyping((scroll_height,rows))
        })
    }
}

impl Component for AutosizeTextArea {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self {
            text_area_rows: 1,
            ..Default::default()
        }
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        log!(self.text_area_rows);
        html! { <textarea
                class = { self.textarea_style() }
                rows = { self.text_area_rows.to_string() }
                onchange = { self.adjust_height_callback(ctx) }
            />
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TextBoxTyping((scroll_height,rows)) => self.update_sizes(scroll_height, rows),
        }; log!(self.text_area_rows); true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    }

}

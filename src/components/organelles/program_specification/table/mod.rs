mod structure;
mod row;

pub use structure::Table;

use std::mem;

use gloo::console::log;
use stylist::{Style, style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Html, html, Properties, Component, Context, Callback};

use self::row::TableRow;

#[derive(PartialEq,Properties)]
pub struct Props {
    pub updated_value: Option<Table>,
    pub onupdate: Callback<Table>
}

pub enum Msg {
    NewRow,
    DeleteRow(usize),
    UpdateConstraint(usize,String),
    UpdateOp(usize,String),
    UpdateValue(usize,String),
    UpdateComment(usize,String),
}

/**
A table in the form:

| Constraint | Op | Value | Comment |

Where the user can insert constraints for the constraint satisfaction problem
 */
pub struct TableBlock {
    table: Table
}

fn unwrap_input_event(event: yew::Event) -> String {
    let target = event.target().unwrap();
    let input = target.unchecked_into::<HtmlInputElement>();
    return input.value();
            
}

impl TableBlock {
    // New and delete callbacks
    fn new_row_callback(&self, ctx: &Context<Self>) -> Callback<yew::MouseEvent>
        { ctx.link().callback( |_ : yew::MouseEvent| { Msg::NewRow } ) }

    fn delete_row_callback(&self, ctx: &Context<Self>, index: usize) -> Callback<yew::MouseEvent>
        { ctx.link().callback( move |_ : yew::MouseEvent| { Msg::DeleteRow(index) } ) }
    
    // Update callbacks
    fn update_constraint_callback(&self, ctx: &Context<Self>, index: usize) -> Callback<yew::Event>
        { ctx.link().callback( move |event : yew::Event| { Msg::UpdateConstraint(index,unwrap_input_event(event)) } ) }
    
    fn update_op_callback(&self, ctx: &Context<Self>, index: usize) -> Callback<yew::Event>
        { ctx.link().callback( move |event : yew::Event| { Msg::UpdateOp(index,unwrap_input_event(event)) } ) }

    fn update_value_callback(&self, ctx: &Context<Self>, index: usize) -> Callback<yew::Event>
        { ctx.link().callback( move |event : yew::Event| { Msg::UpdateValue(index,unwrap_input_event(event)) } ) }

    fn update_comment_callback(&self, ctx: &Context<Self>, index: usize) -> Callback<yew::Event>
        { ctx.link().callback( move |event : yew::Event| { Msg::UpdateComment(index,unwrap_input_event(event)) } ) }
    
}

impl Component for TableBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self { table: Table::default() }
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                // Create the table
                <table>
                    // Add table header
                    <tr>
                        <th>{"Constraint"}</th>
                        <th>{"Op"}</th>
                        <th>{"Value"}</th>
                        <th>{"Comment"}</th>
                    </tr>
                    // Add each row
                    {
                        self.table.rows.iter()
                            .enumerate()
                            .map(|(index,row)| { html!{
                                <tr>
                                    <th><input type = "text" value = {row.constraint.clone()} onchange = { self.update_constraint_callback(ctx, index) }/></th>
                                    <th>{">="}</th>
                                    <th><input type = "text" value = {row.value.clone()} onchange = { self.update_value_callback(ctx, index) }/></th>
                                    <th><input type = "text" value = {row.comment.clone()} onchange = { self.update_comment_callback(ctx, index) }/></th>
                                    <th><button onclick={self.delete_row_callback(ctx,index)}>{"Delete"}</button></th>
                                </tr>
                            }})
                            .collect::<Html>()
                    }
                </table>
                <button onclick={self.new_row_callback(ctx)}>{"New row"}</button>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewRow => { self.table.new_row() },
            Msg::DeleteRow(index) => { self.table.delete_row(index) },
            Msg::UpdateConstraint(index, constraint) => { self.table.update_constraint(index,constraint) },
            Msg::UpdateOp(_, _) => todo!(),
            Msg::UpdateValue(index, value) => { self.table.update_value(index,value) },
            Msg::UpdateComment(index, comment) => { self.table.update_comment(index,comment) },
        };
        true
    }
}

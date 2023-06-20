mod row;

use stylist::{Style, style};
use yew::{Html, html, Properties, Component, Context, Callback};

use self::row::TableRow;

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {
    NewRow,
    DeleteRow(usize)
}

/**
A table in the form:

| Constraint | Op | Value | Comment |

Where the user can insert constraints for the constraint satisfaction problem
 */
pub struct TableBlock {
    rows: Vec<TableRow>
}

impl TableBlock {
    fn new_row_callback(&self, ctx: &Context<Self>) -> Callback<yew::MouseEvent>
        { ctx.link().callback( |_ : yew::MouseEvent| { Msg::NewRow } ) }

    fn delete_row_callback(&self, ctx: &Context<Self>, row: usize) -> Callback<yew::MouseEvent>
        { ctx.link().callback( move |_ : yew::MouseEvent| { Msg::DeleteRow(row) } ) }
}

impl Component for TableBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self { rows: Vec::new() }
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
                        self.rows.iter()
                            .enumerate()
                            .map(|(index,row)| { html!{
                                <tr>
                                    <th><input type = "text"/></th>
                                    <th>{">="}</th>
                                    <th><input type = "text"/></th>
                                    <th><input type = "text"/></th>
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
            Msg::NewRow => { self.rows.push(TableRow::default()); },
            Msg::DeleteRow(index) => { self.rows.remove(index); },
        };
        true
    }
}

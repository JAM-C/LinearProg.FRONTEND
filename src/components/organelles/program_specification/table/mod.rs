mod row;

use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

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
                            .map(|row| { html!{
                                <tr>
                                    <th>{row.constraint.clone()}</th>
                                    <th>{">="}</th>
                                    <th>{row.value.clone()}</th>
                                    <th>{row.comment.clone()}</th>
                                </tr>
                            }})
                            .collect::<Html>()
                    }
                </table>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

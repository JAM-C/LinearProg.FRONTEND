#[derive(Default)]
pub enum Op {
    #[default]
    Ge,Geq,Le,Leq
}

#[derive(Default)]
pub struct TableRow {
    pub constraint: String,
    pub op: Op,
    pub value: String,
    pub comment: String,
}

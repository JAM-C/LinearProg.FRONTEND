#[derive(Default,Clone)]
pub enum Op {
    #[default]
    Ge,Geq,Le,Leq
}

#[derive(Default,Clone)]
pub struct TableRow {
    pub constraint: String,
    pub op: Op,
    pub value: String,
    pub comment: String,
}

impl TableRow {
    pub fn with_constraint(&self, constraint: String) -> Self {
        let mut new = self.clone();
        new.constraint = constraint;
        return new;
    }

    pub fn with_op(&self, op: Op) -> Self {
        let mut new = self.clone();
        new.op = op;
        return new;
    }

    pub fn with_value(&self, value: String) -> Self {
        let mut new = self.clone();
        new.value = value;
        return new;
    }

    pub fn with_comment(&self, comment: String) -> Self {
        let mut new = self.clone();
        new.comment = comment;
        return new;
    }
}
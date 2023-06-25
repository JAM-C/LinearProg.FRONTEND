use super::row::TableRow;

#[derive(Default, PartialEq)]
pub struct Table {
    pub rows: Vec<TableRow>
}

impl Table {
    pub fn new_row(&mut self)
        { self.rows.push(TableRow::default()); }

    pub fn delete_row(&mut self, index: usize)
        { self.rows.remove(index); }
    
    pub fn update_constraint(&mut self, index: usize, constraint: String)
        { self.rows[index] = self.rows[index].with_constraint(constraint); }
    
    pub fn update_value(&mut self, index: usize, value: String)
        { self.rows[index] = self.rows[index].with_value(value); }

    pub fn update_comment(&mut self, index: usize, comment: String)
        { self.rows[index] = self.rows[index].with_comment(comment); }
}

impl TryFrom<String> for Table {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Table> for String {
    fn from(value: Table) -> Self {
        todo!()
    }
}
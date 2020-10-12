#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn get_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|_c: &&Column| true)
    }

    pub fn get_pk_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|c: &&Column| c.is_pk)
    }

    pub fn get_non_pk_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|c: &&Column| !c.is_pk)
    }

    fn filter_columns_and_extract_names(
        &self,
        predicate: impl FnMut(&&Column) -> bool,
    ) -> Vec<String> {
        self.columns
            .iter()
            .filter(predicate)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub sql_type: String,
    pub is_pk: bool,
    pub is_nullable: bool,
    pub is_generated: bool
}

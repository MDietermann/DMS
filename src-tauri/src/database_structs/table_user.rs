pub struct TableUser {
    tables: Vec<String>,
    password: String,
}

impl TableUser {
    pub fn new(tables: Vec<String>, password: String) -> TableUser {
        TableUser { tables, password }
    }

    pub fn get_tables(&self) -> &Vec<String> {
        &self.tables
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn add_table(&mut self, table: Vec<String>) {
        for t in table {
            self.tables.push(t);
        }
    }
}

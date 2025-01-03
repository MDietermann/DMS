use super::database_group::DatabaseGroup;

pub struct DatabaseStructure {
    databases: Vec<DatabaseGroup>,
}

impl DatabaseStructure {
    pub fn generate_connection_string(&self) -> String {
        let mut connection_string = "";

        String::from(connection_string)
    }
}

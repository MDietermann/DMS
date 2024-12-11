use rusqlite::Connection;
use crate::custom_errors::{self, CommandResult, CustomRusqliteErrorType};
use tokio::sync::RwLock;

use super::Employee;
use crate::database_structs::database_structure::DatabaseStructure;

pub struct SqliteFactory {
    host: String,
    pub table: String,
    conn: RwLock<Connection>
}

pub enum ResultType {
    Employee(Employee),
    DatabaseStructure(DatabaseStructure)
}

impl SqliteFactory {
    pub fn new(table: String, host: Option<String>) -> SqliteFactory {
        SqliteFactory {
            host: "dms.db".to_string(),
            table,
            conn: RwLock::new(Connection::open(host.unwrap().to_string()).unwrap())
        }
    }

    pub async fn get_connection(&self) -> CommandResult<Connection, CustomRusqliteErrorType> {
        let connection = self.conn.read().await;
        match connection {
            Ok(connection.as_ref()) => {
                Ok(connection.clone())
            }
            Err(error) => {
                Err(custom_errors::get_custom_rusqlite_errors(Some(error)))
            }
        }
    }

    pub async fn get_all(&self) -> CommandResult<Vec<ResultType>, CustomRusqliteErrorType> {
        let query = "SELECT * FROM ?1";
        match &self.table {
            table if table == &"employee".to_string() => {
                let employees = Employee::get_all_employees(self, query).await?;
                let result = employees.into_iter().map(ResultType::Employee).collect();
                Ok(result)
            }
            table if table == &"database_structure".to_string() => {
                Ok(Vec::new())
            }
            _ => {
                Err(custom_errors::get_custom_rusqlite_errors(None))
            }
        }
    }
}

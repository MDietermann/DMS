use rusqlite::Connection;
use crate::custom_errors::{ CommandResult, CustomRusqliteError };
use tokio::sync::RwLock;
use tokio::task;
use std::sync::Arc;
use super::Employee;
use crate::database_structs::database_structure::DatabaseStructure;

pub struct SqliteFactory {
    host: String,
    pub table: String,
    conn: Arc<RwLock<Connection>>,
}

pub enum ResultType {
    Employee(Employee),
    DatabaseStructure(DatabaseStructure),
}

impl SqliteFactory {
    pub fn new(table: String, host: Option<String>) -> SqliteFactory {
        let host = host.unwrap_or_else(|| "dms.db".to_string());
        let conn = match Connection::open(&host) {
            Ok(conn) => Arc::new(RwLock::new(conn)),
            Err(_) => {
                // Handle error or provide a fallback connection
                Arc::new(RwLock::new(Connection::open_in_memory().unwrap())) // Placeholder
            }
        };
        SqliteFactory { host, table, conn }
    }

    pub async fn get_connection(&self) -> CommandResult<Arc<RwLock<Connection>>, CustomRusqliteError> {
        Ok(self.conn.clone())
    }

    pub async fn get_all(&self) -> CommandResult<Vec<ResultType>, CustomRusqliteError> {
        let conn = self.conn.clone();


        // Use block_in_place to run blocking code
        let result = task::block_in_place(move || {
            let conn = conn.read().unwrap(); // Move the RwLockReadGuard into the closure
            let table = &self.table;
            let query = format!("SELECT * FROM {}", table);

            // Prepare the query
            let mut stmt = conn.prepare(&query).expect("Failed to prepare query");
            let result_iter = stmt.query_map([], |row| Employee::from_row(row)).expect("Failed to execute query");

            // Collect results into a vector
            let result: Vec<ResultType> = result_iter
                .filter_map(|e| e.ok().map(ResultType::Employee))
                .collect();

            // Return the result
            Ok(result)
        });

        // Explicit type annotation in map_err
        result.map_err(|e: std::io::Error| CustomRusqliteError::DatabaseError(format!("Blocking task failed: {}", e)))
    }
}

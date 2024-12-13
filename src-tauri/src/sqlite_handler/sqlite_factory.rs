use crate::database_structs::database_structure::DatabaseStructure;
use crate::custom_errors::{ CommandResult, CustomRusqliteError };
use rusqlite::Connection;
use super::Employee;
use tokio::task;

/// Factory for creating connections to SQLite databases
pub struct SqliteFactory {
    /// The path to the SQLite database file
    host: String,

    /// The name of the table to query
    pub table: String,
}

/// Types of results that can be returned from a query
pub enum ResultType {
    /// The result is an `Employee` struct
    Employee(Employee),
}

impl SqliteFactory {
    /// Create a new `SqliteFactory` instance
    pub fn new(table: String, host: Option<String>) -> SqliteFactory {
        let host = host.unwrap_or_else(|| "dms.db".to_string());
        SqliteFactory { host, table }
    }

    /// Create a new connection to the database
    async fn get_connection(&self) -> CommandResult<Connection, CustomRusqliteError> {
        Connection::open(&self.host).map_err(|e| CustomRusqliteError::DatabaseError(format!("Failed to open connection: {}", e)))
    }

    /// Query the database and return the results
    pub async fn get_all(&self) -> CommandResult<Vec<ResultType>, CustomRusqliteError> {
        let conn = self.get_connection().await?;

        // Use block_in_place to run blocking code
        let result = task::block_in_place(|| async move {
            let table = &self.table;
            let query = format!("SELECT * FROM {}", table);

            // Prepare the query
            let mut stmt = conn.prepare(&query).map_err(|e| {
                CustomRusqliteError::DatabaseError(format!("Failed to prepare query: {}", e))
            })?;

            let result_iter = stmt.query_map([], |row| Employee::from_row(row)).map_err(|e| {
                CustomRusqliteError::DatabaseError(format!("Failed to execute query: {}", e))
            })?;

            // Collect results into a vector
            let result: Vec<ResultType> = result_iter
                .filter_map(|e| e.ok().map(ResultType::Employee))
                .collect();

            // Return the result
            Ok(result)
        }).await;

        // Explicit type annotation in map_err
        result
    }

    /// Query the database and return the results for a specific ID
    pub async fn get_by_id(&self, id: i32) -> CommandResult<Vec<ResultType>, CustomRusqliteError> {
        let conn = self.get_connection().await?;

        // Use block_in_place to run blocking code
        let result = task::block_in_place(|| async move {
            let table = &self.table;
            let query = format!("SELECT * FROM {} WHERE id = {}", table, id);

            // Prepare the query
            let mut stmt = conn.prepare(&query).map_err(|e| {
                CustomRusqliteError::DatabaseError(format!("Failed to prepare query: {}", e))
            })?;

            let result_iter = stmt.query_map([], |row| Employee::from_row(row)).map_err(|e| {
                CustomRusqliteError::DatabaseError(format!("Failed to execute query: {}", e))
            })?;

            // Collect results into a vector
            let result: Vec<ResultType> = result_iter
                .filter_map(|e| e.ok().map(ResultType::Employee))
                .collect();

            // Return the result
            Ok(result)
        }).await;

        result
    }
}

use serde::{ser::Serializer, Deserialize, Serialize};
use rusqlite::Error as SqliteError;

#[derive(Debug, thiserror::Error)]
pub enum CustomRusqliteError {
    #[error(transparent)]
    RusqliteError(#[from] SqliteError),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Invalid credentials")]
    InvalidCredentials(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl Serialize for CustomRusqliteError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let error_info = match *self {
            CustomRusqliteError::RusqliteError(ref err) => {
                serde_json::json!({"type": "RusqliteError", "message": err.to_string()})
            }
            CustomRusqliteError::Custom(ref msg) => {
                serde_json::json!({"type": "CustomError", "message": msg})
            }
            CustomRusqliteError::InvalidCredentials(ref msg) => {
                serde_json::json!({"type": "InvalidCredentials", "message": msg})
            }
            CustomRusqliteError::DatabaseError(ref msg) => {
                serde_json::json!({"type": "DatabaseError", "message": msg})
            }
        };
        error_info.serialize(serializer)
    }
}

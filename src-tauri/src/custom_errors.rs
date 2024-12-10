use serde::{ser::Serializer, Deserialize, Serialize};
use rusqlite::Error as SqliteError;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error(transparent)]
    RusqliteError(#[from] SqliteError),
}

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CustomError> = anyhow::Result<T, E>;

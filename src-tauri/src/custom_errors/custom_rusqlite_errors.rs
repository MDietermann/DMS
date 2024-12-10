use serde::{ser::Serializer, Deserialize, Serialize};
use rusqlite::Error as SqliteError;

#[derive(Debug, thiserror::Error)]
pub enum CustomRusqliteError {
    #[error(transparent)]
    RusqliteError(#[from] SqliteError),
}

impl Serialize for CustomRusqliteError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

mod custom_rusqlite_errors;
use rusqlite::Error;
pub use custom_rusqlite_errors::CustomRusqliteError;

pub fn get_custom_rusqlite_errors(error: Option<Error>) -> CustomRusqliteError {
    match error {
        Some(e) => {
            return CustomRusqliteError::RusqliteError(e);
        }
        None => return CustomRusqliteError::RusqliteError(rusqlite::Error::QueryReturnedNoRows),
    }
}

pub type CommandResult<T, E = CustomRusqliteError> = anyhow::Result<T, E>;
pub type CustomRusqliteErrorType = custom_rusqlite_errors::CustomRusqliteError;

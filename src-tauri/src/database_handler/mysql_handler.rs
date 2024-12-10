use crate::ip_factory::IpAddr;
use crate::custom_errors::{CommandResult, CustomRusqliteErrorType, get_custom_rusqlite_errors};

pub struct MySqlHandler {
    username: String,
    password: String,
    table: String,
    host: IpAddr,
    connection: Option<Connection>,
    port: i32
}

impl MySqlHandler {
    pub fn new(username: String, password: String, table: String, host: IpAddr) -> MySqlHandler {
        MySqlHandler { username, password, table, host, connection: todo!() }
    }

    pub async fn get_connection(&self) -> CommandResult<Connection, CustomRusqliteErrorType> {
        let connection = Connection::open_with_params(&[
            ("host", &self.host.get_ip_string()),
            ("port", "3306"),
            ("username", &self.username),
            ("password", &self.password),
            ("database", &self.table),
        ])?;
        match connection {
            Ok(connection) => {
                &self.connection = &Some(connection);
            }
            Err(e) => {
                Err(get_custom_rusqlite_errors(Some(e)))
            }
        }
    }

    pub async fn select_all(&self) -> Result<, rusqlite::Error> {

    }
}

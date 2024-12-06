use rusqlite::{Connection, Result};

pub struct MySqlDatabase {
    host: String,
}

pub async fn get_connection() -> Result<Connection> {
    let connection = Connection::open("dms.db")?;
    Ok(connection)
}

pub async fn create() -> Result<()> {
    let connection = Connection::open("dms.db")?;
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS employee
        (
            id          INTEGER  PRIMARY KEY NOT NULL,
            first_name  TEXT                 NOT NULL,
            last_name   TEXT                 NOT NULL,
            email       TEXT                 NOT NULL,
            position    TEXT                 NOT NULL,
            password    TEXT                 NOT NULL
        );",
        [],
    )?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS saved_mysql
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            host_ip     TEXT                NOT NULL,
            user_name   TEXT                NOT NULL,
            table_name  TEXT                NOT NULL,
            password    TEXT                NOT NULL
        );",
        [],
    )?;

    Ok(())
}


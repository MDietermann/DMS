use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Models definieren
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseType {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseServer {
    id: i32,
    ip: String,
    database_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseUser {
    id: i32,
    username: String,
    password: String,
    database_server_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTableMapping {
    id: i32,
    user_id: i32,
    table_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsStructure {
    database_type: String,
    servers: Vec<ServerInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    ip: String,
    users: HashMap<String, UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    password: String,
    tables: Vec<String>,
}

// Controller Implementierung
pub struct DbController {
    connection: Connection,
}

impl DbController {
    pub fn new(database_path: &str) -> Result<Self> {
        let connection = Connection::open(database_path)?;
        Ok(Self { connection })
    }

    // Alle Datenbanktypen abrufen
    pub fn get_all_database_types(&self) -> Result<Vec<DatabaseType>> {
        let mut stmt = self.connection.prepare("SELECT id, name FROM database_types")?;
        let rows = stmt.query_map([], |row| {
            Ok(DatabaseType {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        rows.collect()
    }

    // Alle Server für einen bestimmten Datenbanktyp abrufen
    pub fn get_servers_by_type(&self, db_type_id: i32) -> Result<Vec<DatabaseServer>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, ip, database_type_id FROM database_servers WHERE database_type_id = ?"
        )?;
        let rows = stmt.query_map(params![db_type_id], |row| {
            Ok(DatabaseServer {
                id: row.get(0)?,
                ip: row.get(1)?,
                database_type_id: row.get(2)?,
            })
        })?;

        rows.collect()
    }

    // Alle Benutzer für einen bestimmten Server abrufen
    pub fn get_users_by_server(&self, server_id: i32) -> Result<Vec<DatabaseUser>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, username, password, database_server_id FROM users WHERE database_server_id = ?"
        )?;
        let rows = stmt.query_map(params![server_id], |row| {
            Ok(DatabaseUser {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                database_server_id: row.get(3)?,
            })
        })?;

        rows.collect()
    }

    // Alle Tabellen für einen bestimmten Benutzer abrufen
    pub fn get_user_tables(&self, user_id: i32) -> Result<Vec<Table>> {
        let mut stmt = self.connection.prepare(
            "SELECT t.id, t.name \
             FROM tables t \
             INNER JOIN user_table_mappings utm ON t.id = utm.table_id \
             WHERE utm.user_id = ?"
        )?;
        let rows = stmt.query_map(params![user_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        rows.collect()
    }

    // Funktion, um die notwendigen Tabellen zu erstellen
    pub fn create_tables(&self) -> Result<()> {
        // Tabellen erstellen
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS database_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS database_servers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ip TEXT NOT NULL,
                database_type_id INTEGER NOT NULL,
                FOREIGN KEY(database_type_id) REFERENCES database_types(id)
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                database_server_id INTEGER NOT NULL,
                FOREIGN KEY(database_server_id) REFERENCES database_servers(id)
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tables (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS user_table_mappings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                table_id INTEGER NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id),
                FOREIGN KEY(table_id) REFERENCES tables(id)
            )",
            [],
        )?;

        Ok(())
    }

    // Generieren der TypeScript- Struktur
    pub fn generate_ts_structure(&self) -> Result<Vec<TsStructure>> {
        let mut result = Vec::new();

        let database_types = self.get_all_database_types()?;
        for db_type in database_types {
            let mut servers_info = Vec::new();
            let servers = self.get_servers_by_type(db_type.id)?;

            for server in servers {
                let mut users_info = HashMap::new();
                let users = self.get_users_by_server(server.id)?;

                for user in users {
                    let tables = self.get_user_tables(user.id)?
                        .into_iter()
                        .map(|table| table.name)
                        .collect();

                    users_info.insert(
                        user.username.clone(),
                        UserInfo {
                            password: user.password,
                            tables,
                        },
                    );
                }

                servers_info.push(ServerInfo {
                    ip: server.ip,
                    users: users_info,
                });
            }

            result.push(TsStructure {
                database_type: db_type.name,
                servers: servers_info,
            });
        }

        Ok(result)
    }

    // Importieren der TypeScript- Struktur
    pub fn import_ts_structure(&self, structure: Vec<TsStructure>) -> Result<()> {
        for db_type in structure {
            // Datenbanktyp einfügen
            self.connection.execute(
                "INSERT INTO database_types (name) VALUES (?)",
                params![db_type.database_type],
            )?;
            let db_type_id = self.connection.last_insert_rowid();

            for server in db_type.servers {
                // Server einfügen
                self.connection.execute(
                    "INSERT INTO database_servers (ip, database_type_id) VALUES (?, ?)",
                    params![server.ip, db_type_id],
                )?;
                let server_id = self.connection.last_insert_rowid();

                for (username, user_info) in server.users {
                    // Benutzer einfügen
                    self.connection.execute(
                        "INSERT INTO users (username, password, database_server_id) VALUES (?, ?, ?)",
                        params![username, user_info.password, server_id],
                    )?;
                    let user_id = self.connection.last_insert_rowid();

                    for table_name in user_info.tables {
                        // Tabelle einfügen, wenn sie noch nicht existiert
                        let mut stmt = self.connection.prepare(
                            "SELECT id FROM tables WHERE name = ?"
                        )?;
                        let table_id: Option<i32> = stmt
                            .query_map(params![table_name], |row| row.get(0))?
                            .next()
                            .transpose()?;

                        let table_id = match table_id {
                            Some(id) => id,
                            None => {
                                // Tabelle einfügen, wenn sie noch nicht existiert
                                self.connection.execute(
                                    "INSERT INTO tables (name) VALUES (?)",
                                    params![table_name],
                                )?;
                                self.connection.last_insert_rowid() as i32
                            }
                        };

                        // Benutzer auf Tabelle zuweisen
                        self.connection.execute(
                            "INSERT INTO user_table_mappings (user_id, table_id) VALUES (?, ?)",
                            params![user_id, table_id],
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn
}

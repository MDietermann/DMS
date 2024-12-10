use super::database_type::DatabaseType;
use super::database_server::DatabaseServer;

pub struct DatabaseGroup {
    db_type: DatabaseType,
    servers: Vec<DatabaseServer>
}

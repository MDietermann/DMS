use std::fs::File;
use std::io::Write;

use mysql::prelude::*;
use mysql::Pool;
use mysql::*;

#[tauri::command]
pub async fn export_data(
    filetype: String,
    host: String,
    user: String,
    password: String,
    port: u16,
    database: String,
    table: String,
) -> Result<(), Error> {
    let opts: OptsBuilder = OptsBuilder::new()
        .user(Some(user))
        .ip_or_hostname(Some(host))
        .pass(Some(password))
        .tcp_port(port)
        .db_name(Some(database.clone()));

    let pool: Pool = Pool::new(opts).expect("Pool could not be created");

    let mut connection = pool.get_conn().expect("Connection failed");

    connection.query_drop(format!("USE {};", database.clone()))?;

    let query_result: Vec<Row> = connection.query(format!("SELECT * FROM {}", table))?;

    let mut rows: Vec<serde_json::Value> = Vec::new();

    for row in query_result {
        let mut row_map = serde_json::Map::new();
        let columns = row.columns_ref();
        for (col, val) in columns.iter().zip(row.unwrap()) {
            let column_name = col.name_str();

            // Handle mysql::Value types
            let value_str = match val {
                mysql::Value::Bytes(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                mysql::Value::Int(int) => int.to_string(),
                mysql::Value::UInt(uint) => uint.to_string(),
                mysql::Value::Float(float) => float.to_string(),
                mysql::Value::Double(double) => double.to_string(),
                mysql::Value::Date(year, month, day, hour, minute, second, micro) => {
                    format!(
                        "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                        year, month, day, hour, minute, second, micro
                    )
                }
                mysql::Value::NULL => "NULL".to_string(),
                mysql::Value::Time(hour, minute, second, microsecond, _is_negative, _is_zero) => {
                    format!("{:02}:{:02}:{:02}.{:06}", hour, minute, second, microsecond)
                }
            };

            row_map.insert(
                column_name.to_string(),
                serde_json::Value::String(value_str),
            );
        }
        rows.push(serde_json::Value::Object(row_map));
    }

    // Serialize the rows to JSON
    let json_data = serde_json::to_string(&rows)?;

    // Write the serialized JSON to a file
    let filename = format!("{}.json", table); // For example, "table_name.json"
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

#[tauri::command]
pub async fn import_data() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    host: String,
    user: String,
    password: String,
    port: u16,
    database: String,
) -> Result<bool, bool> {
    let opts: OptsBuilder = OptsBuilder::new()
        .user(Some(user))
        .ip_or_hostname(Some(host))
        .pass(Some(password))
        .tcp_port(port)
        .db_name(Some(database.clone()));

    let pool: Pool = Pool::new(opts).expect("Pool could not be created");

    let mut connection = pool.get_conn().expect("Connection failed");

    let result: std::result::Result<(), Error> =
        connection.query_drop(format!("USE {};", database.clone()));

    match result {
        Ok(v) => Ok(true),
        Err(e) => Err(false),
    }
}

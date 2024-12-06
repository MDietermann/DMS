use rusqlite::{Connection, Result, params};

#[derive(Debug, serde::Serialize)]
pub struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    position: String,
    password: String
}

impl Employee {
    pub fn check_password(&self, passwd: String) -> bool {
        if self.password == passwd {
            return true;
        }
        false
    }

    pub fn get_invalid_user() -> Employee {
        Employee {
            id: -1,
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            position: "".to_string(),
            password: "".to_string()
        }
    }

    pub async fn get_employee_by_id(employee_id: i32) -> Result<Employee> {
        let connection = Connection::open("dms.db")?;
        let mut query = connection.prepare("SELECT * FROM employee WHERE id = ?1;")?;
        let mut person_iter = query.query_map(params![employee_id], |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?
            })
        })?;

        let person = person_iter.next().unwrap();
        return person;
    }

    async fn add_employee(&self) -> Result<()> {
        let connection = Connection::open("dms.db")?;
        connection.execute(
            "INSERT INTO employee (first_name, last_name, email, position, password)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.first_name, self.last_name, self.email, self.position, self.password]
        )?;

        Ok(())
    }
}

pub async fn create() -> Result<()> {
    let connection = Connection::open("dms.db")?;
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS employee
        (
            id          INTEGER  PRIMARY KEY NOT NULL,
            first_name   TEXT                 NOT NULL,
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

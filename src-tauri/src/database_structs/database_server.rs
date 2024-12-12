use super::database_user::DatabaseUser;
use super::table_user::TableUser;

pub struct DatabaseServer{
    ip: String,
    users: DatabaseUser,
    admin_user: String,
    admin_password: String
}

impl DatabaseServer {
    pub fn new(ip: String, users: DatabaseUser, admin_user: String, admin_password: String) -> DatabaseServer {
        DatabaseServer { ip, users, admin_user, admin_password }
    }

    pub fn get_ip(&self) -> &String { &self.ip }

    pub fn get_users(&self) -> &DatabaseUser { &self.users }

    pub fn get_admin_user(&self) -> &String { &self.admin_user }

    pub fn get_admin_password(&self) -> &String { &self.admin_password }

    pub fn add_user(&mut self, users: Vec<(String, TableUser)>) {
        for user in users {
            self.users.insert(user.0, user.1);
        }
    }
}

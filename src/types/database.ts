export interface UserInfo {
    password: string;
    tables: string[];
}

export type DatabaseType = ("mysql" | "postgresql" | "sqlserver" | "mariadb" | "sqlite")

export interface ServerInfo {
    ip: string;
    users: Record<string, UserInfo>;
}

export interface DatabaseStructure {
    database_type: DatabaseType;
    servers: ServerInfo[];
}

export const tsStructure: DatabaseStructure[] = [
    {
        database_type: "mysql",
        servers: [
            {
                ip: "192.168.0.1",
                users: {
                    "root1": { password: "rootpass1", tables: ["shop1", "shared1", "timetable1"] },
                    "developer1": { password: "devpass1", tables: ["shop1", "shared1"] },
                    "user1": { password: "userpass1", tables: ["shared1"] }
                }
            },
            {
                ip: "192.168.0.2",
                users: {
                    "root2": { password: "rootpass2", tables: ["shop2", "shared2", "timetable2"] },
                    "developer2": { password: "devpass2", tables: ["shop2", "shared2"] },
                    "user2": { password: "userpass2", tables: ["shared2"] }
                }
            }
        ]
    },
    {
        database_type: "mariadb",
        servers: [
            {
                ip: "192.168.178.1",
                users: {
                    "root1": { password: "rootpass1", tables: ["shop1", "shared1", "timetable1"] },
                    "developer1": { password: "devpass1", tables: ["shop1", "shared1"] },
                    "user1": { password: "userpass1", tables: ["shared1"] }
                }
            },
            {
                ip: "192.168.0.2",
                users: {
                    "root2": { password: "rootpass2", tables: ["shop2", "shared2", "timetable2"] },
                    "developer2": { password: "devpass2", tables: ["shop2", "shared2"] },
                    "user2": { password: "userpass2", tables: ["shared2"] }
                }
            }
        ]
    },
    {
        database_type: "postgresql",
        servers: [
            {
                ip: "192.168.0.1",
                users: {
                    "root1": { password: "rootpass1", tables: ["shop1", "shared1", "timetable1"] },
                    "developer1": { password: "devpass1", tables: ["shop1", "shared1"] },
                    "user1": { password: "userpass1", tables: ["shared1"] }
                }
            },
            {
                ip: "192.168.128.2",
                users: {
                    "root2": { password: "rootpass2", tables: ["shop2", "shared2", "timetable2"] },
                    "developer2": { password: "devpass2", tables: ["shop2", "shared2"] },
                    "user2": { password: "userpass2", tables: ["shared2"] }
                }
            }
        ]
    },
    {
        database_type: "sqlserver",
        servers: [
            {
                ip: "192.168.128.23",
                users: {
                    "root1": { password: "rootpass1", tables: ["shop1", "shared1", "timetable1"] },
                    "developer1": { password: "devpass1", tables: ["shop1", "shared1"] },
                    "user1": { password: "userpass1", tables: ["shared1"] }
                }
            },
            {
                ip: "192.168.178.22",
                users: {
                    "root2": { password: "rootpass2", tables: ["shop2", "shared2", "timetable2"] },
                    "developer2": { password: "devpass2", tables: ["shop2", "shared2"] },
                    "user2": { password: "userpass2", tables: ["shared2"] }
                }
            }
        ]
    }
];

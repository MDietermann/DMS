export interface TableUser {
    tables: string[];
    password: string;
    isPasswordVisible?: boolean;
}

export interface DatabaseUsers {
    [key: string]: TableUser;
}

export interface DatabaseServer {
    ip: string;
    users: DatabaseUsers;
    adminUser?: string;
    adminPassword?: string;
}

export type DatabaseType = 'MySQL' | 'PostgreSQL' | 'MariaDB' | 'SQL-Server';

export interface DatabaseGroup {
    type: DatabaseType;
    servers: DatabaseServer[];
}

export interface DatabaseStructure {
    databases: DatabaseGroup[];
}

export const dbMock: DatabaseStructure = {
    databases: [
        {
            type: 'MySQL',
            servers: [
                {
                    ip: "192.168.0.1",
                    adminUser: "root1",
                    adminPassword: "rootpass1",
                    users: {
                        "root1": { tables: ["shop1", "shared1", "timetable1"], password: "rootpass1" },
                        "developer1": { tables: ["shop1", "shared1"], password: "devpass1" },
                        "user1": { tables: ["shared1"], password: "userpass1" }
                    },
                },
                {
                    ip: "192.168.0.2",
                    adminUser: "root2",
                    adminPassword: "rootpass2",
                    users: {
                        "root2": { tables: ["shop2", "shared2", "timetable2"], password: "rootpass2" },
                        "developer2": { tables: ["shop2", "shared2"], password: "devpass2" },
                        "user2": { tables: ["shared2"], password: "userpass2" }
                    },
                }
            ]
        }
    ]
};

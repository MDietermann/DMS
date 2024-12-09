export const dbMock: Object = {
    "mysql": [
        {
            "ip": "192.168.0.1",
            "users": {
                "root1": { "tables": ["shop1", "shared1", "timetable1"], "password": "rootpass1" },
                "developer1": { "tables": ["shop1", "shared1"], "password": "devpass1" },
                "user1": { "tables": ["shared1"], "password": "userpass1" }
            },
        },
        {
            "ip": "192.168.0.2",
            "users": {
                "root2": { "tables": ["shop2", "shared2", "timetable2"], "password": "rootpass2" },
                "developer2": { "tables": ["shop2", "shared2"], "password": "devpass2" },
                "user2": { "tables": ["shared2"], "password": "userpass2" }
            },
        }
    ]
}

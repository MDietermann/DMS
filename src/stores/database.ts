import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import Fuse from 'fuse.js';
import type { DatabaseStructure, DatabaseGroup, DatabaseServer, DatabaseType, TableUser } from '../types/database';
import { dbMock } from '../types/database';

export const useDatabaseStore = defineStore('database', () => {
    const databases = ref<DatabaseStructure>(dbMock);
    const searchQuery = ref('');
    const searchCategory = ref<'type' | 'ip' | 'user' | 'table'>('type');

    const fuseOptions = {
        includeScore: true,
        keys: ['type', 'servers.ip', 'servers.users.*.tables', 'servers.users'],
        threshold: 0.4,
    };

    const fuse = computed(() => new Fuse(databases.value.databases, fuseOptions));

    const searchResults = computed(() => {
        if (!searchQuery.value) return databases.value.databases;

        const results = fuse.value.search(searchQuery.value);
        return results.map(result => result.item);
    });

    function getServer(databaseType: string, ip: string): DatabaseServer | undefined {
        const database = databases.value.databases.find(db => db.type === databaseType);
        return database?.servers.find(server => server.ip === ip);
    }

    function addDatabase(type: DatabaseType) {
        databases.value.databases.push({
            type,
            servers: []
        });
    }

    function addServer(databaseType: DatabaseType, server: DatabaseServer) {
        const database = databases.value.databases.find(db => db.type === databaseType);
        if (database) {
            database.servers.push(server);
        } else {
            addDatabase(databaseType);
            addServer(databaseType, server);
        }
    }

    function deleteServer(databaseType: DatabaseType, ip: string) {
        const database = databases.value.databases.find(db => db.type === databaseType);
        if (database) {
            database.servers = database.servers.filter(server => server.ip !== ip);
        }
    }

    function updateServer(databaseType: DatabaseType, ip: string, updatedServer: DatabaseServer) {
        const database = databases.value.databases.find(db => db.type === databaseType);
        if (database) {
            const index = database.servers.findIndex(server => server.ip === ip);
            if (index !== -1) {
                database.servers[index] = updatedServer;
            }
        }
    }

    function deleteUser(databaseType: DatabaseType, ip: string, username: string) {
        const database = databases.value.databases.find(db => db.type === databaseType);
        if (database) {
            const server = database.servers.find(s => s.ip === ip);
            if (server && username !== server.adminUser) {
                const { [username]: deletedUser, ...remainingUsers } = server.users;
                server.users = remainingUsers;
            }
        }
    }

    function updateUser(
        databaseType: DatabaseType,
        ip: string,
        username: string,
        updatedUser: TableUser
    ) {
        const database = databases.value.databases.find(db => db.type === databaseType);
        if (database) {
            const server = database.servers.find(s => s.ip === ip);
            if (server) {
                server.users[username] = updatedUser;
            }
        }
    }

    return {
        databases,
        searchQuery,
        searchCategory,
        searchResults,
        getServer,
        addDatabase,
        addServer,
        deleteServer,
        updateServer,
        deleteUser,
        updateUser,
    };
});


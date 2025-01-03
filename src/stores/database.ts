import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import Fuse from 'fuse.js';
import type { DatabaseStructure, ServerInfo, UserInfo, DatabaseType } from '../types/database';
import { invoke } from '@tauri-apps/api/core';

export const useDatabaseStore = defineStore('database', () => {
    const databases = ref<DatabaseStructure[]>([]);
    const searchQuery = ref('');
    const searchCategory = ref<'type' | 'ip' | 'user' | 'table'>('type');
    const isLoading = ref(false);

    // Für die Suche nach Datenbanken wird Fuse.js verwendet
    const fuseOptions = {
        includeScore: true,
        keys: ['database_type', 'servers.ip', 'servers.users.*.tables', 'servers.users'],
        threshold: 0.4,
    };

    const fuse = computed(() => {
        if (databases.value) {
            return new Fuse(databases.value, fuseOptions);
        }
        return null;
    });

    // Ergebnisse der Suche
    const searchResults = computed(() => {
        if (!searchQuery.value || !fuse.value) return databases.value || [];

        const results = fuse.value.search(searchQuery.value);
        return results.map(result => result.item);
    });

    // Daten vom Backend fetchen
    async function fetchDatabaseData() {
        isLoading.value = true;
        try {
            const data: DatabaseStructure[] = await invoke('get_database_data');
            databases.value = data;
        } catch (error) {
            console.error('Failed to fetch database data:', error);
        } finally {
            isLoading.value = false;
        }
    }

    // Neue Daten ans Backend senden
    async function addDatabaseData(newData: DatabaseStructure[]) {
        isLoading.value = true;
        try {
            await invoke('add_to_database', { structure: newData });
            await fetchDatabaseData(); // Refresh the data after adding
        } catch (error) {
            console.error('Failed to add database data:', error);
        } finally {
            isLoading.value = false;
        }
    }

    // Lokale Hilfsfunktionen
    function getServer(databaseType: DatabaseType, ip: string): ServerInfo | undefined {
        const database = databases.value.find(db => db.database_type === databaseType);
        return database?.servers.find(server => server.ip === ip);
    }

    // Hinzufügen einer neuen Datenbank
    function addDatabase(database_type: DatabaseType) {
        databases.value.push({
            database_type,
            servers: [],
        });

    }

    // Hinzufügen eines neuen Servers
    function addServer(databaseType: DatabaseType, server: ServerInfo) {
        const database = databases.value.find(db => db.database_type === databaseType);
        if (database) {
            database.servers.push(server);
        } else {
            addDatabase(databaseType);
            addServer(databaseType, server);
        }
    }

    // Löschen eines Servers
    function deleteServer(databaseType: DatabaseType, ip: string) {
        const database = databases.value.find(db => db.database_type === databaseType);
        if (database) {
            database.servers = database.servers.filter(server => server.ip !== ip);
        }
    }

    // Aktualisieren eines Servers
    function updateServer(databaseType: DatabaseType, ip: string, updatedServer: ServerInfo) {
        const database = databases.value.find(db => db.database_type === databaseType);
        if (database) {
            const index = database.servers.findIndex(server => server.ip === ip);
            if (index !== -1) {
                database.servers[index] = updatedServer;
            }
        }
    }

    // Löschen eines Benutzers
    function deleteUser(databaseType: DatabaseType, ip: string, username: string) {
        const database = databases.value.find(db => db.database_type === databaseType);
        if (database) {
            const server = database.servers.find(s => s.ip === ip);
            if (server) {
                const { [username]: deletedUser, ...remainingUsers } = server.users;
                server.users = remainingUsers;
            }
        }
    }

    // Aktualisieren eines Benutzers
    function updateUser(
        databaseType: DatabaseType,
        ip: string,
        username: string,
        updatedUser: UserInfo
    ) {
        const database = databases.value.find(db => db.database_type === databaseType);
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
        isLoading,
        fetchDatabaseData,
        addDatabaseData,
        getServer,
        addDatabase,
        addServer,
        deleteServer,
        updateServer,
        deleteUser,
        updateUser,
    };
});

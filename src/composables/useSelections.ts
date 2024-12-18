import { ref, computed } from 'vue';
import { useDatabaseStore } from '../stores/database';
import type { DatabaseType } from '../types/database';

/**
 * Provides a composable function for managing selections for exporting a database table.
 * The returned object contains reactive refs to the selected database type, server IP, username, table name, and
 * export type.
 * It also provides computed properties for the available servers, users, tables, and whether the selection is valid.
 */
export function useSelections() {
    const databaseStore = useDatabaseStore();
    /**
     * The selected database type.
     */
    const selectedDatabaseType = ref<DatabaseType>();
    /**
     * The selected server IP.
     */
    const selectedServerIp = ref<string>('');
    /**
     * The selected username.
     */
    const selectedUsername = ref<string>('');
    /**
     * The selected table name.
     */
    const selectedTableName = ref<string>('');
    /**
     * The type of the export. One of 'json' or 'csv'.
     */
    const exportType = ref<'json' | 'csv'>('json');

    /**
     * The available servers for the selected database type.
     */
    const availableServers = computed(() => {
        const database = databaseStore.databases.find(db => db.database_type === selectedDatabaseType.value);
        return database?.servers ?? [];
    });

    /**
     * The available users for the selected server.
     */
    const availableUsers = computed(() => {
        const server = availableServers.value.find(s => s.ip === selectedServerIp.value);
        return server ? Object.keys(server.users) : [];
    });

    /**
     * The available tables for the selected user.
     */
    const availableTables = computed(() => {
        const server = availableServers.value.find(s => s.ip === selectedServerIp.value);
        return server?.users[selectedUsername.value]?.tables ?? [];
    });

    /**
     * Whether the selection is valid.
     */
    const isValid = computed(() =>
        selectedDatabaseType.value &&
        selectedServerIp.value &&
        selectedUsername.value &&
        selectedTableName.value
    );

    return {
        selectedDatabaseType,
        selectedServerIp,
        selectedUsername,
        selectedTableName,
        exportType,
        availableServers,
        availableUsers,
        availableTables,
        isValid,
    };
}

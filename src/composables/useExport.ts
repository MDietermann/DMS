import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

/**
 * Provides a composable function for exporting a database table.
 */
export const useExport = () => {
    /**
     * Indicates whether the export is currently in progress.
     */
    const isExporting = ref(false);

    /**
     * Export a database table.
     *
     * @param databaseType The type of the database.
     * @param serverIp The IP of the server the database is running on.
     * @param username The username to use for the export.
     * @param table The name of the table to export.
     * @param exportType The type of the export. One of 'json' or 'csv'.
     */
    async function exportTable({
        databaseType,
        serverIp,
        username,
        table,
        exportType,
    }: {
        databaseType: string | undefined;
        serverIp: string;
        username: string;
        table: string;
        exportType: 'json' | 'csv';
    }) {
        isExporting.value = true;
        try {
            await invoke('export_database_table', {
                database_type: databaseType,
                server_ip: serverIp,
                username,
                table,
                export_type: exportType,
            });
        } finally {
            isExporting.value = false;
        }
    }

    return {
        /**
         * Indicates whether the export is currently in progress.
         */
        isExporting,
        /**
         * Export a database table.
         */
        exportTable,
    };
}

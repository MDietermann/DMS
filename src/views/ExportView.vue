<script setup lang="ts">
import { useDatabaseStore } from '../stores/database';
import { useExport } from '../composables/useExport';
import { useSelections } from '../composables/useSelections';
import SelectField from '../components/ui/SelectionField.vue';
import ExportButton from '../components/ui/CButton.vue';
import Card from '../components/ui/Card.vue';

const databaseStore = useDatabaseStore();
const { isExporting, exportTable } = useExport();
const {
    selectedDatabaseType,
    selectedServerIp,
    selectedUsername,
    selectedTableName,
    exportType,
    availableServers,
    availableUsers,
    availableTables,
    isValid,
} = useSelections();

/**
 * Handles the database export process.
 *
 * @param {Object} exportOptions - Export options
 * @param {string} exportOptions.databaseType - Type of database
 * @param {string} exportOptions.serverIp - IP address of the server
 * @param {string} exportOptions.username - Username of the user
 * @param {string} exportOptions.table - Table name to export
 * @param {string} exportOptions.exportType - Export format
 */
async function handleExport() {
    if (!isValid.value) return;

    try {
        // Call the exportData function with the export options
        await exportTable({
            databaseType: selectedDatabaseType.value,
            serverIp: selectedServerIp.value,
            username: selectedUsername.value,
            table: selectedTableName.value,
            exportType: exportType.value,
        });
    } catch (error) {
        // Log any errors that occur during the export process
        console.error('Export failed:', error);
    }
}
</script>

<template>
    <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
        <Card>
            <template #header>
                <h2 class="text-2xl font-bold text-gray-900">Database Export</h2>
                <p class="mt-1 text-sm text-gray-500">
                    Select your database configuration and export format
                </p>
            </template>

            <div class="space-y-6">
                <SelectField
                    label="Database Type"
                    v-model="selectedDatabaseType"
                    :options="databaseStore.databases.map(db => ({
                        value: db.database_type,
                        label: db.database_type.toUpperCase()
                    }))"
                    placeholder="Select Database"
                />

                <SelectField
                    label="Server"
                    v-model="selectedServerIp"
                    :options="availableServers.map(server => ({
                        value: server.ip,
                        label: server.ip
                    }))"
                    :disabled="!selectedDatabaseType"
                    placeholder="Select Server"
                />

                <SelectField
                    label="User"
                    v-model="selectedUsername"
                    :options="availableUsers.map(user => ({
                        value: user,
                        label: user
                    }))"
                    :disabled="!selectedServerIp"
                    placeholder="Select User"
                />

                <SelectField
                    label="Table"
                    v-model="selectedTableName"
                    :options="availableTables.map(table => ({
                        value: table,
                        label: table
                    }))"
                    :disabled="!selectedUsername"
                    placeholder="Select Table"
                />

                <SelectField
                    label="Export Format"
                    v-model="exportType"
                    :options="[
                        { value: 'json', label: 'JSON' },
                        { value: 'csv', label: 'CSV' }
                    ]"
                />

                <ExportButton
                    :disabled="!isValid"
                    :loading="isExporting"
                    @click="handleExport"
                />
            </div>
        </Card>
    </div>
</template>

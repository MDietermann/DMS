<script setup lang="ts">
import { useDatabaseStore } from '../stores/database';
import { useExport } from '../composables/useExport';
import { useSelections } from '../composables/useSelections';
import SelectField from '../components/ui/SelectionField.vue';
import ExportButton from '../components/ui/CButton.vue';
import Card from '../components/ui/Card.vue';

const databaseStore = useDatabaseStore();
const { isExporting, exportData } = useExport();
const {
    selectedDatabase,
    selectedServer,
    selectedUser,
    selectedTable,
    exportType,
    availableServers,
    availableUsers,
    availableTables,
    isValid,
} = useSelections();

async function handleExport() {
    if (!isValid.value) return;

    try {
        await exportData({
            databaseType: selectedDatabase.value,
            serverIp: selectedServer.value,
            username: selectedUser.value,
            table: selectedTable.value,
            exportType: exportType.value,
        });
    } catch (error) {
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
                    v-model="selectedDatabase"
                    :options="databaseStore.databases.map(db => ({
                        value: db.database_type,
                        label: db.database_type.toUpperCase()
                    }))"
                    placeholder="Select Database"
                />

                <SelectField
                    label="Server"
                    v-model="selectedServer"
                    :options="availableServers.map(server => ({
                        value: server.ip,
                        label: server.ip
                    }))"
                    :disabled="!selectedDatabase"
                    placeholder="Select Server"
                />

                <SelectField
                    label="User"
                    v-model="selectedUser"
                    :options="availableUsers.map(user => ({
                        value: user,
                        label: user
                    }))"
                    :disabled="!selectedServer"
                    placeholder="Select User"
                />

                <SelectField
                    label="Table"
                    v-model="selectedTable"
                    :options="availableTables.map(table => ({
                        value: table,
                        label: table
                    }))"
                    :disabled="!selectedUser"
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

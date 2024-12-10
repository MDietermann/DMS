<script setup lang="ts">
import { ref } from 'vue';
import { useDatabaseStore } from '../../stores/database';
import type { DatabaseType, DatabaseServer } from '../../types/database';
import { testDatabaseConnection, validateIpAddress } from '../../utils/database';

const store = useDatabaseStore();
const showAddModal = ref(false);
const newServer = ref<DatabaseServer>({
    ip: '',
    users: {},
    adminUser: '',
    adminPassword: '',
});
const selectedType = ref<DatabaseType>('MySQL');
const testingConnection = ref(false);
const connectionError = ref('');

const databaseTypes: DatabaseType[] = ['MySQL', 'PostgreSQL', 'MariaDB', 'SQL-Server'];

async function testConnection() {
    if (!validateIpAddress(newServer.value.ip)) {
        connectionError.value = 'Invalid IP address format';
        return;
    }

    if (!newServer.value.adminUser || !newServer.value.adminPassword) {
        connectionError.value = 'Admin credentials are required';
        return;
    }

    testingConnection.value = true;
    connectionError.value = '';

    try {
        const success = await testDatabaseConnection(
            selectedType.value,
            newServer.value.ip,
            newServer.value.adminUser,
            newServer.value.adminPassword
        );

        if (success) {
            // Add the admin user to the users object
            newServer.value.users[newServer.value.adminUser] = {
                password: newServer.value.adminPassword,
                tables: ['*'],
            };

            store.addServer(selectedType.value, { ...newServer.value });
            showAddModal.value = false;
            newServer.value = { ip: newServer.value.ip, users: { [newServer.value.adminUser]: { tables: ['*'] } }, adminUser: newServer.value.adminUser, adminPassword: newServer.value.adminPassword };
        } else {
            connectionError.value = 'Connection failed. Please check your credentials.';
        }
    } catch (error) {
        connectionError.value = 'Connection error. Please try again.';
    } finally {
        testingConnection.value = false;
    }
}
</script>

<template>
    <div class="mb-6">
        <button class="btn btn-primary" @click="showAddModal = true">
            <i class="bi bi-plus-circle me-2"></i>Add Server
        </button>

        <!-- Add Server Modal -->
        <div v-if="showAddModal" class="modal fade show d-block" tabindex="-1">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">Add New Server</h5>
                        <button type="button" class="btn-close" @click="showAddModal = false" />
                    </div>
                    <div class="modal-body">
                        <div class="mb-3">
                            <label class="form-label">Database Type</label>
                            <select v-model="selectedType" class="form-select">
                                <option v-for="type in databaseTypes" :key="type" :value="type">
                                    {{ type }}
                                </option>
                            </select>
                        </div>
                        <div class="mb-3">
                            <label class="form-label">IP Address</label>
                            <input v-model="newServer.ip" type="text" class="form-control" placeholder="192.168.0.1" />
                        </div>
                        <div class="mb-3">
                            <label class="form-label">Admin Username</label>
                            <input v-model="newServer.adminUser" type="text" class="form-control" placeholder="root" />
                        </div>
                        <div class="mb-3">
                            <label class="form-label">Admin Password</label>
                            <input v-model="newServer.adminPassword" type="password" class="form-control"
                                placeholder="Enter admin password" />
                        </div>
                        <div v-if="connectionError" class="alert alert-danger">
                            {{ connectionError }}
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" @click="showAddModal = false">
                            Cancel
                        </button>
                        <button type="button" class="btn btn-primary" :disabled="testingConnection"
                            @click="testConnection">
                            <span v-if="testingConnection" class="spinner-border spinner-border-sm me-2" />
                            {{ testingConnection ? 'Testing Connection...' : 'Add Server' }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

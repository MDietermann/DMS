<script setup lang="ts">
import { ref } from 'vue';
import type { DatabaseServer } from '../../types/database';
import { useDatabaseStore } from '../../stores/database';
import UserCard from './UserCard.vue';
import ServerEditModal from './modals/ServerEditModal.vue';

const props = defineProps<{
    server: DatabaseServer;
    databaseType: string;
}>();

const store = useDatabaseStore();
const showEditModal = ref(false);

function handleDelete() {
    if (confirm('Are you sure you want to delete this server?')) {
        store.deleteServer(props.databaseType as any, props.server.ip);
    }
}
</script>

<template>
    <div class="bg-gray-50 rounded-lg p-6">
        <div class="flex justify-between items-center mb-4">
            <h2 class="text-xl font-semibold text-gray-800">
                <i class="bi bi-hdd-network me-2"></i>Server: {{ server.ip }}
            </h2>
            <div class="flex gap-2">
                <button class="btn btn-outline-primary btn-sm" @click="showEditModal = true">
                    <i class="bi bi-pencil me-1"></i>Edit
                </button>
                <button class="btn btn-outline-danger btn-sm" @click="handleDelete">
                    <i class="bi bi-trash me-1"></i>Delete
                </button>
            </div>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <UserCard v-for="(userData, username) in server.users" :key="username" :username="username"
                :userData="userData" :database-type="databaseType" :server-ip="server.ip" />
        </div>

        <ServerEditModal v-if="showEditModal" :server="server" :database-type="databaseType"
            @close="showEditModal = false" />
    </div>
</template>

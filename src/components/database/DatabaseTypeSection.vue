<script setup lang="ts">
import type { DatabaseStructure } from '../../types/database';
import ServerCard from './ServerCard.vue';

defineProps<{
    databaseStructure: DatabaseStructure;
}>();
</script>

<template>
    <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center gap-4 mb-6">
            <h2 class="text-2xl font-semibold text-gray-800 m-0">{{ databaseStructure.database_type }}</h2>
            <span class="badge bg-primary">{{ databaseStructure.servers.length }} servers</span>
        </div>
        <div class="space-y-6">
            <ServerCard v-for="server in databaseStructure.servers" :key="server.ip" :server="server"
                :database-type="databaseStructure.database_type" />
        </div>
    </div>
</template>

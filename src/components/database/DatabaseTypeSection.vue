<script setup lang="ts">
import type { DatabaseGroup } from '../../types/database';
import ServerCard from './ServerCard.vue';

defineProps<{
    databaseGroup: DatabaseGroup;
}>();
</script>

<template>
    <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center gap-4 mb-6">
            <h2 class="text-2xl font-semibold text-gray-800 m-0">{{ databaseGroup.type }}</h2>
            <span class="badge bg-primary">{{ databaseGroup.servers.length }} servers</span>
        </div>
        <div class="space-y-6">
            <ServerCard v-for="server in databaseGroup.servers" :key="server.ip" :server="server"
                :database-type="databaseGroup.type" />
        </div>
    </div>
</template>

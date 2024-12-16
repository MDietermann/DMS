<script setup lang="ts">
import DatabaseTypeSection from './DatabaseTypeSection.vue';
import { useDatabaseStore } from '../../stores/database';
import DatabaseActions from './DatabaseActions.vue';
import SearchBar from '../search/SearchBar.vue';

const store = useDatabaseStore();
store.fetchDatabaseData();
console.log(store.searchResults)
</script>

<template>
    <div class="container py-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-8">Database Management System</h1>
        <SearchBar />
        <DatabaseActions />
        <div class="space-y-8">
            <DatabaseTypeSection v-for="dbGroup in store.searchResults" :key="dbGroup.database_type" :database-structure="dbGroup" />
        </div>
        <button class="btn btn-primary" @click="store.fetchDatabaseData">Refresh</button>
    </div>
</template>

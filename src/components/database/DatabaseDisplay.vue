<script setup lang="ts">
import DatabaseTypeSection from './DatabaseTypeSection.vue';
import { useDatabaseStore } from '../../stores/database';
import DatabaseActions from './DatabaseActions.vue';
import SearchBar from '../search/SearchBar.vue';
import { tsStructure } from '../../mock/db-mock';
import { invoke } from '@tauri-apps/api/core';

const store = useDatabaseStore();
store.fetchDatabaseData();
</script>

<template>
    <div class="container py-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-8">Database Management System</h1>
        <SearchBar />
        <DatabaseActions />
        <div class="space-y-8">
            <DatabaseTypeSection v-for="dbGroup in store.searchResults" :key="dbGroup.type" :database-group="dbGroup" />
        </div>
        <button class="btn btn-primary" @click="store.fetchDatabaseData">Refresh</button>
        <button class="btn btn-primary" @click="store.addDatabaseData(tsStructure)">Import Mock</button>
        <button class="btn btn-primary" @click="invoke('create_database')">Generate Databases</button>
    </div>
</template>

<script setup lang="ts">
import { useDatabaseStore } from '../../stores/database';
import DatabaseTypeSection from './DatabaseTypeSection.vue';
import SearchBar from '../search/SearchBar.vue';
import DatabaseActions from './DatabaseActions.vue';

const store = useDatabaseStore();
</script>

<template>
    <div class="container py-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-8">Database Management System</h1>
        <SearchBar />
        <DatabaseActions />
        <div class="space-y-8">
            <DatabaseTypeSection v-for="dbGroup in store.searchResults" :key="dbGroup.type" :database-group="dbGroup" />
        </div>
    </div>
</template>

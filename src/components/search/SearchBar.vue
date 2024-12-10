<script setup lang="ts">
import { useDatabaseStore } from '../../stores/database';

const store = useDatabaseStore();
</script>

<template>
    <div class="mb-6">
        <div class="flex gap-4">
            <div class="flex-grow">
                <input v-model="store.searchQuery" type="text" class="form-control" placeholder="Search databases..." />
            </div>
            <select v-model="store.searchCategory" class="form-select w-48">
                <option value="type">Database Type</option>
                <option value="ip">IP Address</option>
                <option value="user">User</option>
                <option value="table">Table</option>
            </select>
        </div>
    </div>
</template>

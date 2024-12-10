<script setup lang="ts">
defineProps<{
    tables: string[];
}>();
</script>

<template>
    <div class="flex flex-wrap gap-2">
        <span v-for="table in tables" :key="table" class="badge bg-secondary">
            <i class="bi bi-table me-1"></i>{{ table }}
        </span>
    </div>
</template>

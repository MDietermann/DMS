<script setup lang="ts">
import { resolveDbName } from '../../scripts/dbResolver';
import DatabaseTable from './database-table/DatabaseTable.vue';
defineProps({
    databases: {
        type: Object,
        required: true
    },
})
</script>

<template>
    <div class="accordion container-sm" id="accordionFlush">
        <template v-for="(db, name) in databases">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse"
                    :data-bs-target="'#' + resolveDbName(name)" aria-expanded="false"
                    :aria-controls="resolveDbName(name)">
                    <p class="display-6">{{ resolveDbName(name) }}</p>
                </button>
            </h2>
            <div :id="resolveDbName(name)" class="accordion-collapse collapse" data-bs-parent="#accordionFlush">
                <div class="accordion-body">
                    <DatabaseTable :databaseData="db" />
                </div>
            </div>
        </template>
    </div>
</template>

<style lang="css" scoped>
</style>

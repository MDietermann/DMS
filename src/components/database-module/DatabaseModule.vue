<script setup lang="ts">
import { resolveDbName } from '../../scripts/dbResolver';
import DatabaseTable from './database-table/DatabaseTable.vue';
const props = defineProps({
    databasesForRow: {
        type: Object,
        required: true
    },
})
</script>

<template>
    <div class="accordion container-sm section" id="accordionFlush">
        <template v-for="(db, name) in databasesForRow">
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
.section {
    box-shadow: rgba(0, 0, 0, 0.15) 0px 5px 15px 0px;
    padding: 8px;
    border-radius: 8px;
    margin-top: 32px;
    margin-bottom: 32px;
}
</style>

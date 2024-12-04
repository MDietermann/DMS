<script setup lang="ts">
import DatabaseSelector from "../database-selector/DatabaseSelector.vue";
import { dbMock } from "../../mock/db-mock";
import { acceptedFileTypesExport } from '../../mock/filetypes.ts'
import Checkbox from "../checkbox/Checkbox.vue";

</script>

<template>
    <div class="text-center">
        <div class="row">
            <div class="col">
                <h2>Export-Auswahl</h2>
                <hr>
                <div class="d-flex justify-content-center flex-column h-50">
                   <Checkbox :data="acceptedFileTypesExport"
                        vertical
                        prefix="Zum Datentyp"
                        suffix="exportieren"/>
                </div>
            </div>
            <div class="col">
                <h2>Datenbank-Auswahl</h2>
                <hr>
                <DatabaseSelector :tableContent="dbMock" />
            </div>
        </div>
    </div>
</template>

<style scoped></style>

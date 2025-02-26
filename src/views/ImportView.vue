<script setup>
import DatabaseForm from '../components/ui/DatabaseForm.vue';
</script>

<template>
    <div class="pt-4 container">
        <div class="w-full h-full border py-4">
            <h2 class="display-6">Datenbankinformationen</h2>
            <DatabaseForm/>
        </div>
        <div class="w-1/2 h-full">
            <input type="file" name="importFile" id="import_file" class="form-control">
        </div>
    </div>
</template>

<style scoped>

</style>
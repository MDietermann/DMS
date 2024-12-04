<script setup>
import Exporter from "../components/exporter/Exporter.vue";
import Button from "../components/button/Button.vue";
</script>

<template>
    <div class="text-center justify-content-center h-100 d-flex flex-column">
        <div class="row">
            <Exporter/>
        </div>
        <div class="row p-3">
            <button class="btn btn-success" type="button">
                Export Starten
            </button>
        </div>
    </div>
</template>

<style scoped>

</style>

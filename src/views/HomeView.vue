<script setup lang="ts">
import DatabaseDisplay from "../components/database/DatabaseDisplay.vue";
import { useEmployeeStore } from "../stores/employee";
import { ref, watch } from "vue";

const employeeStore = useEmployeeStore();

</script>

<template>
    <template>
        <DatabaseDisplay />
    </template>
</template>

<style lang="css" scoped>
.home-body {
    margin: 32px 0;
}
</style>

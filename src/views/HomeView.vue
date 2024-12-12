<script setup lang="ts">
import DatabaseDisplay from "../components/database/DatabaseDisplay.vue";
import { acceptedFileTypesExport, acceptedFileTypesImport } from "../mock/filetypes";
import { resolveUserDataNames } from "../scripts/dbResolver";
import Section from "../components/section/Section.vue";
import { useEmployeeStore } from "../stores/employee";
import Login from "../components/login/Login.vue";
import { dbMock } from "../mock/db-mock";
import { ref, watch } from "vue";

const employeeStore = useEmployeeStore();
const isLoggedIn = ref(false);

watch(
    () => employeeStore.isLoggedIn,
    () => {
        isLoggedIn.value = employeeStore.isLoggedIn;
    }
);

const validDataTypes = (typesArray: string[]) => {
    return typesArray.join(", ");
};
</script>

<template>
    <template v-if="!isLoggedIn">
        <Login />
    </template>
    <template v-else>
        <DatabaseDisplay />
    </template>
</template>

<style lang="css" scoped>
.home-body {
    margin: 32px 0;
}
</style>

<script setup lang="ts">
import DatabaseDisplay from "../components/database/DatabaseDisplay.vue";
import { useEmployeeStore } from "../stores/employee";
import Login from "../components/login/Login.vue";
import { ref, watch } from "vue";

const employeeStore = useEmployeeStore();
const isLoggedIn = ref(employeeStore.isLoggedIn);

watch(
    () => employeeStore.isLoggedIn,
    () => {
        isLoggedIn.value = employeeStore.isLoggedIn;
    }
);
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

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useEmployeeStore } from '../stores/employee'
import Offcanvas from './offcanvas/Offcanvas.vue';
import { RouterLink } from 'vue-router';
import { ref, watch } from 'vue'

const props = defineProps({
    currentRouteName: {
        type: String,
        required: true
    }
})

const employeeStore = useEmployeeStore()
const { loggedIn } = storeToRefs(employeeStore)
watch(() => employeeStore.loggedIn,
    () => {
        console.log('isLoggedIn ref changed, do something!')
    })
</script>

<template>
    <nav class="navbar navbar-expand-lg bg-body-tertiary fixed-top" data-bs-theme="dark">
        <div class="container-fluid">
            <RouterLink class="navbar-brand" to="/">
                DMS-System
            </RouterLink>
            <button v-if="loggedIn" class="btn btn-outline-light me-2" data-bs-toggle="offcanvas"
                data-bs-target="#offcanvasMenu" aria-controls="offcanvasMenu" type="button">
                Menü
            </button>
        </div>
    </nav>
    <Offcanvas :currentRouteName="currentRouteName" />
</template>

<style scoped></style>

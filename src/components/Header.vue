<script setup lang="ts">
import { RouterLink } from 'vue-router';
import Offcanvas from './offcanvas/Offcanvas.vue';

const props = defineProps({
    currentRouteName: {
        type: String,
        required: true
    }
})
</script>

<template>
    <nav class="navbar navbar-expand-lg bg-body-tertiary fixed-top"
        data-bs-theme="dark">
        <div class="container-fluid">
            <RouterLink class="navbar-brand" to="/">
                DMS-System
            </RouterLink>
            <button class="btn btn-outline-light me-2"
                data-bs-toggle="offcanvas"
                data-bs-target="#offcanvasMenu"
                aria-controls="offcanvasMenu"
                type="button">
                Menü
            </button>
        </div>
    </nav>
    <Offcanvas :currentRouteName="currentRouteName"/>
</template>

<style scoped></style>

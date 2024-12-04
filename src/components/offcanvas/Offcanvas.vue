<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { routes } from '../../routes'
import { useEmployeeStore } from '../../stores/employee'

const props = defineProps({
    currentRouteName: {
        type: String,
        required: true
    }
})

const employeeStore = useEmployeeStore()
</script>

<template>
    <div class="offcanvas offcanvas-start bg-body-tertiary" tabindex="-1" id="offcanvasMenu"
        aria-labelledby="offcanvasMenuLabel" data-bs-theme="dark">
        <div class="offcanvas-header">
            <h5 class="offcanvas-title display-5" id="offcanvasMenuLabel">Auswahlmenü</h5>
            <button type="button" class="btn-close" data-bs-dismiss="offcanvas" aria-label="Close"></button>
        </div>
        <hr>
        <div class="offcanvas-body d-flex flex-column justify-content-between">
            <div>
                <div>
                    <p class="lead">Hier finden sich die derzeitig vorhandenen Module an</p>
                </div>
                <hr>
                <ul class="list-group list-group-flush">
                    <RouterLink v-for="route in routes" :to="route.path" :class="[
                        `list-group-item list-group-item-action`,
                        currentRouteName == route.name ? 'active' : ''
                    ]" :aria-current="currentRouteName == route.name" type="button">
                        <li>{{ route.name }}</li>
                    </RouterLink>
                </ul>
            </div>
            <div>
                <hr>
                <div class="btn-group w-100" role="group" aria-label="Basic outlined example">
                    <button type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasMenu" @click="employeeStore.logout" class="btn btn-success">Benutzer wechseln</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped></style>

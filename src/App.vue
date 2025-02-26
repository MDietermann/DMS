<script setup>
import { useEmployeeStore } from './stores/employee'
import Header from "./components/Header.vue";
import { onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const currentRouteName = ref("")

onMounted(() => {
    currentRouteName.value = route.name
})

watch(route, (to) => {
    currentRouteName.value = to.name
})


</script>

<template>
    <Header :currentRouteName="currentRouteName" />
    <div class="spacer" />
    <div class="container text-center">
        <p class="display-3">{{ currentRouteName }}</p>
        <hr>
        <router-view />
        <br>
    </div>
</template>

<style scoped>
.spacer {
    margin-bottom: 60px;
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease-out;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>

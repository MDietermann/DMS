<script setup>
import Header from "./components/Header.vue";
import { useRoute } from 'vue-router'
import { onMounted, ref, watch } from 'vue'

const route = useRoute()
const currentRouteName = ref("")
onMounted(() => {
    currentRouteName.value = route.name
})
watch(route, (to) => {
    currentRouteName.value = to.name
    console.log(currentRouteName.value);

})

</script>

<template>
        <Header :currentRouteName="currentRouteName"/>
        <div class="spacer"/>
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

.fade-enter-active, .fade-leave-active {
    transition: opacity 0.3s ease-out;
}

.fade-enter-from, .fade-leave-to {
    opacity: 0;
}
</style>

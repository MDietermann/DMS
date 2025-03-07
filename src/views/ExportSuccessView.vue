<template>
    <div class="container mx-auto mt-5">
        <div class="alert alert-success" role="alert">
            <h4 class="alert-heading">Export Successful!</h4>
            <p>{{ payload.message }}</p>
            <hr>
            <p class="mb-0">Details: {{ payload }}</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const payload = ref();
onMounted(() => {
    if (route.query.result) {
        payload.value = JSON.parse(route.query.result as string);
    }
});

</script>

<style scoped>
.container {
    max-width: 600px;
}
</style>

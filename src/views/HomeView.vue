<script setup lang="ts">

import { useRouter } from "vue-router";

const router = useRouter();

const reroute = (newRoute: string) => {
    router.push(newRoute);
}

</script>

<template>
     <Button class="btn btn-primary m-4" @click="reroute('/export')">Export Database</Button>
     <Button class="btn btn-primary m-4" @click="reroute('/import')">Import Database</Button>
</template>

<style lang="css" scoped>
    .home-body {
        margin: 32px 0;
    }
</style>

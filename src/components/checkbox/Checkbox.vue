<script setup lang="ts">

const props = defineProps({
    data: { type: Array, required: true },
    prefix: { type: String, default: '' },
    suffix: { type: String, default: '' },
    vertical: { type: Boolean, default: false }
});
</script>

<template>
    <div :class="[vertical ? `btn-group-vertical` : `btn-group`]" role="group" aria-label="Basic checkbox toggle button group">
        <template v-for="singleData in data">
            <input type="checkbox" class="btn-check" :id="`checkbox_${singleData}`" autocomplete="off">
            <label class="btn btn-outline-success" :for="`checkbox_${singleData}`">
                {{ prefix }} {{ singleData }} {{ suffix }}
            </label>
        </template>
    </div>
</template>

<style scoped>
.container {
    padding: 4px;
    margin: 4px;
    cursor: pointer;
    border: 1px solid var(--main-color);
    border-radius: 4px;
    transition: all 0.3s ease-out;
}

.container:hover {
    background: var(--main-color);
    color: white;
}
</style>

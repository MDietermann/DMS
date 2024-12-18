<script setup lang="ts">
defineSlots<{
    header: () => any;
    default: () => any;
}>();
</script>

<template>
    <div class="max-w-2xl mx-auto">
        <div class="bg-white shadow-xl rounded-lg overflow-hidden">
            <div class="px-6 py-5 border-b border-gray-200">
                <slot name="header" />
            </div>
            <div class="px-6 py-5">
                <slot />
            </div>
        </div>
    </div>
</template>

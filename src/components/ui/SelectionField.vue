<script setup lang="ts">
interface Option {
    value: string;
    label: string;
}

interface Props {
    label: string;
    modelValue: string | undefined;
    options: Option[];
    disabled?: boolean;
    placeholder?: string;
}

defineProps<Props>();
defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();
</script>

<template>
    <div class="space-y-1">
        <label class="block text-sm font-medium text-gray-700">
            {{ label }}
        </label>
        <select
            :value="modelValue"
            @input="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
            class="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md transition-colors duration-200"
            :class="{
                'bg-gray-50 cursor-not-allowed': disabled,
                'hover:border-gray-400': !disabled
            }"
            :disabled="disabled"
        >
            <option value="">{{ placeholder || 'Select option' }}</option>
            <option
                v-for="option in options"
                :key="option.value"
                :value="option.value"
            >
                {{ option.label }}
            </option>
        </select>
    </div>
</template>

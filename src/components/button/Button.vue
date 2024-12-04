<script setup lang="ts">
const props = defineProps({
    text: {
        type: String,
        required: true,
    },

    clicked: {
        type: Function,
        default: function () {
            console.log('clicked');
        },
    }
});


</script>

<template>
    <div class="button" :onclick="props.clicked">
        {{ props.text }}
    </div>
</template>

<style scoped>
.button {
    cursor: pointer;
    display: inline-block;
    padding: .5rem 1rem;
    border-radius: 4px;
    border: 1px solid var(--main-color);
    transition: all .3s ease-out;
}

.button:hover {
    background-color: var(--main-color);
    color: #f9f9f9;
}

.button:active {
    background-color: var(--main-color-active);
    transition: background-color 0.1s ease-in-out;
}
</style>
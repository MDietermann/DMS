<template>
  <section class="section">
    <p class="display-5">{{ title }}</p>
    <hr>
    <slot />
  </section>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
  }
})

defineEmits({
  // add your events here
})
</script>

<style scoped>
.section {
    box-shadow: rgba(0, 0, 0, 0.15) 0px 5px 15px 0px;
    padding: 8px;
    border-radius: 8px;
}
</style>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const word = ref("");

const handleClick = () => {
    invoke<string>('return_string', { word: "Triggered" })
    .then((tauriResponse) => {
        word.value = tauriResponse
    }).catch((err) => {
        word.value = err
    });
}

async function greet() {
  word.value = await invoke("return_string", { word: "Triggered" });
}
</script>

<template>
  <div>
    <h1>Word signal is currently: {{word}}</h1>
    <button @click="greet">Trigger the Tauri Action</button>
  </div>
</template>

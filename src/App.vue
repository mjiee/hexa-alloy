<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");

async function greet() {
  greetMsg.value = await invoke("greet");
}
</script>

<template>
  <main>
    <button @click="greet">Greet</button>
    <p>{{ greetMsg }}</p>
  </main>
</template>

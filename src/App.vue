<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const data = ref<string[]>([]);

async function getData() {
  console.log("click");

  data.value = await invoke("get_assets", { data: { page: 1, size: 1 } });
}
</script>

<template>
  <div>Hello World!</div>
  <p></p>
  <button @click="getData">click me</button>
  <p></p>
  <p v-for="(item, index) in data" :key="index">{{ item }}</p>
</template>

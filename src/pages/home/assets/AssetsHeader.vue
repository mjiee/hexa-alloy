<script setup lang="ts">
import { IconArrowLeft, IconFilter, IconSearch, IconPlus } from "@tabler/icons-vue";
</script>

<template>
  <div class="flex justify-between items-center mb-6">
    <div class="flex items-center">
      <router-link to="/" class="mr-3 p-2 rounded-full hover:bg-gray-100">
        <IconArrowLeft class="text-gray-600" />
      </router-link>
      <h1 class="text-2xl font-bold">资产详情</h1>
    </div>
    <div class="flex space-x-3">
      <button class="p-2 rounded-full hover:bg-gray-100">
        <IconSearch class="text-gray-600" />
      </button>
      <button class="p-2 rounded-full hover:bg-gray-100">
        <IconPlus class="text-gray-600" />
      </button>
      <button class="p-2 rounded-full hover:bg-gray-100">
        <IconFilter class="text-gray-600" />
      </button>
    </div>
  </div>
</template>

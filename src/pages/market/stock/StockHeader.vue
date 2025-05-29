<script setup lang="ts">
import { IconStar } from "@tabler/icons-vue";
</script>

<template>
  <div class="flex flex-col md:flex-row md:items-center justify-between mb-6">
    <div>
      <div class="flex items-center">
        <h1 class="text-2xl font-bold">阿里巴巴 (BABA)</h1>
        <button class="ml-3 text-gray-400 hover:text-yellow-500">
          <IconStar />
        </button>
        <span class="ml-3 px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">科技</span>
      </div>
      <p class="text-gray-500 text-sm mt-1">纽约证券交易所</p>
    </div>
    <div class="mt-4 md:mt-0">
      <div class="flex items-baseline">
        <span class="text-3xl font-bold">$87.24</span>
        <span class="ml-2 text-lg stock-down">-2.18 (-2.44%)</span>
      </div>
      <p class="text-gray-500 text-xs">更新于: 2025-03-19 15:30 EST</p>
    </div>
  </div>
</template>

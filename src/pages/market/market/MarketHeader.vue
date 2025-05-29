<script setup lang="ts">
import { IconFilter, IconSearch } from "@tabler/icons-vue";
</script>

<template>
  <div class="bg-white p-4 md:p-6 border-b border-gray-200">
    <div class="flex justify-between items-center">
      <h1 class="text-2xl font-bold md:hidden">财富管家</h1>
      <div class="hidden md:block">
        <h2 class="text-xl font-semibold">股市分析</h2>
        <p class="text-sm text-gray-500">
          2025年3月17日 上证指数: <span class="text-green-500">3726.87 +23.73 (+0.64%)</span>
        </p>
      </div>
      <div class="flex items-center">
        <div class="relative mr-3">
          <input
            type="text"
            placeholder="搜索股票..."
            class="pl-10 pr-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <div class="absolute left-3 top-2.5 text-gray-400">
            <IconSearch />
          </div>
        </div>
        <button class="bg-blue-500 text-white p-2 rounded-lg hover:bg-blue-600 transition">
          <IconFilter />
        </button>
      </div>
    </div>
  </div>
</template>

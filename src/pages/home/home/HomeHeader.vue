<script setup lang="ts">
import { IconSearch } from "@tabler/icons-vue";
</script>

<template>
  <header class="bg-white shadow-sm p-4">
    <div class="flex items-center justify-between">
      <div class="md:hidden flex items-center">
        <h1 class="text-xl font-bold text-blue-600">HEXA ALLOY</h1>
      </div>
      <div class="flex-1 max-w-xl mx-4">
        <div class="relative">
          <input
            type="text"
            placeholder="搜索股票、资产..."
            class="w-full p-2 pl-10 rounded-lg border border-slate-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <IconSearch class="absolute left-3 top-3 text-slate-400" />
        </div>
      </div>
    </div>
  </header>
</template>

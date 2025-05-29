<script setup lang="ts">
import { IconMenu2, IconSearch, IconChecks } from "@tabler/icons-vue";
</script>

<template>
  <header class="bg-white shadow-soft sticky top-0 z-10">
    <div class="px-4 lg:px-6 py-4 flex items-center justify-between">
      <div class="flex items-center">
        <button class="lg:hidden text-neutral-600 mr-2">
          <IconMenu2 />
        </button>
        <h2 class="text-xl font-bold text-neutral-800">消息中心</h2>
      </div>
      <div class="flex items-center">
        <button class="p-2 text-neutral-600 hover:text-primary transition-colors">
          <IconSearch />
        </button>
        <button class="ml-2 p-2 text-neutral-600 hover:text-primary transition-colors">
          <IconChecks />
        </button>
      </div>
    </div>
    <!-- 消息分类标签 -->
    <div class="px-4 lg:px-6 pb-2 flex space-x-1 overflow-x-auto scrollbar-hide">
      <button class="px-4 py-2 text-sm font-medium text-primary border-b-2 border-primary">全部消息</button>
      <button class="px-4 py-2 text-sm font-medium text-neutral-600 hover:text-primary">互动通知</button>
      <button class="px-4 py-2 text-sm font-medium text-neutral-600 hover:text-primary relative">
        市场提醒
        <span class="absolute -top-1 -right-1 h-2 w-2 bg-red-500 rounded-full badge-pulse"></span>
      </button>
      <button class="px-4 py-2 text-sm font-medium text-neutral-600 hover:text-primary">资产变动</button>
      <button class="px-4 py-2 text-sm font-medium text-neutral-600 hover:text-primary">系统通知</button>
    </div>
  </header>
</template>

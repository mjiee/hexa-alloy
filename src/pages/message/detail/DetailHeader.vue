<script setup lang="ts">
import { IconStar, IconMenu2 } from "@tabler/icons-vue";
</script>

<template>
  <header class="bg-white shadow-soft sticky top-0 z-10">
    <div class="px-4 lg:px-6 py-4 flex items-center justify-between">
      <div class="flex items-center">
        <a href="#" class="flex items-center text-neutral-600 transition-colors">
          <i class="fas fa-arrow-left text-lg mr-3"></i>
          <h2 class="text-lg font-medium text-neutral-800">消息详情</h2>
        </a>
      </div>
      <div class="flex items-center space-x-3">
        <button class="p-2 text-neutral-600 transition-colors">
          <IconStar />
        </button>
        <button class="p-2 text-neutral-600 transition-colors">
          <IconMenu2 />
        </button>
      </div>
    </div>
  </header>
</template>

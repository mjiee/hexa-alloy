<script setup lang="ts">
import { IconHome, IconReport } from "@tabler/icons-vue";
</script>

<template>
  <nav class="md:hidden fixed bottom-0 left-0 right-0 z-50 bg-white shadow-[0_-1px_2px_rgba(0,0,0,0.05)] pb-safe">
    <div class="flex justify-around items-center py-2">
      <router-link to="/">
        <IconHome class="h-5 w-5" />
        <span class="text-xs font-medium">主页</span>
      </router-link>

      <router-link to="/reports">
        <IconReport class="h-5 w-5" />
        <span class="text-xs font-medium">报表</span>
      </router-link>
    </div>
  </nav>
</template>

<style scoped>
.pb-safe {
  padding-bottom: env(safe-area-inset-bottom, 4px);
}
</style>

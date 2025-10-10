<script setup lang="ts">
import { IconHome, IconReport } from "@tabler/icons-vue";
</script>

<template>
  <div class="w-64 min-h-screen bg-white backdrop-blur-sm shadow-sm p-4">
    <div class="mb-8">
      <h1 class="text-xl font-bold text-gray-800">股票资讯</h1>
    </div>

    <nav class="space-y-2">
      <router-link
        to="/"
        active-class="bg-white text-blue-600 font-medium shadow-sm"
        class="flex items-center space-x-3 p-3 rounded-xl text-gray-600 transition-all hover:bg-white hover:shadow-sm"
      >
        <IconHome class="h-5 w-5" />
        <span>主页</span>
      </router-link>

      <router-link
        to="/reports"
        active-class="bg-white text-blue-600 font-medium shadow-sm"
        class="flex items-center space-x-3 p-3 rounded-xl text-gray-600 transition-all hover:bg-white hover:shadow-sm"
      >
        <IconReport class="h-5 w-5" />
        <span>报表</span>
      </router-link>
    </nav>
  </div>
</template>

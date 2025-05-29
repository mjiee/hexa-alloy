<script setup lang="ts">
import { ref } from "vue";
import { IconArrowUp, IconArrowDown } from "@tabler/icons-vue";

const data = ref([
  { name: "净资产", value: 1235600, change: 3.2 },
  { name: "总资产", value: 1580400, change: 2.8 },
  { name: "负债", value: 344800, change: -0.5 },
]);
</script>

<template>
  <router-link to="/assets">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div v-for="(item, index) in data" :key="index" class="bg-white rounded-xl shadow-sm p-6 asset-card">
        <p class="text-sm text-slate-500 mb-1">{{ item.name }}</p>
        <p class="text-2xl font-bold">{{ item.value }} ¥</p>
        <div class="flex items-center mt-2 text-sm">
          <span class="flex items-center" :class="item.change > 0 ? 'text-emerald-500' : 'text-rose-500'">
            <IconArrowUp v-if="item.change > 0" class="mr-1" />
            <IconArrowDown v-else class="mr-1" />
            {{ item.change }}%
          </span>
          <span class="text-slate-500 ml-2">较上月</span>
        </div>
      </div>
    </div>
  </router-link>
</template>

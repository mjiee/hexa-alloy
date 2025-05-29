<script setup lang="ts">
import { IconArrowDown, IconArrowUp } from "@tabler/icons-vue";
</script>

<template>
  <h2 class="text-lg font-semibold mb-4">关键指标</h2>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
    <div class="indicator-card bg-white rounded-xl shadow-sm p-4">
      <p class="text-gray-500 text-sm">市值</p>
      <p class="text-xl font-bold">2.34万亿</p>
      <div class="flex items-center mt-1">
        <span class="text-xs stock-up"><IconArrowDown class="mr-1" />0.8% </span>
        <span class="text-xs text-gray-500 ml-2">较上月</span>
      </div>
    </div>
    <div class="indicator-card bg-white rounded-xl shadow-sm p-4">
      <p class="text-gray-500 text-sm">市盈率(TTM)</p>
      <p class="text-xl font-bold">18.42</p>
      <div class="flex items-center mt-1">
        <span class="text-xs stock-down"><IconArrowDown class="mr-1" />2.1% </span>
        <span class="text-xs text-gray-500 ml-2">较上月</span>
      </div>
    </div>
    <div class="indicator-card bg-white rounded-xl shadow-sm p-4">
      <p class="text-gray-500 text-sm">股息率</p>
      <p class="text-xl font-bold">1.75%</p>
      <div class="flex items-center mt-1">
        <span class="text-xs stock-up"><IconArrowUp class="mr-1" />0.3% </span>
        <span class="text-xs text-gray-500 ml-2">较上月</span>
      </div>
    </div>
    <div class="indicator-card bg-white rounded-xl shadow-sm p-4">
      <p class="text-gray-500 text-sm">52周波动</p>
      <p class="text-xl font-bold">$65.32 - $112.74</p>
      <div class="mt-1">
        <div class="w-full bg-gray-200 rounded-full h-1.5">
          <div class="bg-red-500 h-1.5 rounded-full" style="width: 40%"></div>
        </div>
      </div>
    </div>
  </div>
</template>

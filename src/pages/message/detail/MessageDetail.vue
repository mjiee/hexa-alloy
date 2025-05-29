<script setup lang="ts">
import { ref } from "vue";
import { IconChartLine } from "@tabler/icons-vue";

const message = ref({
  title: "市场分析报告",
  time: "2025年3月19日 08:30",
  content:
    "尊敬的张智慧用户：\n根据您的投资偏好和关注列表，我们为您生成了本季度科技股投资策略分析报告。本报告基于最新市场数据和行业趋势，旨在为您的投资决策提供参考。\n第一季度科技板块整体表现活跃，特别是AI芯片、云计算和新能源技术领域。我们观察到以下主要趋势：",
});
</script>

<template>
  <div class="bg-white rounded-lg shadow-soft p-5 mb-6">
    <div class="flex items-center mb-4">
      <div class="w-12 h-12 flex items-center justify-center bg-primary-light text-primary rounded-full">
        <IconChartLine class="text-xl" />
      </div>
      <div class="ml-4">
        <h3 class="text-lg font-medium text-neutral-800">{{ message.title }}</h3>
        <p class="text-sm text-neutral-500">{{ message.time }}</p>
      </div>
      <div class="ml-auto">
        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
          重要
        </span>
      </div>
    </div>

    <div class="prose max-w-none text-neutral-700">
      <p class="mb-4">
        {{ message.content }}
      </p>
    </div>
  </div>
</template>

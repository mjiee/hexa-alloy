<script lang="ts" setup>
import { computed } from "vue";
import { ReportDetail, marketImpactOptions, categoryOptions } from "@/services/reportApi";

const props = defineProps<{ report: ReportDetail }>();

const trades = computed(() => {
  const prices = props.report.prices || [];
  const fixedLength = 7;

  return Array.from({ length: fixedLength }, (__, index) => {
    const day = index + 1;
    const priceData = prices[index];

    return {
      label: "D" + day,
      value: priceData && priceData.change_percent != 0 ? priceData.change_percent + "%" : "-",
      color:
        priceData && priceData.change_percent
          ? priceData.change_percent > 0
            ? "text-green-600"
            : "text-red-600"
          : "text-gray-500",
    };
  });
});
</script>

<template>
  <router-link :to="'/report/' + props.report.id">
    <div class="bg-white rounded-2xl shadow p-4 space-y-3">
      <div v-if="props.report.news">
        <h3 class="text-base font-semibold text-gray-900">{{ props.report.news.title }}</h3>
        <p class="text-xs text-gray-500 mt-1">来源：{{ props.report.news.source }} · {{ props.report.report_time }}</p>
      </div>
      <div v-if="props.report.stock" class="flex flex-wrap gap-2">
        <span class="px-2 py-1 text-xs rounded-full bg-blue-100 font-medium">
          {{ props.report.stock.name }}
        </span>
        <span class="px-2 py-1 text-xs rounded-full border border-blue-500 text-blue-600">
          {{ props.report.stock.ticker }}
        </span>
        <span class="px-2 py-1 text-xs rounded-full bg-green-100 text-green-700 font-medium">
          {{ marketImpactOptions.find((v) => v.value == props.report.market_impact)?.label || "中性" }}
        </span>
        <span class="px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-700 font-medium">
          {{ categoryOptions.find((v) => v.value == props.report.category)?.label || "" }}
        </span>
      </div>
      <div class="grid grid-cols-7 text-center text-xs gap-y-2">
        <div v-for="item in trades" :key="item.label">
          <div class="text-gray-500">{{ item.label }}</div>
          <div class="font-medium" :class="[item.color]">{{ item.value }}</div>
        </div>
      </div>
    </div>
  </router-link>
</template>

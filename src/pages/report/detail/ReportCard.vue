<script lang="ts" setup>
import { ReportDetail, marketImpactOptions, categoryOptions } from "@/services/reportApi";
import PricesChart from "./PricesChart.vue";
import { IconClock } from "@tabler/icons-vue";

const { report } = defineProps<{ report: ReportDetail }>();
</script>

<template>
  <div class="bg-white rounded-lg shadow-md px-4">
    <div>
      <h1 class="text-xl font-bold text-gray-900 mb-2">{{ report?.news?.title ?? "" }}</h1>
      <p v-if="report.report_time" class="flex items-center gap-1 mb-2">
        <IconClock class="w-4 h-4" />
        {{ report.report_time }}
      </p>
      <div class="flex flex-wrap items-center gap-4 text-sm text-gray-600 mb-2">
        <span v-if="report.category" class="px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-700 font-medium">
          {{ categoryOptions.find((v) => v.value == report.category)?.label }}
        </span>

        <span
          v-if="report.market_impact"
          class="px-2 py-1 text-xs rounded-full bg-green-100 text-green-700 font-medium"
        >
          {{ marketImpactOptions.find((v) => v.value == report.market_impact)?.label || "中性" }}
        </span>
      </div>

      <PricesChart :prices="report.prices ?? []" />
    </div>
  </div>
</template>

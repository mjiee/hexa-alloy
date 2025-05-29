<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { LineChart } from "echarts/charts";
import { TooltipComponent, LegendComponent, GridComponent } from "echarts/components";
import type { LineSeriesOption } from "echarts/charts";
import type { TooltipComponentOption, LegendComponentOption, GridComponentOption } from "echarts/components";
import type { ComposeOption } from "echarts/core";
import VChart from "vue-echarts";

type ECOption = ComposeOption<LineSeriesOption | TooltipComponentOption | LegendComponentOption | GridComponentOption>;

use([CanvasRenderer, LineChart, TooltipComponent, LegendComponent, GridComponent]);

const option = ref<ECOption>({
  tooltip: { trigger: "axis" },
  legend: { data: ["收入", "支出"] },
  xAxis: { type: "category", boundaryGap: false, data: ["1月", "2月", "3月", "4月", "5月"] },
  yAxis: { type: "value", show: false },
  grid: { containLabel: false },
  series: [
    {
      name: "收入",
      type: "line",
      stack: "Total",
      smooth: true,
      data: [120, 132, 101, 134, 90],
    },
    {
      name: "支出",
      type: "line",
      stack: "Total",
      smooth: true,
      data: [220, 182, 191, 234, 290],
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-semibold">收支趋势</h2>
      <div class="flex space-x-2">
        <button class="px-3 py-1 text-sm rounded-full bg-primary text-white">全部</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100 text-gray-700 hover:bg-gray-200">收入</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100 text-gray-700 hover:bg-gray-200">支出</button>
      </div>
    </div>
    <div class="h-[200px]">
      <VChart class="w-full h-full" :option="option" autoresize />
    </div>
  </div>
</template>

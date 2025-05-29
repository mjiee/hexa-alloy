<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { LineChart } from "echarts/charts";
import { TooltipComponent, GridComponent } from "echarts/components";
import type { LineSeriesOption } from "echarts/charts";
import type { TooltipComponentOption, GridComponentOption } from "echarts/components";
import type { ComposeOption } from "echarts/core";
import VChart from "vue-echarts";

type ECOption = ComposeOption<LineSeriesOption | TooltipComponentOption | GridComponentOption>;

use([CanvasRenderer, LineChart, TooltipComponent, GridComponent]);

const option = ref<ECOption>({
  tooltip: { trigger: "axis" },
  xAxis: { type: "category", show: false, data: ["1", "2", "3", "4", "5", "6", "7"] },
  yAxis: { type: "value", show: false },
  grid: { top: 0, bottom: 0, left: 0, right: 0, containLabel: false },
  series: [
    {
      data: [820, 932, 901, 934, 700, 1330, 500],
      type: "line",
      areaStyle: { opacity: 0.2 },
      smooth: true,
      symbol: "none",
      lineStyle: { width: 2 },
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-md p-4 mb-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-semibold">股价走势</h2>
      <div class="flex space-x-2">
        <button class="px-3 py-1 text-sm rounded-full bg-primary text-white">1天</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100">1周</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100">1月</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100">1年</button>
        <button class="px-3 py-1 text-sm rounded-full bg-gray-100">5年</button>
      </div>
    </div>
    <div class="h-[300px] relative rounded-lg overflow-hidden">
      <VChart width="100%" height="100%" :option="option" autoresize />
      <div class="absolute bottom-4 right-4 bg-white/80 backdrop-blur-sm px-3 py-2 rounded-lg shadow-sm">
        <div class="flex items-center">
          <div class="w-3 h-3 rounded-full bg-primary mr-2"></div>
          <span class="text-sm">收盘价: $87.24</span>
        </div>
      </div>
    </div>
  </div>
</template>

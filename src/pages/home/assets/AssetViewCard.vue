<script setup lang="ts">
import { ref } from "vue";
import { IconArrowUp } from "@tabler/icons-vue";
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
  xAxis: { type: "category", show: false, data: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] },
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
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
    <div class="flex flex-col md:flex-row justify-between md:items-center gap-4">
      <div class="min-w-0">
        <p class="text-gray-500 text-sm">总资产</p>
        <h2 class="text-3xl font-bold">¥2,568,421.36</h2>
        <div class="flex items-center mt-1">
          <span class="text-sm text-green-500 font-medium flex items-center"><IconArrowUp class="mr-1" />3.2%</span>
          <span class="text-xs text-gray-500 ml-2">较上月</span>
        </div>
      </div>
      <div class="min-w-0 w-full max-w-[400px] h-[60px]">
        <VChart class="w-full h-full" :option="option" autoresize />
      </div>
    </div>
  </div>
</template>

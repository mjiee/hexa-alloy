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
  legend: { data: ["总资产", "净资产", "负债"] },
  xAxis: { type: "category", boundaryGap: false, data: ["2023-01", "2023-02", "2023-03", "2023-04", "2023-05"] },
  yAxis: { type: "value" },
  grid: { containLabel: false },
  series: [
    {
      name: "总资产",
      type: "line",
      stack: "Total",
      smooth: true,
      data: [120, 132, 101, 134, 90],
    },
    {
      name: "净资产",
      type: "line",
      stack: "Total",
      smooth: true,
      data: [220, 182, 191, 234, 290],
    },
    {
      name: "负债",
      type: "line",
      stack: "Total",
      smooth: true,
      data: [150, 232, 201, 154, 190],
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
    <div class="flex justify-between items-center mb-4">
      <h3 class="font-medium">资产变化趋势</h3>
      <button class="text-blue-600 text-sm hover:underline">查看全部</button>
    </div>
    <div class="h-[300px]">
      <v-chart class="w-full h-full" :option="option" autoresize />
    </div>
  </div>
</template>

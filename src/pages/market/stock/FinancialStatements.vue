<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { BarChart } from "echarts/charts";
import { TooltipComponent, GridComponent } from "echarts/components";
import type { BarSeriesOption } from "echarts/charts";
import type { TooltipComponentOption, GridComponentOption } from "echarts/components";
import type { ComposeOption } from "echarts/core";
import VChart from "vue-echarts";

type ECOption = ComposeOption<BarSeriesOption | TooltipComponentOption | GridComponentOption>;

use([CanvasRenderer, BarChart, TooltipComponent, GridComponent]);

const option = ref<ECOption>({
  xAxis: { type: "category", data: ["Q1", "Q2", "Q3", "Q4"] },
  yAxis: { type: "value" },
  grid: { top: 5, bottom: 20, left: 30, right: 5, containLabel: false },
  series: [{ data: [120, 200, 150, 80], type: "bar" }],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-md p-4 mb-6">
    <h2 class="text-lg font-semibold mb-4">财务数据</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div>
        <h3 class="text-md font-medium mb-3">季度收益（十亿美元）</h3>
        <div class="h-40 relative">
          <VChart :option="option" autoresize />
        </div>
      </div>
      <div>
        <h3 class="text-md font-medium mb-3">关键财务比率</h3>
        <div class="space-y-3">
          <div>
            <div class="flex justify-between text-sm mb-1">
              <span>净利润率</span>
              <span>21.4%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-green-500 h-2 rounded-full" style="width: 21.4%"></div>
            </div>
          </div>
          <div>
            <div class="flex justify-between text-sm mb-1">
              <span>资产回报率</span>
              <span>8.6%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-blue-500 h-2 rounded-full" style="width: 8.6%"></div>
            </div>
          </div>
          <div>
            <div class="flex justify-between text-sm mb-1">
              <span>负债比率</span>
              <span>35.2%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-yellow-500 h-2 rounded-full" style="width: 35.2%"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

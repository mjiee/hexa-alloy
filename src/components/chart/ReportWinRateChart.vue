<script lang="ts" setup>
import { computed } from "vue";
import VChart from "vue-echarts";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { BarChart, GaugeChart } from "echarts/charts";
import { TitleComponent, TooltipComponent, GridComponent, LegendComponent } from "echarts/components";
import type { TooltipComponentFormatterCallbackParams } from "echarts";
import type { ReportWinRate } from "@/services/reportApi";

use([CanvasRenderer, BarChart, GaugeChart, TitleComponent, TooltipComponent, GridComponent, LegendComponent]);

const props = defineProps<{ winRate?: ReportWinRate }>();

const totalCount = computed(() => props.winRate?.total ?? 0);

const dailyRates = computed(() => [
  { day: "D1", rate: props.winRate?.day1_win_rate ?? 0, label: "第1天" },
  { day: "D2", rate: props.winRate?.day2_win_rate ?? 0, label: "第2天" },
  { day: "D3", rate: props.winRate?.day3_win_rate ?? 0, label: "第3天" },
  { day: "D4", rate: props.winRate?.day4_win_rate ?? 0, label: "第4天" },
  { day: "D5", rate: props.winRate?.day5_win_rate ?? 0, label: "第5天" },
  { day: "D6", rate: props.winRate?.day6_win_rate ?? 0, label: "第6天" },
  { day: "D7", rate: props.winRate?.day7_win_rate ?? 0, label: "第7天" },
]);

const getGradientColor = (rate: number) => {
  if (rate >= 70) {
    return {
      type: "linear",
      x: 0,
      y: 0,
      x2: 0,
      y2: 1,
      colorStops: [
        { offset: 0, color: "#10b981" },
        { offset: 1, color: "#34d399" },
      ],
    };
  }
  if (rate >= 50) {
    return {
      type: "linear",
      x: 0,
      y: 0,
      x2: 0,
      y2: 1,
      colorStops: [
        { offset: 0, color: "#3b82f6" },
        { offset: 1, color: "#22d3ee" },
      ],
    };
  }
  if (rate >= 30) {
    return {
      type: "linear",
      x: 0,
      y: 0,
      x2: 0,
      y2: 1,
      colorStops: [
        { offset: 0, color: "#f59e0b" },
        { offset: 1, color: "#fb923c" },
      ],
    };
  }
  return {
    type: "linear",
    x: 0,
    y: 0,
    x2: 0,
    y2: 1,
    colorStops: [
      { offset: 0, color: "#ef4444" },
      { offset: 1, color: "#ec4899" },
    ],
  };
};

const barOption = computed(() => {
  const rates = dailyRates.value.map((item) => item.rate);
  const maxRate = Math.max(...rates, 0);
  const minRate = Math.min(...rates.filter((r) => r > 0), 0);

  const padding = 5;
  const yAxisMax = Math.min(Math.ceil((maxRate + padding) / 10) * 10, 100);
  const yAxisMin = Math.max(Math.floor(Math.max(minRate - padding, 0) / 10) * 10, 0);

  return {
    grid: { left: "2%", right: "1%", bottom: "2%", top: "2%", containLabel: true },
    xAxis: {
      type: "category",
      data: dailyRates.value.map((item) => item.day),
      axisLine: { lineStyle: { color: "#e5e7eb" } },
      axisLabel: { color: "#6b7280", fontSize: 13, fontWeight: 600 },
    },
    yAxis: {
      type: "value",
      min: yAxisMin,
      max: yAxisMax,
      interval: 10,
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: "#6b7280", fontSize: 10, fontWeight: 500, formatter: "{value}%" },
      splitLine: {
        lineStyle: { color: "#f3f4f6", type: "dashed" },
      },
    },
    tooltip: {
      trigger: "axis",
      backgroundColor: "rgba(0, 0, 0, 0.8)",
      borderWidth: 0,
      textStyle: { color: "#fff", fontSize: 12 },
      formatter: (params: TooltipComponentFormatterCallbackParams) => {
        if (Array.isArray(params) && params.length > 0) {
          const data = params[0];
          const value = typeof data.value === "number" ? data.value : 0;
          return `${dailyRates.value[data.dataIndex].label}<br/><strong>${value.toFixed(1)}%</strong>`;
        }
        return "";
      },
    },
    series: [
      {
        type: "bar",
        data: dailyRates.value.map((item) => ({
          value: item.rate,
          itemStyle: { color: getGradientColor(item.rate), borderRadius: [8, 8, 0, 0] },
        })),
        barWidth: "65%",
        emphasis: {
          itemStyle: { shadowBlur: 10, shadowColor: "rgba(0, 0, 0, 0.2)" },
        },
      },
    ],
  };
});
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-medium text-gray-600">7日胜率趋势</h3>
      <div class="flex items-center space-x-3">
        <span class="text-xs text-gray-500">
          样本数: <span class="font-semibold text-gray-700">{{ totalCount }}</span>
        </span>
        <div class="flex items-center space-x-1.5">
          <div class="w-1.5 h-1.5 rounded-full bg-blue-500 animate-pulse"></div>
          <span class="text-xs text-gray-500">实时</span>
        </div>
      </div>
    </div>

    <div class="h-46">
      <v-chart :option="barOption" autoresize class="w-full" />
    </div>

    <div class="flex items-center justify-between pt-4 border-t border-gray-100">
      <div class="text-xs text-gray-500">
        更新时间: {{ new Date().toLocaleString("zh-CN", { hour: "2-digit", minute: "2-digit" }) }}
      </div>
      <div class="flex items-center space-x-4 text-xs">
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 rounded-full bg-gradient-to-r from-emerald-500 to-green-400"></div>
          <span class="text-gray-600">优秀 ≥70%</span>
        </div>
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 rounded-full bg-gradient-to-r from-blue-500 to-cyan-400"></div>
          <span class="text-gray-600">良好 ≥50%</span>
        </div>
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 rounded-full bg-gradient-to-r from-amber-500 to-orange-400"></div>
          <span class="text-gray-600">一般 ≥30%</span>
        </div>
      </div>
    </div>
  </div>
</template>

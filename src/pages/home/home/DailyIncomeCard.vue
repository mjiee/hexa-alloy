<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { HeatmapChart } from "echarts/charts";
import { TitleComponent, TooltipComponent, VisualMapComponent, CalendarComponent } from "echarts/components";
import type { HeatmapSeriesOption } from "echarts/charts";
import type {
  TitleComponentOption,
  TooltipComponentOption,
  VisualMapComponentOption,
  CalendarComponentOption,
} from "echarts/components";
import type { CallbackDataParams } from "echarts/types/dist/shared";
import type { ComposeOption } from "echarts/core";
import VChart from "vue-echarts";

type ECOption = ComposeOption<
  | HeatmapSeriesOption
  | TitleComponentOption
  | TooltipComponentOption
  | VisualMapComponentOption
  | CalendarComponentOption
>;

use([CanvasRenderer, HeatmapChart, TitleComponent, TooltipComponent, VisualMapComponent, CalendarComponent]);

const graphData = [
  ["2025-02-01", -260],
  ["2025-02-04", 200],
  ["2025-02-09", 279],
  ["2025-02-13", 847],
  ["2025-02-18", 241],
  ["2025-02-23", -411],
  ["2025-02-27", 985],
];

const option = ref<ECOption>({
  tooltip: {
    show: true,
    position: "top",
  },
  visualMap: {
    show: false,
    pieces: [
      { gt: 0, color: "#f87171" },
      { lte: 0, color: "#4ade80" },
    ],
    dimension: 1,
  },
  calendar: [
    {
      orient: "vertical",
      cellSize: "auto",
      left: "center",
      dayLabel: {
        firstDay: 1,
        nameMap: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
      },
      itemStyle: {
        borderWidth: 1,
        borderColor: "#e5e7eb",
      },
      yearLabel: { show: false },
      monthLabel: { show: false },
      range: "2025-02",
    },
  ],
  series: [
    {
      type: "heatmap",
      coordinateSystem: "calendar",
      calendarIndex: 0,
      data: graphData,
      label: {
        show: true,
        formatter: (params: CallbackDataParams) => {
          const [date, amount] = params.value as [string, number];
          return `${date}\n\n${amount}`;
        },
        color: "#000",
      },
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
    <div class="flex justify-between items-center mb-4">
      <h3 class="font-medium">近期收益</h3>
      <router-link to="/transactions">
        <button class="text-blue-600 text-sm hover:underline">查看全部</button>
      </router-link>
    </div>

    <div class="h-[400px]">
      <VChart class="w-full h-full" :option="option" autoresize />
    </div>
  </div>
</template>

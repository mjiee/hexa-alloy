<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { ScatterChart, HeatmapChart } from "echarts/charts";
import { TitleComponent, TooltipComponent, VisualMapComponent, CalendarComponent } from "echarts/components";
import type { ScatterSeriesOption, HeatmapSeriesOption } from "echarts/charts";
import type {
  TitleComponentOption,
  TooltipComponentOption,
  VisualMapComponentOption,
  CalendarComponentOption,
} from "echarts/components";
import type { ComposeOption } from "echarts/core";
import VChart from "vue-echarts";

type ECOption = ComposeOption<
  | ScatterSeriesOption
  | HeatmapSeriesOption
  | TitleComponentOption
  | TooltipComponentOption
  | VisualMapComponentOption
  | CalendarComponentOption
>;

use([
  CanvasRenderer,
  ScatterChart,
  HeatmapChart,
  TitleComponent,
  TooltipComponent,
  VisualMapComponent,
  CalendarComponent,
]);

const graphData = [
  ["2025-02-01", 260],
  ["2025-02-04", 200],
  ["2025-02-09", 279],
  ["2025-02-13", 847],
  ["2025-02-18", 241],
  ["2025-02-23", 411],
  ["2025-02-27", 985],
];

const option = ref<ECOption>({
  tooltip: {
    show: true,
    position: "top",
  },
  visualMap: [
    {
      min: 0,
      max: 1000,
      seriesIndex: [0],
    },
  ],
  calendar: [
    {
      orient: "vertical",
      cellSize: "auto",
      dayLabel: {
        firstDay: 1,
        nameMap: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
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
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6">
    <div class="flex justify-between items-center mb-4">
      <h3 class="font-medium">近期收益</h3>
      <button class="text-blue-600 text-sm hover:underline">查看全部</button>
    </div>
    <div>
      <v-chart class="chart-container" :option="option" />
    </div>
  </div>
</template>

<style scoped>
.chart-container {
  width: 100%;
  height: 800px;
}
</style>

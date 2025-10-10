<script lang="ts" setup>
import { computed } from "vue";
import VChart from "vue-echarts";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { LineChart, CandlestickChart } from "echarts/charts";
import { TitleComponent, TooltipComponent, GridComponent, DataZoomComponent } from "echarts/components";
import type {
  ComposeOption,
  TitleComponentOption,
  TooltipComponentOption,
  GridComponentOption,
  CandlestickSeriesOption,
} from "echarts";
import type { CallbackDataParams } from "echarts/types/dist/shared";
import { StockPrice } from "@/services/stockApi";

use([CanvasRenderer, LineChart, CandlestickChart, TitleComponent, TooltipComponent, GridComponent, DataZoomComponent]);

type ECOption = ComposeOption<
  TitleComponentOption | TooltipComponentOption | GridComponentOption | CandlestickSeriesOption
>;

const { prices } = defineProps<{ prices: StockPrice[] }>();

const formatKLineData = (): number[][] => {
  return prices.map((p) =>
    [
      p.open_price ?? p.close_price ?? 0,
      p.close_price ?? 0,
      p.low_price ?? p.close_price ?? 0,
      p.high_price ?? p.close_price ?? 0,
    ].map((v) => Number(v.toFixed(2))),
  );
};

const formatDates = (): string[] => {
  return prices.map((p) => p.trade_date || "");
};

const chartOption = computed(() => {
  const baseOption: ECOption = {
    grid: { left: "3%", right: "4%", bottom: prices.length > 50 ? "15%" : "10%", top: "10%", containLabel: true },
    xAxis: {
      type: "category",
      data: formatDates(),
      axisLine: { lineStyle: { color: "#e5e7eb" } },
      axisLabel: { color: "#6b7280" },
    },
    yAxis: {
      type: "value",
      scale: true,
      axisLine: { show: false },
      splitLine: { lineStyle: { color: "#f3f4f6" } },
      axisLabel: { color: "#6b7280" },
    },
  };

  return {
    ...baseOption,
    xAxis: {
      ...baseOption.xAxis,
      boundaryGap: true,
    },
    tooltip: {
      trigger: "axis",
      axisPointer: { type: "cross" },
      formatter: (params: CallbackDataParams | CallbackDataParams[]): string => {
        const param = Array.isArray(params) ? params[0] : params;
        const data = param.data as number[] | undefined;
        if (!data || data.length < 4) return "";

        return `${param.name}<br/>
            开盘: ¥${data[0].toFixed(2)}<br/>
            收盘: ¥${data[1].toFixed(2)}<br/>
            最低: ¥${data[2].toFixed(2)}<br/>
            最高: ¥${data[3].toFixed(2)}`;
      },
    },
    series: [
      {
        type: "candlestick",
        data: formatKLineData(),
        itemStyle: { color: "#ef4444", color0: "#10b981", borderColor: "#ef4444", borderColor0: "#10b981" },
      },
    ],
  } as ECOption;
});
</script>

<template>
  <div v-if="prices.length > 0" class="h-46">
    <v-chart :option="chartOption" autoresize />
  </div>
  <div v-else class="h-full flex items-center justify-center text-gray-500">暂无数据...</div>
</template>

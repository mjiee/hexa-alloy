<script lang="ts" setup>
import { computed, ref, watch } from "vue"; // 引入 watch 和 onMounted
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
  DataZoomComponentOption,
  LineSeriesOption,
  CandlestickSeriesOption,
} from "echarts";
import type { CallbackDataParams } from "echarts/types/dist/shared";
import { getStockPrices, StockDetail, StockPrice } from "@/services/stockApi";

// register components
use([CanvasRenderer, LineChart, CandlestickChart, TitleComponent, TooltipComponent, GridComponent, DataZoomComponent]);

type ECOption = ComposeOption<
  | TitleComponentOption
  | TooltipComponentOption
  | GridComponentOption
  | DataZoomComponentOption
  | LineSeriesOption
  | CandlestickSeriesOption
>;

const props = defineProps<{ stock: StockDetail }>();
const days = ref<number>(1);
const prices = ref<StockPrice[]>([]);

const priceInfo = computed(() => {
  const quote = props.stock?.quote;
  if (!quote || quote.current_price === undefined || quote.prev_close_price === undefined) {
    return { change: 0, changePercent: 0, isPositive: false };
  }

  const change = quote.change ?? 0;
  return { change: change, changePercent: quote.change_percent ?? 0, isPositive: change >= 0 };
});

const getPrices = async () => {
  if (!props.stock?.ticker) return;
  const resp = await getStockPrices({ ticker: props.stock.ticker, days: days.value });
  prices.value = resp ?? [];
};

watch(days, getPrices, { immediate: true });

const formatKLineData = (priceList: StockPrice[]): number[][] => {
  if (days.value === 1) return [];

  return priceList.map((p) =>
    [
      p.open_price ?? p.close_price ?? 0,
      p.close_price ?? 0,
      p.low_price ?? p.close_price ?? 0,
      p.high_price ?? p.close_price ?? 0,
    ].map((v) => Number(v.toFixed(2))),
  );
};

const formatDates = (priceList: StockPrice[]): string[] => {
  if (days.value === 1) {
    return priceList.map((p) => p.trade_time || "");
  }

  return priceList.map((p) => p.trade_date || "");
};

// chart options
const chartOption = computed(() => {
  const quote = props.stock?.quote;
  if (!quote) return {};

  const dates = formatDates(prices.value);
  const isPositive = priceInfo.value.isPositive;

  const baseOption: ECOption = {
    dataZoom: [
      { type: "inside", xAxisIndex: [0], start: 0, end: 100 },
      { show: prices.value.length > 50, type: "slider", xAxisIndex: [0], start: 0, end: 100 },
    ],
    grid: { left: "3%", right: "4%", bottom: prices.value.length > 50 ? "15%" : "10%", top: "10%", containLabel: true },
    xAxis: {
      type: "category",
      data: dates,
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

  if (days.value <= 1) {
    const lineData = prices.value.map((p) => p.current_price ?? p.close_price ?? 0);

    return {
      ...baseOption,
      xAxis: {
        ...baseOption.xAxis,
        boundaryGap: false,
      },
      tooltip: {
        trigger: "axis",
        formatter: (params: CallbackDataParams[] | CallbackDataParams): string => {
          const param = Array.isArray(params) ? params[0] : params;
          const index = param.dataIndex;
          const price = prices.value[index];
          if (!price) return "";
          return `${price.trade_time}<br/>价格: ¥${(price.current_price ?? 0).toFixed(2)}`;
        },
      },
      series: [
        {
          data: lineData,
          type: "line",
          smooth: true,
          symbol: "none",
          lineStyle: { color: isPositive ? "#ef4444" : "#10b981", width: 2 },
          areaStyle: {
            color: {
              type: "linear",
              x: 0,
              y: 0,
              x2: 0,
              y2: 1,
              colorStops: [
                { offset: 0, color: isPositive ? "rgba(239, 68, 68, 0.3)" : "rgba(16, 185, 129, 0.3)" },
                { offset: 1, color: isPositive ? "rgba(239, 68, 68, 0)" : "rgba(16, 185, 129, 0)" },
              ],
            },
          },
        },
      ],
    } as ECOption;
  }

  const klineData = formatKLineData(prices.value);

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
        data: klineData,
        itemStyle: { color: "#ef4444", color0: "#10b981", borderColor: "#ef4444", borderColor0: "#10b981" },
      },
    ],
  } as ECOption;
});

const daysOptions = [
  { label: "分时", value: 1 },
  { label: "周", value: 7 },
  { label: "月", value: 30 },
  { label: "年", value: 360 },
];
</script>

<template>
  <div>
    <div class="flex justify-between items-center mb-3">
      <h3 class="text-lg font-semibold text-gray-900">价格走势</h3>
      <div class="flex gap-2">
        <button
          v-for="item in daysOptions"
          :key="item.value"
          @click="days = item.value"
          :class="[
            'px-3 py-1 text-sm rounded-md transition-colors',
            days === item.value ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
          ]"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
    <div class="h-46">
      <v-chart v-if="prices.length > 0" :option="chartOption" autoresize class="w-full" />
      <div v-else class="h-full flex items-center justify-center text-gray-500">加载中或暂无数据...</div>
    </div>
  </div>
</template>

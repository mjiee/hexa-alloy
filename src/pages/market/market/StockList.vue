<script setup lang="ts">
import { ref } from "vue";
import { IconChartLine, IconDotsVertical } from "@tabler/icons-vue";
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

const stocks = ref([
  {
    id: 1,
    name: "阿里巴巴",
    symbol: "09988.HK",
    industry: "电子商务",
    price: 72.8,
    change: 2.95,
    changePercent: 4.22,
    volume: 4.32,
    marketCap: 1.43,
    peRatio: 13.5,
    pbRatio: 1.8,
  },
  {
    id: 2,
    name: "腾讯科技",
    symbol: "700.HK",
    industry: "互联网服务",
    price: 250.0,
    change: -1.5,
    changePercent: -6.0,
    volume: 2.8,
    marketCap: 2.2,
    peRatio: 18.0,
    pbRatio: 1.2,
  },
  {
    id: 3,
    name: "京东集团",
    symbol: "00700.HK",
    industry: "电商",
    price: 120.0,
    change: 3.2,
    changePercent: 2.6,
    volume: 3.5,
    marketCap: 1.8,
    peRatio: 15.0,
    pbRatio: 1.5,
  },
]);
</script>

<template>
  <!-- 标签页 -->
  <div class="border-b border-gray-200 mb-6">
    <div class="flex">
      <button class="tab-active py-2 px-4 font-medium">我的关注</button>
      <button class="py-2 px-4 text-gray-500">我的持仓</button>
      <button class="py-2 px-4 text-gray-500">热门推荐</button>
      <button class="py-2 px-4 text-gray-500">行业板块</button>
    </div>
  </div>

  <!-- 我的关注列表 -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-6">
    <router-link v-for="item in stocks" :key="item.id" to="/stock">
      <div class="bg-white rounded-xl card-shadow p-5 animate-in animate-delay-1 stock-item cursor-pointer">
        <div class="flex justify-between">
          <div>
            <div class="flex items-center">
              <h3 class="text-lg font-semibold">{{ item.name }}</h3>
              <span class="ml-2 text-gray-500 text-sm">{{ item.symbol }}</span>
            </div>
            <div class="mt-1 text-sm text-gray-500">{{ item.industry }}</div>
          </div>
          <div class="text-right">
            <div class="text-xl font-semibold text-green-500">¥{{ item.price }}</div>
            <div class="text-sm text-green-500">{{ item.change }} ({{ item.changePercent }}%)</div>
          </div>
        </div>

        <div class="mt-4 flex">
          <div class="w-4/5">
            <VChart width="100%" height="60" :option="option" autoresize />
          </div>
          <div class="w-1/3 text-right">
            <div class="flex flex-col justify-between h-full">
              <div>
                <div class="text-xs text-gray-500">成交量</div>
                <div class="text-sm">{{ item.volume }}亿</div>
              </div>
              <div>
                <div class="text-xs text-gray-500">市值</div>
                <div class="text-sm">{{ item.marketCap }}万亿</div>
              </div>
            </div>
          </div>
        </div>

        <div class="mt-4 flex justify-between">
          <div class="flex items-center">
            <div class="px-2 py-1 rounded-full bg-blue-100 text-blue-600 text-xs mr-2">PE {{ item.peRatio }}</div>
            <div class="px-2 py-1 rounded-full bg-blue-100 text-blue-600 text-xs">PB {{ item.pbRatio }}</div>
          </div>
          <div>
            <button class="text-gray-500 hover:text-blue-600 mr-3">
              <IconChartLine />
            </button>
            <button class="text-gray-500 hover:text-blue-600">
              <IconDotsVertical />
            </button>
          </div>
        </div>
      </div>
    </router-link>
  </div>
</template>

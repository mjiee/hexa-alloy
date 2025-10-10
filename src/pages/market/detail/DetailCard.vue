<script lang="ts" setup>
import { computed, defineAsyncComponent } from "vue";
import { StockDetail } from "@/services/stockApi";
import { formatNumber, formatLargeNumber, formatDateTime } from "@/utils/format";

const PriceChart = defineAsyncComponent(() => import("./PriceChart.vue"));

const { stock } = defineProps<{ stock: StockDetail }>();

// stock prick info
const priceInfo = computed(() => {
  const quote = stock?.quote;
  if (!quote || !quote.current_price || !quote.prev_close_price)
    return { change: 0, changePercent: 0, isPositive: false };

  const change = quote.change ?? 0;

  return { change: change, changePercent: quote.change_percent, isPositive: change >= 0 };
});
</script>

<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <h1 class="text-3xl font-semibold py-2">{{ stock.name }} ({{ stock.ticker }})</h1>

    <!-- stock price -->
    <div v-if="stock.quote" class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div>
        <div class="flex items-baseline gap-4 mb-4">
          <span class="text-2xl font-bold" :class="priceInfo.isPositive ? 'text-red-600' : 'text-green-600'">
            ¥{{ formatNumber(stock.quote.current_price) }}
          </span>
          <div class="flex gap-2">
            <span class="text-lg font-semibold" :class="priceInfo.isPositive ? 'text-red-600' : 'text-green-600'">
              {{ priceInfo.isPositive ? "+" : "" }}{{ formatNumber(priceInfo.change) }}
            </span>
            <span class="text-lg font-semibold" :class="priceInfo.isPositive ? 'text-red-600' : 'text-green-600'">
              {{ priceInfo.isPositive ? "+" : "" }}{{ formatNumber(priceInfo.changePercent) }}%
            </span>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4 text-sm">
          <div class="space-y-2">
            <div class="flex justify-between">
              <span class="text-gray-600">开盘</span>
              <span class="font-medium">¥{{ formatNumber(stock.quote.open_price) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">最高</span>
              <span class="font-medium text-red-600">¥{{ formatNumber(stock.quote.high_price) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">成交量</span>
              <span class="font-medium">{{ formatLargeNumber(stock.quote.volume) }}</span>
            </div>
          </div>
          <div class="space-y-2">
            <div class="flex justify-between">
              <span class="text-gray-600">昨收</span>
              <span class="font-medium">¥{{ formatNumber(stock.quote.prev_close_price) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">最低</span>
              <span class="font-medium text-green-600">¥{{ formatNumber(stock.quote.low_price) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">成交额</span>
              <span class="font-medium">{{ formatLargeNumber(stock.quote.turnover) }}</span>
            </div>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-2 text-sm text-gray-600">
          <span>{{ stock.exchange }}</span>
          <span v-if="stock.industry">{{ stock.industry }}</span>
          <span v-if="stock.listing_date">上市日期: {{ stock.listing_date }}</span>
        </div>

        <div class="mt-2 text-xs text-gray-500">
          更新时间: {{ formatDateTime(stock.quote.trade_date, stock.quote.trade_time) }}
        </div>
      </div>

      <!-- K chart -->
      <PriceChart :stock="stock" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { IconChartLine, IconSearch, IconX, IconEraser } from "@tabler/icons-vue";
import { marketImpactOptions } from "@/services/reportApi";
import { useQueryReportsStore } from "@/stores/queryReportsStore";

const props = defineProps<{ onSearch: () => void }>();
const queryStore = useQueryReportsStore();

const ticker = ref(queryStore.searchParams?.stock_ticker ?? "");
const startDate = ref(queryStore.searchParams?.start_date ?? "");
const endDate = ref(queryStore.searchParams?.end_date ?? "");
const market_impact = ref(queryStore.searchParams?.market_impact);

const isExpanded = ref(false);

const expandSearch = () => {
  isExpanded.value = true;
};

const collapseSearch = () => {
  isExpanded.value = false;
};

const clearSearch = () => {
  ticker.value = "";
  startDate.value = "";
  endDate.value = "";
  market_impact.value = "";

  queryStore.resetSearchParams();
  props.onSearch();
};

const onSearch = () => {
  queryStore.setSearchParams({
    stock_ticker: ticker.value,
    start_date: startDate.value,
    end_date: endDate.value,
    market_impact: market_impact.value,
  });

  props.onSearch();
  collapseSearch();
};
</script>

<template>
  <header class="bg-white top-0 z-50 shadow-md w-full relative">
    <div class="px-4 pb-2">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-3 cursor-pointer select-none">
          <IconChartLine class="w-6 h-6 text-blue-600" />
          <div>
            <h1 class="text-xl font-bold text-gray-900">HexaAlloy</h1>
          </div>
        </div>

        <button
          v-if="!isExpanded"
          @click="expandSearch"
          class="flex items-center gap-2 px-4 py-2 bg-gray-50 hover:bg-gray-100 border border-gray-300 rounded-full transition-all shadow-inner"
        >
          <span class="text-sm text-gray-500">搜索...</span>
          <IconSearch class="w-4 h-4 text-gray-500" />
        </button>
      </div>
    </div>

    <div
      v-show="isExpanded"
      class="absolute top-full left-0 right-0 z-40 bg-white shadow-xl border-t border-gray-100 p-6 rounded-b-xl"
    >
      <div class="relative">
        <button
          @click="collapseSearch"
          class="absolute -top-3 right-0 text-gray-400 hover:text-gray-600 transition-colors p-1 rounded-full hover:bg-gray-50"
          title="关闭搜索"
        >
          <IconX class="w-5 h-5" />
        </button>

        <div class="grid grid-cols-1 gap-4 md:grid-cols-6">
          <div class="md:col-span-2">
            <label for="ticker-search" class="block text-xs font-medium text-gray-500 mb-1">股票代码</label>
            <input
              id="ticker-search"
              v-model="ticker"
              type="text"
              placeholder="请输入股票代码 (000001)"
              class="w-full border border-gray-300 rounded-lg h-11 px-4 text-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm transition-all bg-white"
              @keyup.enter="onSearch"
            />
          </div>

          <div class="md:col-span-2">
            <label for="date-range-container" class="block text-xs font-medium text-gray-500 mb-1">日期范围</label>
            <div id="date-range-container" class="flex gap-4">
              <input
                id="start-date"
                v-model="startDate"
                type="date"
                title="开始日期"
                class="flex-1 border border-gray-300 rounded-lg h-11 px-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm transition-all bg-white"
              />

              <input
                id="end-date"
                v-model="endDate"
                type="date"
                title="结束日期"
                class="flex-1 border border-gray-300 rounded-lg h-11 px-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm transition-all bg-white"
              />
            </div>
          </div>

          <div class="md:col-span-1">
            <label for="market-impact" class="block text-xs font-medium text-gray-500 mb-1">市场影响</label>
            <select
              id="market-impact"
              v-model="market_impact"
              class="w-full border border-gray-300 rounded-lg h-11 px-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm transition-all bg-white appearance-none cursor-pointer"
            >
              <option v-for="item in marketImpactOptions" :value="item.value" :key="item.value">
                {{ item.label }}
              </option>
            </select>
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-6">
          <button
            @click="clearSearch"
            class="px-5 py-2.5 bg-white text-gray-600 text-sm font-medium border border-gray-300 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 shadow-sm flex items-center gap-2 transition-colors"
          >
            <IconEraser class="w-4 h-4" />
            <span>清空</span>
          </button>

          <button
            @click="onSearch"
            class="px-5 py-2.5 bg-blue-600 text-white text-sm font-semibold rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 shadow-lg flex items-center gap-2 transition-colors"
          >
            <IconSearch class="w-4 h-4" />
            <span>搜索</span>
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

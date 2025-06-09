<script setup lang="ts">
import { ref } from "vue";
import { IconTriangle, IconChartLine } from "@tabler/icons-vue";

const messages = ref([
  {
    id: 1,
    type: "stock",
    title: "市场异动提醒",
    content: " 你关注的股票 盛思科技(SZ600089) 出现大幅波动，当前下跌",
    change: -2.5,
    time: "10:35",
  },
  {
    id: 2,
    type: "assets",
    title: "资产增值提醒",
    content: "您的投资组合价值在过去一周增长了3.2%，超过同期大盘指数",
    total: 689245.0,
    change: 21450.0,
    time: "09:23",
  },
]);
</script>

<template>
  <div class="space-y-4">
    <router-link
      v-for="item in messages"
      :key="item.id"
      to="/message/detail"
      class="block transition-transform hover:scale-[1.02] duration-200"
    >
      <!-- 股票异动消息 -->
      <div
        v-if="item.type === 'stock'"
        class="bg-red-50 border-l-4 border-red-500 rounded-lg p-4 flex items-start shadow-sm hover:shadow-md transition-shadow"
      >
        <div class="w-12 h-12 flex items-center justify-center bg-red-100 text-red-500 rounded-full flex-shrink-0">
          <IconTriangle class="text-xl" />
        </div>
        <div class="ml-4 flex-1 min-w-0">
          <div class="flex items-center justify-between mb-2">
            <h4 class="text-sm font-medium text-neutral-800">{{ item.title }}</h4>
            <span class="text-xs text-neutral-500 flex-shrink-0 ml-2">{{ item.time }}</span>
          </div>
          <p class="text-sm text-neutral-600 mb-3 leading-relaxed">
            {{ item.content }}
            <span class="font-medium text-red-600 ml-1">{{ item.change }}%</span>
          </p>
          <div class="flex gap-2">
            <button
              class="px-3 py-1.5 text-xs text-white bg-red-500 hover:bg-red-600 rounded-full transition-colors duration-200"
            >
              查看详情
            </button>
            <button
              class="px-3 py-1.5 text-xs text-neutral-600 bg-neutral-200 hover:bg-neutral-300 rounded-full transition-colors duration-200"
            >
              稍后提醒
            </button>
          </div>
        </div>
      </div>

      <!-- 资产增值消息 -->
      <div
        v-else-if="item.type === 'assets'"
        class="bg-green-50 border-l-4 border-green-500 rounded-lg p-4 flex items-start shadow-sm hover:shadow-md transition-shadow"
      >
        <div class="w-12 h-12 flex items-center justify-center bg-green-100 text-green-500 rounded-full flex-shrink-0">
          <IconChartLine class="text-xl" />
        </div>
        <div class="ml-4 flex-1 min-w-0">
          <div class="flex items-center justify-between mb-2">
            <h4 class="text-sm font-medium text-neutral-800">{{ item.title }}</h4>
            <span class="text-xs text-neutral-500 flex-shrink-0 ml-2">{{ item.time }}</span>
          </div>
          <p class="text-sm text-neutral-600 mb-3 leading-relaxed">
            {{ item.content }}
            <span class="font-medium text-green-600 ml-1">1.7%</span>
          </p>
          <div class="mb-3 p-3 bg-white rounded-lg border border-green-200">
            <div class="flex justify-between items-center text-sm mb-2">
              <span class="text-neutral-600">目前总资产</span>
              <span class="font-medium text-neutral-800">¥{{ item.total?.toLocaleString() }}</span>
            </div>
            <div class="flex justify-between items-center text-sm">
              <span class="text-neutral-600">本周增值</span>
              <span class="font-medium text-green-600">+¥{{ item.change.toLocaleString() }}</span>
            </div>
          </div>
          <div class="flex gap-2">
            <button
              class="px-3 py-1.5 text-xs text-white bg-green-500 hover:bg-green-600 rounded-full transition-colors duration-200"
            >
              查看详情
            </button>
          </div>
        </div>
      </div>
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Icon123 } from "@tabler/icons-vue";

const dateLabels = ["今天", "昨天", "本周", "上周", "本月", "上月", "自定义"];
const transactionTypes = ["收入", "支出", "转账", "投资", "理财"];
const data = ref([
  { id: 1, name: "午餐费用", type: "支出", amount: 68.5, category: "餐饮", date: "2025-03-19", time: "12:30" },
  { id: 2, name: "晚餐费用", type: "支出", amount: 120.0, category: "餐饮", date: "2025-03-19", time: "18:00" },
]);
</script>

<template>
  <!-- 时间选择器 -->
  <div class="mb-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-semibold">交易记录</h2>
      <div class="flex items-center">
        <button class="flex items-center px-3 py-1 text-sm rounded-lg bg-gray-100 text-gray-700 hover:bg-gray-200">
          <span>2025年3月</span>
          <i class="fas fa-chevron-down ml-2"></i>
        </button>
      </div>
    </div>
    <div class="flex overflow-x-auto space-x-2 py-2 no-scrollbar">
      <button class="date-pill flex-shrink-0 px-4 py-2 bg-primary text-white rounded-full">全部</button>
      <button
        v-for="(item, index) in dateLabels"
        :key="index"
        class="date-pill flex-shrink-0 px-4 py-2 bg-gray-100 text-gray-700 rounded-full"
      >
        {{ item }}
      </button>
    </div>
  </div>

  <!-- 交易分类 -->
  <div class="flex overflow-x-auto space-x-2 mb-6 no-scrollbar">
    <button class="flex-shrink-0 px-4 py-2 bg-primary text-white rounded-lg">全部交易</button>
    <button
      v-for="(item, index) in transactionTypes"
      :key="index"
      class="flex-shrink-0 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200"
    >
      {{ item }}
    </button>
  </div>

  <!-- 交易列表 -->
  <div class="bg-white rounded-xl shadow-sm p-4 mb-6">
    <!-- 日期分组 -->
    <div class="mb-4">
      <div class="flex justify-between items-center py-2">
        <p class="text-sm font-medium">2025年3月19日</p>
        <p class="text-sm text-gray-500">支出 ¥852.36 / 收入 ¥5,000.00</p>
      </div>

      <!-- 交易项 -->
      <div
        v-for="item in data"
        :key="item.id"
        class="p-3 rounded-lg flex justify-between items-center border-b border-gray-100"
      >
        <div class="flex items-center">
          <div class="w-10 h-10 rounded-full bg-red-100 flex items-center justify-center">
            <Icon123 class="text-red-500" />
          </div>
          <div class="ml-3">
            <p class="font-medium">{{ item.name }}</p>
            <p class="text-xs text-gray-500">{{ item.time }} · {{ item.category }}</p>
          </div>
        </div>
        <p class="font-medium">-¥{{ item.amount }}</p>
      </div>
    </div>
  </div>
</template>

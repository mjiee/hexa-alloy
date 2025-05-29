<script setup lang="ts">
import { ref, computed } from "vue";
import { IconPlus } from "@tabler/icons-vue";

const assetTypes = ref([
  { id: 1, name: "现金类" },
  { id: 2, name: "股票类" },
  { id: 3, name: "基金类" },
  { id: 4, name: "房产类" },
  { id: 5, name: "债券类" },
  { id: 6, name: "其它" },
]);

const assets = ref([
  { id: 1, name: "招商银行储蓄账户", type: 1, balance: 125362.45, change: 120.56, remark: "活期存款 | 0.3%" },
  { id: 2, name: "工商银行定期存款", type: 1, balance: 300000, change: 0, remark: "定期三年 | 2.75%" },
  { id: 3, name: "余额宝", type: 1, balance: 85632.78, change: 0, remark: "货币基金 | 2.1%" },
  { id: 4, name: "贵州茅台(600519)", type: 2, balance: 15000, change: 2.34, remark: "信上海证券交易所" },
  { id: 5, name: "中国平安(000001)", type: 2, balance: 20000, change: -1.23, remark: "香港证券交易所" },
  { id: 6, name: "北京海淀区三居室", type: 4, balance: 4200000, change: 5.2, remark: "出租中 | 80平方米" },
  { id: 7, name: "上海浦东新区", type: 4, balance: 1800000, change: -2.5, remark: "可售 | 120平方米" },
]);

const filteredAssetTypes = computed(() => {
  return assetTypes.value.filter((item) => assets.value.some((data) => data.type === item.id));
});
</script>

<template>
  <!-- 资产类型导航 -->
  <div class="flex overflow-x-auto mb-6 no-scrollbar">
    <button class="px-4 py-2 bg-gray-500 text-white rounded-lg mr-3 whitespace-nowrap">全部资产</button>
    <button
      v-for="item in assetTypes"
      :key="item.id"
      class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg mr-3 whitespace-nowrap hover:bg-gray-200"
    >
      {{ item.name }}
    </button>
  </div>

  <!-- 资产列表 -->
  <template v-for="item in filteredAssetTypes" :key="item.id">
    <h2 class="text-lg font-semibold mb-4">{{ item.name }}</h2>
    <div class="space-y-4 mb-8">
      <div
        v-for="asset in assets.filter((data) => data.type == item.id)"
        :key="asset.id"
        class="asset-card bg-white rounded-xl shadow-sm p-4 flex justify-between items-center"
      >
        <div class="flex items-center">
          <div class="w-12 h-12 rounded-full bg-blue-100 flex items-center justify-center">
            <IconPlus class="text-primary" />
          </div>
          <div class="ml-4">
            <h3 class="font-medium">{{ asset.name }}</h3>
            <p class="text-sm text-gray-500">{{ asset.remark }}</p>
          </div>
        </div>
        <div class="text-right">
          <p class="font-bold">¥{{ asset.balance }}</p>
          <p class="text-xs text-green-500">{{ asset.change }} (今日)</p>
        </div>
      </div>
    </div>
  </template>

  <!-- 添加资产按钮 -->
  <div class="fixed bottom-20 md:bottom-8 right-8 z-10">
    <button
      class="w-14 h-14 bg-white rounded-full shadow-lg flex items-center justify-center hover:bg-blue-600 transition"
    >
      <IconPlus class="text-grey text-xl" />
    </button>
  </div>
</template>

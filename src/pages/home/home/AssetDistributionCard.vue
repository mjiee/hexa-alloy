<script setup lang="ts">
import { ref } from "vue";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { TreemapChart } from "echarts/charts";
import type { TreemapSeriesOption } from "echarts/charts";
import type { ComposeOption } from "echarts/core";
import type { CallbackDataParams } from "echarts/types/dist/shared";
import VChart from "vue-echarts";

type ECOption = ComposeOption<TreemapSeriesOption>;

use([CanvasRenderer, TreemapChart]);

const option = ref<ECOption>({
  tooltip: {
    show: true,
    formatter: function (params: CallbackDataParams) {
      return `${params.name}: ${params.value}¥`;
    },
  },
  series: [
    {
      type: "treemap",
      roam: false,
      nodeClick: false,
      label: {
        show: true,
        formatter: "{b}\n{c}¥",
      },
      breadcrumb: {
        show: false,
      },
      data: [
        { name: "建行卡", value: 1000 },
        { name: "农行卡", value: 300 },
        { name: "支付宝", value: 25 },
        { name: "微信钱包", value: 8 },
      ],
    },
  ],
});
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 mb-6 h-[400px]">
    <div class="flex justify-between items-center mb-4">
      <h3 class="font-medium">资产分布</h3>
      <button class="text-blue-600 text-sm hover:underline">查看详情</button>
    </div>

    <VChart class="w-full h-full" :option="option" autoresize />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const news = ref([
  {
    id: 1,
    title: "阿里巴巴宣布新一轮云计算战略投资",
    content: "阿里巴巴宣布将在未来三年投资500亿元发展云计算业务，重点发力人工智能和大数据领域。",
    source: "财经日报",
    time: "2小时前",
  },
  {
    id: 2,
    title: "阿里巴巴Q4财报超预期，电商业务增长强劲",
    content: "阿里巴巴公布2024财年第四季度财报，营收同比增长12%，电商业务表现强劲，超出市场预期。",
    source: "华尔街见闻",
    time: "1天前",
  },
  {
    id: 3,
    title: "阿里巴巴与美团达成战略合作，外卖市场格局或生变",
    content: "阿里巴巴旗下饿了么与美团宣布战略合作，双方将在配送网络和商家资源上进行共享，市场竞争格局将迎来新变化。",
    source: "科技日报",
    time: "3小时前",
  },
]);
</script>

<template>
  <div class="bg-white rounded-xl shadow-md p-4 mb-6">
    <h2 class="text-lg font-semibold mb-4">相关新闻</h2>
    <div class="space-y-4">
      <div v-for="item in news" :key="item.id" class="flex">
        <div class="w-24 h-16 bg-gray-200 rounded-md overflow-hidden flex-shrink-0">
          <div class="w-full h-full bg-gradient-to-r from-gray-300 to-gray-200 pulse"></div>
        </div>
        <div class="ml-4 flex-1">
          <h3 class="text-md font-medium line-clamp-1">{{ item.title }}</h3>
          <p class="text-sm text-gray-500 line-clamp-2 mt-1">
            {{ item.content }}
          </p>
          <div class="flex items-center mt-1">
            <span class="text-xs text-gray-400">{{ item.source }}</span>
            <span class="text-xs text-gray-400 mx-2">•</span>
            <span class="text-xs text-gray-400">{{ item.time }}</span>
          </div>
        </div>
      </div>
    </div>
    <button class="w-full mt-4 text-primary text-sm font-medium py-2 hover:bg-blue-50 rounded-lg transition">
      查看更多新闻
    </button>
  </div>
</template>

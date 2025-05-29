<script setup lang="ts">
import { IconStar, IconStarHalf } from "@tabler/icons-vue";
</script>

<template>
  <div class="bg-white rounded-xl shadow-md p-4 mb-6">
    <h2 class="text-lg font-semibold mb-4">分析师预测</h2>
    <div class="flex flex-col md:flex-row gap-6">
      <div class="flex-1">
        <div class="flex items-center justify-between mb-3">
          <div>
            <p class="text-sm text-gray-500">目标价格</p>
            <p class="text-xl font-bold">$115.30</p>
          </div>
          <div class="flex items-center">
            <span class="text-sm stock-up mr-2"> <i class="fas fa-arrow-up"></i> 32.16% </span>
            <span class="text-xs text-gray-500">潜在上涨空间</span>
          </div>
        </div>
        <div class="w-full bg-gray-100 rounded-lg h-8 relative">
          <div class="absolute left-0 right-0 top-0 bottom-0 flex items-center justify-between px-3">
            <span class="text-xs font-medium">$65.32</span>
            <span class="text-xs font-medium">$112.74</span>
          </div>
          <div class="absolute h-full bg-gray-200 rounded-lg" style="width: 50%; left: 25%"></div>
          <div class="absolute w-2 h-8 bg-primary rounded-full" style="left: 40%"></div>
          <div class="absolute w-2 h-8 bg-green-500 rounded-full" style="left: 75%"></div>
        </div>
        <div class="flex justify-between text-xs text-gray-500 mt-1">
          <span>52周最低</span>
          <span>当前价格</span>
          <span>目标价格</span>
          <span>52周最高</span>
        </div>
      </div>
      <div class="flex-1">
        <p class="text-sm text-gray-500 mb-3">分析师评级</p>
        <div class="flex items-center mb-4">
          <div class="w-28">
            <div class="flex items-center">
              <span class="text-lg font-bold">4.2</span>
              <div class="ml-2">
                <div class="flex text-yellow-400 text-sm">
                  <IconStar />
                  <IconStar />
                  <IconStar />
                  <IconStar />
                  <IconStarHalf />
                </div>
                <p class="text-xs text-gray-500">32个评级</p>
              </div>
            </div>
          </div>
          <div class="flex-1">
            <div class="flex items-center mb-1">
              <span class="text-xs w-16">强烈买入</span>
              <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                <div class="bg-green-500 h-1.5 rounded-full" style="width: 45%"></div>
              </div>
              <span class="text-xs ml-2">45%</span>
            </div>
            <div class="flex items-center mb-1">
              <span class="text-xs w-16">买入</span>
              <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                <div class="bg-green-300 h-1.5 rounded-full" style="width: 35%"></div>
              </div>
              <span class="text-xs ml-2">35%</span>
            </div>
            <div class="flex items-center mb-1">
              <span class="text-xs w-16">持有</span>
              <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                <div class="bg-yellow-400 h-1.5 rounded-full" style="width: 15%"></div>
              </div>
              <span class="text-xs ml-2">15%</span>
            </div>
            <div class="flex items-center mb-1">
              <span class="text-xs w-16">卖出</span>
              <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                <div class="bg-red-300 h-1.5 rounded-full" style="width: 5%"></div>
              </div>
              <span class="text-xs ml-2">5%</span>
            </div>
            <div class="flex items-center">
              <span class="text-xs w-16">强烈卖出</span>
              <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                <div class="bg-red-500 h-1.5 rounded-full" style="width: 0%"></div>
              </div>
              <span class="text-xs ml-2">0%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

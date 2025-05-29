<script setup lang="ts">
import HomeHeader from "./home/HomeHeader.vue";
import AssetViewCard from "./home/AssetViewCard.vue";
import DailyIncomeCard from "./home/DailyIncomeCard.vue";
import AssetChangeCard from "./home/AssetChangeCard.vue";
import AssetDistributionCard from "./home/AssetDistributionCard.vue";
</script>

<template>
  <HomeHeader />

  <!-- 主体内容 -->
  <div class="flex-1 p-4 overflow-y-auto">
    <h2 class="text-2xl font-bold mb-6">资产总览</h2>

    <!-- 资产概览卡片 -->
    <AssetViewCard />
    <DailyIncomeCard />
    <AssetChangeCard />
    <AssetDistributionCard />
  </div>
</template>

<script setup lang="ts">
import MarketHeader from "./market/MarketHeader.vue";
import StockList from "./market/StockList.vue";
</script>

<template>
  <MarketHeader />
  <div class="flex-1 p-4 overflow-y-auto">
    <StockList />
  </div>
</template>

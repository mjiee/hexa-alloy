<script setup lang="ts">
import StockHeader from "./stock/StockHeader.vue";
import StockChart from "./stock/StockChart.vue";
import StockIndicators from "./stock/StockIndicators.vue";
import FinancialStatements from "./stock/FinancialStatements.vue";
import StockForecast from "./stock/StockForecast.vue";
import CompanyNews from "./stock/CompanyNews.vue";
</script>

<template>
  <div class="flex-1 p-4 overflow-y-auto">
    <StockHeader />
    <StockChart />
    <StockIndicators />
    <FinancialStatements />
    <StockForecast />
    <CompanyNews />
  </div>
</template>

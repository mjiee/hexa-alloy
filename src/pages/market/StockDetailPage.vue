<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { MainLayout, StockHeader } from "@/components";
import { getStock, StockDetail } from "@/services/stockApi";
import DetailCard from "./detail/DetailCard.vue";
import DataCard from "./detail/DataCard.vue";

const route = useRoute();
const stock = ref<StockDetail>();
const loading = ref(true);

const getStockDetail = async () => {
  loading.value = true;
  const resp = await getStock({ ticker: String(route.params.ticker) });
  if (resp) stock.value = resp;
  loading.value = false;
};

onMounted(() => {
  getStockDetail();
});
</script>

<template>
  <MainLayout>
    <template #header>
      <StockHeader />
    </template>

    <div class="max-w-7xl mx-auto px-4 py-6 space-y-6">
      <!-- loading -->
      <div v-if="loading" class="flex justify-center items-center py-20">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>

      <template v-if="stock">
        <DetailCard :stock="stock" />
        <DataCard :ticker="stock.ticker" />
      </template>
    </div>
  </MainLayout>
</template>

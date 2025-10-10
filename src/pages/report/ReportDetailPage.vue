<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { MainLayout, StockHeader } from "@/components";
import { getReport, ReportDetail } from "@/services/reportApi";
import { IconReport } from "@tabler/icons-vue";
import NewsCard from "./detail/NewsCard.vue";
import ReportCard from "./detail/ReportCard.vue";

const route = useRoute();
const report = ref<ReportDetail>();
const loading = ref(true);

const getReportDetail = async () => {
  loading.value = true;
  const resp = await getReport({ id: Number(route.params.id) });
  if (resp) report.value = resp;
  loading.value = false;
};

onMounted(() => {
  getReportDetail();
});
</script>

<template>
  <MainLayout>
    <template #header>
      <StockHeader :stock="report?.stock" />
    </template>

    <div class="max-w-7xl mx-auto px-4 py-6 space-y-6">
      <!-- loading -->
      <div v-if="loading" class="flex justify-center items-center py-20">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>

      <template v-else-if="report">
        <ReportCard :report="report" />
        <NewsCard v-if="report.news" :news="report.news" />
      </template>

      <!-- no data -->
      <div v-else class="text-center py-20">
        <IconReport class="w-16 h-16 mx-auto text-gray-400 mb-4" />
        <p class="text-gray-600 text-lg">未找到相关资讯</p>
      </div>
    </div>
  </MainLayout>
</template>

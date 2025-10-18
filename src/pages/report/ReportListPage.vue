<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from "vue";
import { MainLayout, Pagination, ReportWinRateChart, ReportCard } from "@/components";
import { getReportWinRate, queryReports, ReportDetail, ReportWinRate } from "@/services/reportApi";
import SearchHeader from "./list/SearchHeader.vue";
import { useQueryReportsStore } from "@/stores/queryReportsStore";

const reports = ref<ReportDetail[]>([]);
const winRate = ref<ReportWinRate>();
const total = ref(0);
const queryStore = useQueryReportsStore();

const fetchReports = async () => {
  if (queryStore.searchParams.page === 1) {
    const [winResp, reportsResp] = await Promise.all([
      getReportWinRate(queryStore.searchParams),
      queryReports(queryStore.searchParams),
    ]);
    if (winResp) winRate.value = winResp;
    if (reportsResp) {
      reports.value = reportsResp.data ?? [];
      total.value = reportsResp.total;
    }
  } else {
    const reportsResp = await queryReports(queryStore.searchParams);
    if (reportsResp) {
      reports.value = reportsResp.data ?? [];
      total.value = reportsResp.total;
    }
  }
};

const handlePageChange = (page: number) => {
  queryStore.setPage(page);
  fetchReports();
};

onMounted(() => {
  document.body.dataset.disableSwipeBack = "true";
  fetchReports();
});

onUnmounted(() => delete document.body.dataset.disableSwipeBack);
</script>

<template>
  <MainLayout>
    <template #header>
      <SearchHeader :onSearch="fetchReports" />
    </template>

    <div class="px-4 max-w-7xl mx-auto">
      <div class="bg-white rounded-2xl shadow p-4">
        <ReportWinRateChart v-if="winRate" :winRate="winRate" />
      </div>

      <div class="py-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <ReportCard v-for="report in reports" :key="report.id" :report="report" />
      </div>

      <Pagination :total="total" :currentPage="queryStore.searchParams.page ?? 0" :onChange="handlePageChange" />
    </div>
  </MainLayout>
</template>

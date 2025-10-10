<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { MainLayout, Pagination } from "@/components";
import { getReportWinRate, queryReports, QueryReportsRequest, ReportDetail, ReportWinRate } from "@/services/reportApi";
import { defaultPage } from "@/utils/constants";
import SearchHeader from "./list/SearchHeader.vue";
import ReportItem from "./list/ReportItem.vue";
import WinRateCard from "./list/WinRateCard.vue";
import { useQueryReportsStore } from "@/stores/queryReportsStore";

const params = ref<QueryReportsRequest>({ ...defaultPage });
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
    const reportsResp = await queryReports(params.value);
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
  fetchReports();
});
</script>

<template>
  <MainLayout>
    <template #header>
      <SearchHeader :onSearch="fetchReports" v-model="params" />
    </template>

    <div class="px-4 max-w-7xl mx-auto">
      <div class="bg-white rounded-2xl shadow p-4">
        <WinRateCard v-if="winRate" :winRate="winRate" />
      </div>

      <div class="py-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <ReportItem v-for="report in reports" :key="report.id" :report="report" />
      </div>

      <Pagination :total="total" :currentPage="queryStore.searchParams.page ?? 0" :onChange="handlePageChange" />
    </div>
  </MainLayout>
</template>

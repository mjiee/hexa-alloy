<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { ReportWinRateChart, ReportCard, Pagination } from "@/components";
import { getReportWinRate, queryReports, QueryReportsRequest, ReportDetail, ReportWinRate } from "@/services/reportApi";
import { defaultPage } from "@/utils/constants";

const props = defineProps<{ ticker: string }>();

const params = ref<QueryReportsRequest>({ ...defaultPage, stock_ticker: props.ticker });
const reports = ref<ReportDetail[]>([]);
const winRate = ref<ReportWinRate>();
const total = ref(0);

const fetchReports = async () => {
  if (params.value.page && params.value?.page > 1) {
    const reportsResp = await queryReports(params.value);
    if (reportsResp) {
      reports.value = reportsResp.data ?? [];
      total.value = reportsResp.total;
    }

    return;
  }

  const [winResp, reportsResp] = await Promise.all([getReportWinRate(params.value), queryReports(params.value)]);
  if (winResp) winRate.value = winResp;
  if (reportsResp) {
    reports.value = reportsResp.data ?? [];
    total.value = reportsResp.total;
  }
};

const handlePageChange = (page: number) => {
  params.value = { ...params.value, page };
  fetchReports();
};

onMounted(() => fetchReports());
</script>

<template>
  <div class="bg-white rounded-2xl shadow p-4">
    <ReportWinRateChart v-if="winRate" :winRate="winRate" />
  </div>

  <div class="py-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    <ReportCard v-for="report in reports" :key="report.id" :report="report" />
  </div>

  <Pagination :total="total" :currentPage="params.page ?? 0" :onChange="handlePageChange" />
</template>

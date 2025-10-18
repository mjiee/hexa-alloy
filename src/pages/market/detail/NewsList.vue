<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { NewsDetail, QueryNewsRequest, queryNews } from "@/services/newsApi";
import { NewsCard, Pagination } from "@/components";
import { defaultPage } from "@/utils/constants";

const props = defineProps<{ ticker: string }>();
const newsData = ref<NewsDetail[]>([]);
const total = ref(0);
const params = ref<QueryNewsRequest>({ ...defaultPage, stock_ticker: props.ticker });

const fetchNews = async () => {
  const resp = await queryNews(params.value);

  if (!resp) return;

  newsData.value = resp.data ?? [];
  total.value = resp.total;
};

const handlePageChange = (page: number) => {
  params.value = { ...params.value, page };
  fetchNews();
};

onMounted(() => fetchNews());
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    <NewsCard v-for="news in newsData" :news="news" :key="news.id" />
  </div>
  <Pagination :total="total" :currentPage="params.page ?? 0" :onChange="handlePageChange" />
</template>

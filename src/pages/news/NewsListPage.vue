<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { NewsDetail, queryNews } from "@/services/newsApi";
import { MainLayout, Pagination } from "@/components";
import SearchHeader from "./list/SearchHeader.vue";
import NewsItem from "./list/NewsItem.vue";
import { useQueryNewsStore } from "@/stores/queryNewsStore";

const newsData = ref<NewsDetail[]>([]);
const total = ref(0);
const queryStore = useQueryNewsStore();

const fetchNews = async () => {
  const resp = await queryNews(queryStore.searchParams);

  if (!resp) return;

  newsData.value = resp.data ?? [];
  total.value = resp.total;
};

const handlePageChange = (page: number) => {
  queryStore.setPage(page);
  fetchNews();
};

onMounted(() => fetchNews());
</script>

<template>
  <MainLayout>
    <template #header>
      <SearchHeader :onSearch="fetchNews" />
    </template>

    <div class="p-4 max-w-7xl mx-auto">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <NewsItem v-for="news in newsData" :news="news" :key="news.id" />
      </div>
      <Pagination :total="total" :currentPage="queryStore.searchParams.page ?? 0" :onChange="handlePageChange" />
    </div>
  </MainLayout>
</template>

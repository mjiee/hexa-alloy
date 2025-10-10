<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { NewsDetail, getNews } from "@/services/newsApi";
import { MainLayout, StockHeader } from "@/components";
import { IconClock, IconExternalLink, IconNews } from "@tabler/icons-vue";
import { marketImpactOptions } from "@/services/reportApi";

import { IconReport } from "@tabler/icons-vue";

const route = useRoute();
const news = ref<NewsDetail | undefined>();
const loading = ref(true);

const getNewsDetail = async () => {
  loading.value = true;
  const resp = await getNews({ id: Number(route.params.id) });
  if (resp) news.value = resp;
  loading.value = false;
};

onMounted(() => getNewsDetail());
</script>

<template>
  <MainLayout>
    <template #header>
      <StockHeader :stock="news?.stock" />
    </template>

    <div class="max-w-7xl mx-auto px-4 py-6 space-y-6">
      <!-- loading -->
      <div v-if="loading" class="flex justify-center items-center py-20">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>

      <div v-else-if="news" class="bg-white rounded-lg shadow-md p-4">
        <!-- news meta and title -->
        <div class="border-b pb-4 mb-6">
          <h1 class="text-xl font-bold text-gray-900 mb-4">{{ news.title }}</h1>

          <div class="flex flex-wrap items-center gap-4 text-sm text-gray-600">
            <span v-if="news.source" class="flex items-center gap-1">
              <IconNews class="w-4 h-4" />
              {{ news.source }}
            </span>

            <span v-if="news.publish_time" class="flex items-center gap-1">
              <IconClock class="w-4 h-4" />
              {{ news.publish_time }}
            </span>

            <span v-if="news.category" class="bg-blue-100 text-blue-700 rounded-md">
              {{ news.category }}
            </span>

            <span v-if="news.market_impact">
              市场影响: {{ marketImpactOptions.find((v) => v.value == news?.market_impact)?.label || "中性" }}
            </span>
          </div>
        </div>

        <!-- news content -->
        <div class="prose prose-lg max-w-none">
          <div v-if="news.contents && news.contents.length" class="space-y-4">
            <div v-for="(paragraph, index) in news.contents" :key="index" class="text-gray-700 leading-relaxed">
              <div v-html="paragraph"></div>
            </div>
          </div>
          <div v-else class="text-gray-500 italic">暂无详细内容</div>
        </div>

        <!--  -->
        <div v-if="news.link" class="mt-6 pt-6 border-t">
          <a
            :href="news.link"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-2 text-blue-600 hover:text-blue-700 font-medium transition-colors"
          >
            查看原文
            <IconExternalLink class="w-4 h-4" />
          </a>
        </div>
      </div>

      <!-- no data -->
      <div v-else class="text-center py-20">
        <IconReport class="w-16 h-16 mx-auto text-gray-400 mb-4" />
        <p class="text-gray-600 text-lg">未找到相关资讯</p>
      </div>
    </div>
  </MainLayout>
</template>

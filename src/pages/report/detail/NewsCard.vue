<script lang="ts" setup>
import { NewsDetail } from "@/services/newsApi";
import { IconExternalLink } from "@tabler/icons-vue";

const { news } = defineProps<{ news: NewsDetail }>();
</script>

<template>
  <div class="bg-white rounded-lg shadow-md p-4">
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
</template>

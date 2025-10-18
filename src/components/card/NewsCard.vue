<script lang="ts" setup>
import { NewsDetail } from "@/services/newsApi";
import { marketImpactOptions, categoryOptions } from "@/services/reportApi";

const props = defineProps<{ news: NewsDetail }>();
</script>

<template>
  <router-link :to="'/news/' + props.news.id">
    <div class="bg-white rounded-2xl shadow p-4 space-y-3">
      <div>
        <h3 class="text-base font-semibold text-gray-900 leading-tight">{{ props.news.title }}</h3>
        <div class="flex items-center text-xs text-gray-500 mt-1 space-x-2">
          <span>来源：{{ props.news.source }}</span>
          <span>·</span>
          <span>{{ props.news.publish_time }}</span>
        </div>
      </div>
      <p v-if="props.news.contents" class="text-sm text-gray-700 leading-snug line-clamp-3">
        {{ props.news.contents[0] }}
      </p>
      <div v-if="props.news.stock" class="flex flex-wrap gap-2">
        <span class="px-2 py-1 text-xs rounded-full bg-blue-100 font-medium">
          {{ props.news.stock.name }}
        </span>
        <span class="px-2 py-1 text-xs rounded-full border border-blue-500 text-blue-600">
          {{ props.news.stock.ticker }}
        </span>
        <span class="px-2 py-1 text-xs rounded-full bg-green-100 text-green-700 font-medium">
          {{ marketImpactOptions.find((v) => v.value == props.news.market_impact)?.label || "中性" }}
        </span>
        <span v-if="props.news.category" class="px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-700 font-medium">
          {{ categoryOptions.find((v) => v.value == props.news.category)?.label || "" }}
        </span>
      </div>
    </div>
  </router-link>
</template>

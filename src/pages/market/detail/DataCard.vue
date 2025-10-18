<script lang="ts" setup>
import { ref } from "vue";
import NewsList from "./NewsList.vue";
import ReportList from "./ReportList.vue";

const props = defineProps<{ ticker: string }>();
const option = ref("report");

const cardOptions = [
  { label: "报表", value: "report" },
  { label: "资讯", value: "news" },
];
</script>

<template>
  <div class="flex gap-2">
    <button
      v-for="item in cardOptions"
      :key="item.value"
      @click="option = item.value"
      :class="[
        'px-3 py-1 text-sm rounded-md transition-colors',
        option === item.value ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
      ]"
    >
      {{ item.label }}
    </button>
  </div>
  <div v-if="option === 'report'">
    <ReportList :ticker="props.ticker" />
  </div>
  <div v-else>
    <NewsList :ticker="props.ticker" />
  </div>
</template>

<script lang="ts" setup>
import { IconArrowLeft, IconArrowRight } from "@tabler/icons-vue";
import { computed } from "vue";

interface Props {
  currentPage: number;
  total: number;
  onChange: (page: number) => void;
}

const props = defineProps<Props>();

const totalPages = computed(() => Math.ceil(props.total / 25));

const handlePageChange = (page: number) => {
  if (page < 1 || page > totalPages.value) return;
  props.onChange(page);
};
</script>

<template>
  <div class="flex items-center justify-center gap-4 py-4">
    <button
      @click="handlePageChange(currentPage - 1)"
      :disabled="currentPage === 1"
      class="px-4 py-2 border rounded hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <IconArrowLeft class="w-6 h-6" />
    </button>

    <span class="text-gray-700"> {{ currentPage }} / {{ totalPages }} </span>

    <button
      @click="handlePageChange(currentPage + 1)"
      :disabled="currentPage === totalPages"
      class="px-4 py-2 border rounded hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <IconArrowRight class="w-6 h-6" />
    </button>
  </div>
</template>

<script lang="ts" setup>
import { useRouter } from "vue-router";
import { IconArrowLeft } from "@tabler/icons-vue";

const router = useRouter();
</script>

<template>
  <header class="bg-white top-0 z-50 shadow-md w-full relative">
    <div class="px-4 pb-1 flex items-center gap-4">
      <IconArrowLeft @click="router.back()" class="w-6 h-6 text-blue-600" />
      <slot></slot>
    </div>
  </header>
</template>

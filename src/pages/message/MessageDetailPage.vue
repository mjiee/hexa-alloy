<script setup lang="ts">
import DetailHeader from "./detail/DetailHeader.vue";
import MessageDetail from "./detail/MessageDetail.vue";
</script>

<template>
  <DetailHeader />
  <div class="flex-1 p-4 overflow-y-auto">
    <MessageDetail />
  </div>
</template>

<script setup lang="ts">
import ListHeader from "./list/ListHeader.vue";
import MessageList from "./list/MessageList.vue";
</script>

<template>
  <ListHeader />
  <div class="flex-1 p-4 overflow-y-auto">
    <MessageList />
  </div>
</template>

<script setup lang="ts">
import TransactionsHeader from "./transactions/TransactionsHeader.vue";
import TransactionsViewCard from "./transactions/TransactionsViewCard.vue";
import TransactionsList from "./transactions/TransactionsList.vue";
</script>

<template>
  <TransactionsHeader />
  <div class="flex-1 p-4 overflow-y-auto">
    <TransactionsViewCard />
    <TransactionsList />
  </div>
</template>

<script lang="ts" setup>
import { StockDetail } from "@/services/stockApi";
import BackHeader from "./BackHeader.vue";

const { stock } = defineProps<{ stock?: StockDetail }>();
</script>

<template>
  <BackHeader>
    <RouterLink v-if="stock" :to="'/stock/' + stock.ticker">
      <div class="flex items-end gap-1">
        <h1 class="text-xl font-bold text-gray-900">{{ stock.name }}</h1>
        <p class="text-xl">({{ stock.ticker }})</p>
        <p
          v-if="stock.quote && stock.quote.change_percent"
          :class="stock.quote.change_percent > 0 ? 'text-green-500' : 'text-red-500'"
        >
          {{ stock.quote.current_price }} {{ stock.quote.change_percent }}%
        </p>
      </div>
    </RouterLink>
  </BackHeader>
</template>

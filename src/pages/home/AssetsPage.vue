<script setup lang="ts">
import AssetsHeader from "./assets/AssetsHeader.vue";
import AssetViewCard from "./assets/AssetViewCard.vue";
import AssetList from "./assets/AssetList.vue";
</script>

<template>
  <AssetsHeader />
  <div class="flex-1 p-4 overflow-y-auto">
    <AssetViewCard />
    <AssetList />
  </div>
</template>

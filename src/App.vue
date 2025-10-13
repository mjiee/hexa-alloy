<script lang="ts" setup>
import { Toaster } from "vue-sonner";
import { useSwipeBack } from "./utils/useSwipeBack";
import "vue-sonner/style.css";

const { transform, transition } = useSwipeBack();
</script>

<template>
  <Toaster />
  <div
    class="app-container"
    :style="{
      transform: transform,
      transition: transition,
    }"
  >
    <RouterView />
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
  will-change: transform;
}
</style>

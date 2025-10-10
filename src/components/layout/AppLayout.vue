<script lang="ts" setup>
import SideNavbar from "./SideNavbar.vue";
import BottomNavbar from "./BottomNavbar.vue";
</script>

<template>
  <div class="flex h-screen overflow-hidden">
    <!-- desktop side navbar -->
    <aside class="hidden md:flex md:w-64 md:flex-shrink-0">
      <SideNavbar />
    </aside>

    <!-- main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <main class="flex-1 overflow-hidden md:pb-0 pb-safe-bottom pt-safe-top">
        <RouterView />
      </main>

      <!-- mobild buttom navbar -->
      <BottomNavbar />
    </div>
  </div>
</template>

<style scoped>
.pb-safe-bottom {
  padding-bottom: calc(4rem + env(safe-area-inset-bottom, 4px));
}

.pt-safe-top {
  padding-top: env(safe-area-inset-top, 0px);
}

@media (min-width: 768px) {
  .pb-safe-bottom {
    padding-bottom: 0;
  }
}
</style>

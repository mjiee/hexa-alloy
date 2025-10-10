<template>
  <div class="h-full flex flex-col overflow-hidden">
    <div class="flex-shrink-0">
      <slot name="header"></slot>
    </div>

    <div class="flex-1 overflow-y-auto">
      <slot></slot>
    </div>
  </div>
</template>

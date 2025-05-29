<script setup lang="ts">
import { ref } from "vue";
import type { Component } from "vue";
import { IconChartCandle, IconHome, IconMessage, IconUser } from "@tabler/icons-vue";

interface NavItem {
  id: string;
  label: string;
  icon: Component;
}

const activeTab = ref<string>("/");

const navItems: NavItem[] = [
  { id: "/", label: "首页", icon: IconHome },
  { id: "/market", label: "市场", icon: IconChartCandle },
  { id: "/message", label: "消息", icon: IconMessage },
  { id: "/profile", label: "我的", icon: IconUser },
];
</script>

<template>
  <!-- 左侧导航栏 -->
  <div class="flex flex-col md:flex-row min-h-screen">
    <aside class="hidden md:flex flex-col w-64 bg-white shadow-lg">
      <div class="p-4 border-b border-slate-200">
        <h1 class="text-2xl font-bold text-blue-600">HEXA ALLOY</h1>
        <p class="text-sm text-slate-500">专业金融管理平台</p>
      </div>
      <nav class="flex flex-col flex-1 p-4 space-y-2 overflow-y-auto">
        <router-link
          v-for="item in navItems"
          :key="item.id"
          :to="item.id"
          @click="activeTab = item.id"
          class="nav-item flex items-center p-3 rounded-lg"
          :class="activeTab === item.id ? 'bg-blue-50 text-blue-600' : 'text-slate-700 hover:bg-slate-100'"
        >
          <component
            :is="item.icon"
            class="w-6 transition-transform duration-300"
            :class="activeTab === item.id ? 'text-blue-600' : 'text-gray-500'"
          />
          <span class="ml-3" :class="{ 'font-medium': activeTab === item.id }">{{ item.label }}</span>
        </router-link>
      </nav>
      <div class="p-4 border-t border-slate-200">
        <div class="flex items-center">
          <img
            src="https://img.icons8.com/?size=100&id=80976&format=png&color=000000"
            alt="用户头像"
            class="w-10 h-10 rounded-full"
          />
          <div class="ml-3">
            <p class="font-medium">小王</p>
            <p class="text-xs text-slate-500">VIP会员</p>
          </div>
        </div>
      </div>
    </aside>

    <!-- 内容区域 -->
    <main class="flex-1 flex flex-col">
      <RouterView />
    </main>
  </div>

  <!-- 底部导航栏 -->
  <nav class="bg-white shadow-lg flex flex-row md:hidden fixed bottom-0 left-0 right-0 h-16 z-10">
    <router-link
      v-for="item in navItems"
      :key="item.id"
      :to="item.id"
      @click="activeTab = item.id"
      class="flex items-center justify-center flex-1 flex-col py-2 transition-all duration-300 ease-in-out"
      :class="activeTab === item.id ? 'text-blue-600' : 'text-gray-500 hover:text-blue-400'"
    >
      <div
        class="transition-transform duration-300 ease-in-out"
        :class="activeTab === item.id ? 'scale-110' : 'scale-100'"
      >
        <component :is="item.icon" class="w-6 h-6" />
      </div>
      <span
        class="mt-1 text-xs font-medium transition-all duration-300 ease-in-out"
        :class="activeTab === item.id ? 'opacity-100' : 'opacity-70'"
      >
        {{ item.label }}
      </span>
    </router-link>
  </nav>
</template>

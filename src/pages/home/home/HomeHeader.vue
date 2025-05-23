<!-- 顶部搜索栏 -->
<template>
  <header class="bg-white shadow-sm p-4">
    <div class="flex items-center justify-between">
      <div class="md:hidden flex items-center">
        <h1 class="text-xl font-bold text-blue-600">HEXA ALLOY</h1>
      </div>
      <div class="flex-1 max-w-xl mx-4">
        <div class="relative">
          <input
            type="text"
            placeholder="搜索股票、资产..."
            class="w-full p-2 pl-10 rounded-lg border border-slate-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <i class="fas fa-search absolute left-3 top-3 text-slate-400"></i>
        </div>
      </div>
      <div class="hidden md:flex items-center space-x-4">
        <button class="p-2 rounded-full text-slate-500 hover:bg-slate-100">
          <i class="fas fa-bell"></i>
        </button>
        <button class="p-2 rounded-full text-slate-500 hover:bg-slate-100">
          <i class="fas fa-question-circle"></i>
        </button>
      </div>
    </div>
  </header>
</template>

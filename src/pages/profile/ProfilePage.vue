<template>
  <!-- 设置内容 -->
  <div class="flex-1 p-4 md:p-6 overflow-y-auto">
    <h2 class="text-2xl font-bold mb-6">设置</h2>

    <!-- 账户设置 -->
    <div class="bg-white rounded-xl shadow-sm p-6 mb-6 setting-card">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <i class="fas fa-user-circle text-blue-500 mr-2"></i>
        账户设置
      </h3>
      <div class="space-y-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">个人资料</h4>
            <p class="text-sm text-slate-500">修改您的个人信息与账户照片</p>
          </div>
          <button class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition">
            编辑资料
          </button>
        </div>
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">密码管理</h4>
            <p class="text-sm text-slate-500">更改您的账户密码</p>
          </div>
          <button class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition">
            修改密码
          </button>
        </div>
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">默认货币</h4>
            <p class="text-sm text-slate-500">设置您的默认货币单位</p>
          </div>
          <select
            class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition border border-slate-200"
          >
            <option>人民币 (¥)</option>
            <option>美元 ($)</option>
            <option>欧元 (€)</option>
            <option>英镑 (£)</option>
          </select>
        </div>
      </div>
    </div>

    <!-- 股票分析偏好 -->
    <div class="bg-white rounded-xl shadow-sm p-6 mb-6 setting-card">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <i class="fas fa-chart-line text-blue-500 mr-2"></i>
        股票分析偏好
      </h3>
      <div class="space-y-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">默认市场</h4>
            <p class="text-sm text-slate-500">设置您的默认股票市场</p>
          </div>
          <select
            class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition border border-slate-200"
          >
            <option>上海证券交易所</option>
            <option>深圳证券交易所</option>
            <option>香港交易所</option>
            <option>纽约证券交易所</option>
          </select>
        </div>
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">股票价格提醒</h4>
            <p class="text-sm text-slate-500">关注的股票达到目标价格时提醒我</p>
          </div>
          <label class="toggle-button mt-2 md:mt-0">
            <input type="checkbox" checked />
            <span class="toggle-slider"></span>
          </label>
        </div>
        <div class="flex flex-col md:flex-row md:items-center justify-between">
          <div>
            <h4 class="font-medium">量化分析深度</h4>
            <p class="text-sm text-slate-500">设置股票分析的复杂度</p>
          </div>
          <select
            class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition border border-slate-200"
          >
            <option>基础</option>
            <option>中级</option>
            <option selected>高级</option>
            <option>专业</option>
          </select>
        </div>
      </div>
    </div>

    <!-- 高级设置 -->
    <div class="bg-white rounded-xl shadow-sm p-6 mb-6 setting-card">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <i class="fas fa-sliders-h text-blue-500 mr-2"></i>
        高级设置
      </h3>
      <div class="space-y-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between pb-4 border-b border-slate-100">
          <div>
            <h4 class="font-medium">数据导出</h4>
            <p class="text-sm text-slate-500">导出您的资产和交易数据</p>
          </div>
          <button class="mt-2 md:mt-0 px-4 py-2 bg-slate-100 text-slate-700 rounded-lg hover:bg-slate-200 transition">
            导出数据
          </button>
        </div>
        <div class="flex flex-col md:flex-row md:items-center justify-between">
          <div>
            <h4 class="font-medium text-rose-600">删除账户</h4>
            <p class="text-sm text-slate-500">永久删除您的账户和所有数据</p>
          </div>
          <button class="mt-2 md:mt-0 px-4 py-2 bg-rose-50 text-rose-600 rounded-lg hover:bg-rose-100 transition">
            删除账户
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

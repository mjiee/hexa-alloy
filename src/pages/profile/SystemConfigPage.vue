<script setup>
import { saveConfig, getConfig, ConfigKey } from "@/services/configApi";
import { ref, onMounted } from "vue";

const enabled = ref(false);
const host = ref("");

onMounted(async () => {
  const result = await getConfig({ key: ConfigKey.ForgeClient });
  if (result && result.value) {
    enabled.value = result.value.enable;
    host.value = result.value.host;
  }
});

const onSaveConfig = async () => {
  await saveConfig({
    key: ConfigKey.ForgeClient,
    value: { enable: enabled.value, host: host.value },
  });
};
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm p-6 m-4">
    <h2 class="text-xl font-bold text-gray-800 mb-6">服务域名配置</h2>

    <div class="space-y-6">
      <!-- 开关控件 -->
      <div class="flex items-center justify-between py-2">
        <label class="text-gray-700 font-medium text-base"> 启用自定义域名 </label>
        <button
          @click="enabled = !enabled"
          :class="[
            'relative inline-flex h-8 w-14 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2',
            enabled ? 'bg-indigo-600' : 'bg-gray-300',
          ]"
        >
          <span
            :class="[
              'inline-block h-6 w-6 transform rounded-full bg-white transition-transform shadow-sm',
              enabled ? 'translate-x-7' : 'translate-x-1',
            ]"
          ></span>
        </button>
      </div>

      <!-- 域名输入框 -->
      <div>
        <label class="block text-gray-700 font-medium mb-3 text-base"> 服务域名 </label>
        <input
          type="text"
          v-model="host"
          :disabled="!enabled"
          placeholder="例如: api.example.com"
          :class="[
            'w-full px-4 py-3 text-base border rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-colors',
            enabled
              ? 'border-gray-300 bg-white text-gray-900'
              : 'border-gray-200 bg-gray-100 text-gray-400 cursor-not-allowed',
          ]"
        />
        <p class="mt-3 text-sm text-gray-500">
          {{ enabled ? "请输入完整的域名地址" : "请先启用自定义域名" }}
        </p>
      </div>

      <!-- 提交按钮 -->
      <button
        @click="onSaveConfig"
        class="w-full bg-indigo-600 hover:bg-indigo-700 active:bg-indigo-800 text-white font-medium py-3 px-4 rounded-xl transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 text-base"
      >
        保存配置
      </button>
    </div>

    <!-- 配置预览 -->
    <div v-if="enabled && host" class="mt-6 mx-4 p-4 bg-indigo-50 rounded-xl border border-indigo-200">
      <h3 class="text-sm font-medium text-indigo-900 mb-2">当前配置</h3>
      <p class="text-sm text-indigo-700"><span class="font-medium">域名:</span> {{ host }}</p>
    </div>
  </div>
</template>

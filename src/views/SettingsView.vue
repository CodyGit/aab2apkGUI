<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSettingsStore } from "../stores/settings";
import { useEnvStore } from "../stores/env";
import SettingsForm from "../components/SettingsForm.vue";

const settingsStore = useSettingsStore();
const envStore = useEnvStore();
const saved = ref(false);

onMounted(async () => {
  await settingsStore.load();
});

async function handleSave() {
  const ok = await settingsStore.save();
  if (ok) {
    saved.value = true;
    await envStore.checkEnvironment();
    setTimeout(() => (saved.value = false), 2000);
  }
}
</script>

<template>
  <div class="settings">
    <h1>设置</h1>
    <SettingsForm
      v-model:bundletool-path="settingsStore.bundletoolJarPath"
    />
    <div class="actions">
      <button class="btn btn-primary" @click="handleSave">保存设置</button>
      <button class="btn btn-secondary" @click="envStore.checkEnvironment">重新检测环境</button>
      <span v-if="saved" class="save-ok">保存成功</span>
    </div>
  </div>
</template>

<style scoped>
.settings {
  max-width: 600px;
}
.settings h1 {
  font-size: 24px;
  margin-bottom: 24px;
}
.actions {
  display: flex;
  gap: 12px;
  align-items: center;
  margin-top: 20px;
}
.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}
.btn-primary {
  background: var(--accent);
  color: white;
}
.btn-secondary {
  background: var(--primary);
  color: var(--text);
}
.save-ok {
  color: var(--success);
  font-size: 13px;
}
</style>

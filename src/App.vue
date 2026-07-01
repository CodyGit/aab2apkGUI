<script setup lang="ts">
import { onMounted, ref, provide } from "vue";
import { useEnvStore } from "./stores/env";
import EnvCheckDialog from "./components/EnvCheckDialog.vue";
import SettingsDialog from "./components/SettingsDialog.vue";

const envStore = useEnvStore();
const showEnvDialog = ref(false);
const showSettingsDialog = ref(false);

provide("showSettings", () => { showSettingsDialog.value = true; });

onMounted(async () => {
  await envStore.checkEnvironment();
  if (!envStore.allOk) {
    showEnvDialog.value = true;
  }
});

function closeTopDialog() {
  if (showSettingsDialog.value) { showSettingsDialog.value = false; return; }
  if (showEnvDialog.value) { showEnvDialog.value = false; return; }
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    closeTopDialog();
    return;
  }
  if (e.key === "Enter") {
    if (!showSettingsDialog.value && !showEnvDialog.value) {
      window.dispatchEvent(new CustomEvent("global-enter"));
    }
  }
}
</script>

<template>
  <div class="app-shell" @keydown="onKeydown" tabindex="-1">
    <header class="app-header">
      <div class="header-left">
        <span class="app-icon">◆</span>
        <span class="app-name">aab2apkGUI</span>
      </div>
      <div class="header-right">
        <span class="env-dot" :class="envStore.javaOk ? 'ok' : 'err'">
          <span class="env-label">Java</span>
        </span>
        <span class="env-dot" :class="envStore.bundletoolOk ? 'ok' : 'err'">
          <span class="env-label">bundletool</span>
        </span>
        <button class="btn-icon" title="设置" @click="showSettingsDialog = true">&#9881;</button>
      </div>
    </header>
    <main>
      <router-view />
    </main>
    <EnvCheckDialog v-if="showEnvDialog" @close="showEnvDialog = false" />
    <SettingsDialog v-if="showSettingsDialog" @close="showSettingsDialog = false" />
  </div>
</template>

<style>
:root {
  --bg: #f5f5f5;
  --surface: #ffffff;
  --primary: #16a34a;
  --accent: #16a34a;
  --link: #2563eb;
  --text: #1f2937;
  --text-dim: #6b7280;
  --border: #e5e7eb;
  --success: #16a34a;
  --error: #dc2626;
  --hover-bg: #f0fdf4;
}
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
  background: var(--bg);
  color: var(--text);
  font-size: 13px;
}
.app-shell {
  outline: none;
}
.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  height: 36px;
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  -webkit-app-region: drag;
}
.header-left {
  display: flex;
  align-items: center;
  gap: 6px;
}
.app-icon {
  font-size: 14px;
  color: var(--primary);
}
.app-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
}
.header-right {
  display: flex;
  align-items: center;
  gap: 10px;
  -webkit-app-region: no-drag;
}
.env-dot {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: 11px;
  cursor: default;
}
.env-dot.ok {
  color: var(--success);
}
.env-dot.err {
  color: var(--error);
}
.env-label {
  color: inherit;
  font-size: 10px;
}
.btn-icon {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
  padding: 2px 4px;
  color: var(--text-dim);
  line-height: 1;
}
.btn-icon:hover {
  color: var(--text);
}
.btn-cancel {
  background: #f97316 !important;
  color: white !important;
}
main {
  padding: 10px 16px;
  max-width: 700px;
  margin: 0 auto;
}
</style>

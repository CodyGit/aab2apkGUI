<script setup lang="ts">
import { inject } from "vue";
import { useEnvStore } from "../stores/env";

const envStore = useEnvStore();
const showSettings = inject<() => void>("showSettings", () => {});

const emit = defineEmits<{
  (e: "close"): void;
}>();

function goToSettings() {
  emit("close");
  showSettings();
}

function handleLater() {
  emit("close");
}
</script>

<template>
  <div class="overlay">
    <div class="dialog">
      <h2>环境检测</h2>
      <div class="checks">
        <div class="check-row">
          <span class="label">Java 运行环境</span>
          <span v-if="envStore.javaOk" class="status ok">已检测到 ({{ envStore.javaVersion }})</span>
          <span v-else class="status err">未检测到</span>
        </div>
        <div v-if="!envStore.javaOk" class="guide">
          <p>请安装 JDK 11 或更高版本：</p>
          <a href="https://adoptium.net/download/" target="_blank" class="link">下载 Adoptium OpenJDK →</a>
          <p class="note">安装后请确保 java 已添加到系统 PATH 环境变量中，然后重启应用。</p>
        </div>

        <div class="check-row">
          <span class="label">bundletool</span>
          <span v-if="envStore.bundletoolOk" class="status ok">已配置</span>
          <span v-else class="status err">未配置</span>
        </div>
        <div v-if="!envStore.bundletoolOk" class="guide">
          <p>请下载 bundletool.jar：</p>
          <a href="https://github.com/google/bundletool/releases" target="_blank" class="link">GitHub Releases →</a>
          <p class="note">下载后请在设置中配置 bundletool.jar 文件路径。</p>
        </div>
      </div>
      <div class="actions">
        <button class="btn btn-cancel" @click="handleLater">稍后处理</button>
        <button class="btn btn-primary" @click="goToSettings">前往设置</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.dialog {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  max-width: 480px;
  width: 90%;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}
.dialog h2 {
  font-size: 18px;
  margin-bottom: 20px;
}
.checks {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.check-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.label {
  font-size: 14px;
}
.status {
  font-size: 13px;
}
.status.ok {
  color: var(--success);
}
.status.err {
  color: var(--error);
}
.guide {
  background: var(--bg);
  border-radius: 6px;
  padding: 10px 12px;
  font-size: 12px;
  line-height: 1.8;
}
.link {
  color: var(--link);
  text-decoration: none;
}
.link:hover {
  text-decoration: underline;
}
.note {
  color: var(--text-dim);
  font-size: 11px;
  margin-top: 4px;
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
}
.btn {
  padding: 8px 18px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
}
.btn-primary {
  background: var(--primary);
  color: white;
}
.btn-cancel {
  background: #f97316;
  color: white;
}
</style>

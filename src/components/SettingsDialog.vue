<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "../stores/settings";
import { useEnvStore } from "../stores/env";

const settingsStore = useSettingsStore();
const envStore = useEnvStore();
const saved = ref(false);
const bundletoolPath = ref("");

const emit = defineEmits<{
  (e: "close"): void;
}>();

onMounted(async () => {
  await settingsStore.load();
  bundletoolPath.value = settingsStore.bundletoolJarPath;
});

async function selectBundletool() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      multiple: false,
      filters: [
        { name: "JAR files", extensions: ["jar"] },
        { name: "All files", extensions: ["*"] },
      ],
      title: "选择 bundletool.jar",
    });
    if (selected && typeof selected === "string") {
      bundletoolPath.value = selected;
    }
  } catch (e) {
    console.error("选择文件失败:", e);
  }
}

async function handleRecheck() {
  await envStore.checkEnvironment();
}

async function handleSave() {
  settingsStore.bundletoolJarPath = bundletoolPath.value;
  const ok = await settingsStore.save();
  if (ok) {
    await envStore.checkEnvironment();
    saved.value = true;
    setTimeout(() => emit("close"), 600);
  }
}
</script>

<template>
  <div class="overlay">
    <div class="dialog">
      <h3>设置</h3>
      <div class="field">
        <label>bundletool.jar 路径</label>
        <div class="input-row">
          <input type="text" :value="bundletoolPath" readonly placeholder="未配置" class="text-input" />
          <button class="btn btn-secondary btn-sm" @click="selectBundletool">浏览...</button>
        </div>
        <p class="hint">
          下载地址: <a href="https://github.com/google/bundletool/releases" target="_blank" class="link">GitHub Releases</a>
        </p>
      </div>
      <div class="field">
        <button class="btn btn-secondary" @click="handleRecheck">重新检测环境</button>
        <span v-if="envStore.allOk" class="env-ok">● Java {{ envStore.javaVersion }} · bundletool 已就绪</span>
        <span v-else class="env-err">● 环境未就绪</span>
      </div>
      <div class="actions">
        <span v-if="saved" class="save-ok">保存成功</span>
        <button class="btn btn-cancel" @click="emit('close')">取消</button>
        <button class="btn btn-primary" @click="handleSave">保存</button>
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
  max-width: 460px;
  width: 90%;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}
.dialog h3 {
  font-size: 16px;
  margin-bottom: 18px;
}
.field {
  margin-bottom: 14px;
}
.field label {
  display: block;
  font-size: 12px;
  color: var(--text-dim);
  margin-bottom: 4px;
}
.input-row {
  display: flex;
  gap: 8px;
}
.text-input {
  flex: 1;
  padding: 8px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 13px;
}
.text-input:focus {
  outline: none;
  border-color: var(--primary);
}
.btn {
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
}
.btn-sm { padding: 6px 10px; }
.btn-primary {
  background: var(--primary);
  color: white;
}
.btn-secondary {
  background: var(--bg);
  color: var(--text);
  border: 1px solid var(--border);
}
.hint {
  font-size: 11px;
  color: var(--text-dim);
  margin-top: 4px;
}
.link {
  color: var(--link);
  text-decoration: none;
}
.env-ok {
  margin-left: 10px;
  font-size: 11px;
  color: var(--success);
}
.env-err {
  margin-left: 10px;
  font-size: 11px;
  color: var(--error);
}
.actions {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 8px;
  margin-top: 16px;
}
.save-ok {
  font-size: 12px;
  color: var(--success);
  margin-right: auto;
}
</style>
